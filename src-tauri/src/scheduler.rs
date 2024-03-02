use crate::println;
use crate::sound_coordinator::{SCMessage, SoundCoordinator};
use crate::ttelement;

use chrono::DateTime;
use chrono::Local;
use chrono::TimeDelta;
use chrono::Timelike;

pub enum SMessage {
    Overwrite(ttelement::TTElement),
    // Not impremented
    // this element is reserved for adding/deleting schedule.
    #[allow(unused)]
    Delete(ttelement::TTElement),
}

pub struct Scheduler {}
const POLLING_OFFSET_SECS: u64 = 1;
const EFFECT_SECS: u64 = 3;
const DETECTION_ERROR_SECS: u64 = 5;

impl Scheduler {
    pub fn activate(
        tx_sc: std::sync::mpsc::SyncSender<SCMessage>,
    ) -> std::sync::mpsc::SyncSender<SMessage> {
        use std::sync::mpsc;
        let (tx, rx) = mpsc::sync_channel::<SMessage>(1);
        let mut time_table: Vec<ttelement::TTElement> = Vec::new();
        let mut next_play: Option<ttelement::TTElement> = None;
        let mut prev_play: Option<ttelement::TTElement> = None;

        std::thread::spawn(move || {
            loop {
                let mut table_changed: bool = false;

                // Recv table change event
                let sleep_secs = POLLING_OFFSET_SECS;
                let rv = rx.recv_timeout(std::time::Duration::from_secs(sleep_secs));
                if let Ok(msg) = rv {
                    println!("recved OK");
                    table_changed = Self::process_message(&mut time_table, msg);
                }

                let now: DateTime<Local> = Local::now();
                println!("Scheduler: {}:", now);
                println!("[next:{:?}]", next_play);

                // Update target to play
                if next_play.is_none() || table_changed {
                    next_play = Self::get_next_play(&prev_play, &time_table);
                }

                // Play sound if the time is come
                if next_play.is_some()
                    && Self::should_play_now(
                        &Self::to_time_delta_from_u32(next_play.as_ref().unwrap().time),
                        &Self::to_time_delta_from_date_time(&now),
                        next_play.as_ref().unwrap().effect,
                    )
                {
                    let active = next_play.as_ref().unwrap().active;
                    if active {
                        let voice = next_play.as_ref().unwrap().voice.clone();
                        let effect = next_play.as_ref().unwrap().effect;
                        let index = next_play.as_ref().unwrap().time / 100;
                        let volume = next_play.as_ref().unwrap().volume;
                        println!("playing index: {:?}, {:?}", voice, index);
                        SoundCoordinator::play_index(
                            &tx_sc,
                            voice,
                            index.try_into().unwrap(),
                            effect,
                            volume,
                        );
                    }

                    println!("NextPlay is set none");
                    prev_play = next_play;
                    next_play = None;
                }

                println!();
            }
        });

        tx
    }

    pub fn edit(tx_s: &std::sync::mpsc::SyncSender<SMessage>, row: ttelement::TTElement) {
        tx_s.send(SMessage::Overwrite(row)).unwrap();
    }

    fn process_message(time_table: &mut Vec<ttelement::TTElement>, message: SMessage) -> bool {
        match message {
            SMessage::Overwrite(src) => {
                if let Some(row) = time_table.iter_mut().find(|e| e.time == src.time) {
                    println!("Overwrite record {:?}", src);
                    row.active = src.active;
                    row.effect = src.effect;
                    row.voice = src.voice;
                    row.volume = src.volume;
                } else {
                    println!("New record {:?}", src);
                    time_table.push(src);
                }
            }
            SMessage::Delete(target) => {
                println!("Delete record {:?}", target);
                time_table.retain(|e| e.time == target.time);
            }
        };

        time_table.sort_by(|a, b| (a.time).partial_cmp(&b.time).unwrap());
        println!("{:?}", time_table);

        true
    }

    fn should_play_now(target: &TimeDelta, now: &TimeDelta, effect: bool) -> bool {
        let offset = if effect {
            EFFECT_SECS.try_into().unwrap()
        } else {
            0
        };

        let offsetted_target = Self::advance_counterclockwise(target, offset);

        let clockwise_secs = Self::clockwise_sub_seconds(&offsetted_target, now);
        let counterclockwise_secs = Self::counterclockwise_sub_seconds(&offsetted_target, now);
        let past = clockwise_secs >= counterclockwise_secs;

        if past && counterclockwise_secs < DETECTION_ERROR_SECS {
            return true;
        }

        false
    }

    fn to_time_delta_from_u32(time: u32) -> TimeDelta {
        let h = chrono::Duration::hours((time / 100).into());
        let m = chrono::Duration::minutes((time % 100).into());

        h + m
    }

    fn to_time_delta_from_date_time(time: &DateTime<Local>) -> TimeDelta {
        let h = chrono::Duration::hours(time.hour().into());
        let m = chrono::Duration::minutes(time.minute().into());
        let s = chrono::Duration::seconds(time.second().into());

        h + m + s
    }

    fn advance_counterclockwise(time: &TimeDelta, secs: i64) -> TimeDelta {
        let mut sub = *time - chrono::Duration::seconds(secs);
        if sub.num_seconds() < 0 {
            sub = sub + chrono::Duration::days(1);
        }

        sub
    }

    fn advance_clockwise(time: &TimeDelta, secs: i64) -> TimeDelta {
        let mut added = *time + chrono::Duration::seconds(secs);
        if added.num_days() >= 1 {
            added = added - chrono::Duration::days(1);
        }

        added
    }

    fn clockwise_sub_seconds(target: &TimeDelta, now: &TimeDelta) -> u64 {
        let adjusted_target: TimeDelta = if target < now {
            *target + chrono::Duration::days(1).into()
        } else {
            *target
        };

        let sub = adjusted_target - *now;
        sub.num_seconds().try_into().unwrap()
    }

    fn counterclockwise_sub_seconds(target: &TimeDelta, now: &TimeDelta) -> u64 {
        let adjusted_now: TimeDelta = if now < target {
            *now + chrono::Duration::days(1).into()
        } else {
            *now
        };

        let sub = adjusted_now - *target;
        sub.num_seconds().try_into().unwrap()
    }

    fn get_next_play(
        prev_play: &Option<ttelement::TTElement>,
        time_table: &Vec<ttelement::TTElement>,
    ) -> Option<ttelement::TTElement> {
        if time_table.is_empty() {
            return None;
        }

        let base = if prev_play.is_some() {
            Self::to_time_delta_from_u32(prev_play.as_ref().unwrap().time)
        } else {
            let now = Local::now();
            Self::to_time_delta_from_date_time(&now)
        };

        // Find the next minute for the given element
        // For example, if prev_play is 23:00, then find the nearest element clockwise from 23:01
        let target = Self::advance_clockwise(&base, 60);
        Some(
            time_table
                .iter()
                .min_by_key(|x| {
                    Self::clockwise_sub_seconds(&Self::to_time_delta_from_u32(x.time), &target)
                })
                .unwrap()
                .clone(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCondition {
        target: TimeDelta,
        now: TimeDelta,
        effect: bool,
        expect: bool,
    }

    impl std::fmt::Display for TestCondition {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(
                f,
                "({}:{}:{}, {}:{}:{}, {:?}, {:?})",
                self.target.num_seconds() / 3600,
                self.target.num_seconds() % 3600 / 60,
                self.target.num_seconds() % 3600 % 60,
                self.now.num_seconds() / 3600,
                self.now.num_seconds() % 3600 / 60,
                self.now.num_seconds() % 3600 % 60,
                self.effect,
                self.expect
            )
        }
    }

    fn hms(hours: i64, mins: i64, secs: i64) -> TimeDelta {
        TimeDelta::hours(hours) + TimeDelta::minutes(mins) + TimeDelta::seconds(secs)
    }

    #[test]
    fn it_works() {
        #[rustfmt::skip]
        let test_conditions: Vec<TestCondition> = vec![
            TestCondition { target: hms( 0,  0,  0), now: hms(23, 59, 59), effect: false, expect: false },
            TestCondition { target: hms( 0,  0,  0), now: hms( 0,  0,  0), effect: false, expect: true },
            TestCondition { target: hms( 0,  0,  0), now: hms( 0,  0,  1), effect: false, expect: true },
            TestCondition { target: hms( 0,  0,  0), now: hms( 0,  0,  2), effect: false, expect: true },
            TestCondition { target: hms( 0,  0,  0), now: hms( 0,  0,  3), effect: false, expect: true },
            TestCondition { target: hms( 0,  0,  0), now: hms( 0,  0,  4), effect: false, expect: true },
            TestCondition { target: hms( 0,  0,  0), now: hms( 0,  0,  5), effect: false, expect: false },
            TestCondition { target: hms( 1,  0,  0), now: hms( 0, 59, 59), effect: false, expect: false },
            TestCondition { target: hms( 1,  0,  0), now: hms( 1,  0,  4), effect: false, expect: true },
            TestCondition { target: hms( 1,  0,  0), now: hms( 1,  0,  5), effect: false, expect: false },
            TestCondition { target: hms(23, 59, 59), now: hms(23, 59, 58), effect: false, expect: false },
            TestCondition { target: hms(23, 59, 59), now: hms(23, 59, 59), effect: false, expect: true },
            TestCondition { target: hms(23, 59, 59), now: hms( 0,  0,  3), effect: false, expect: true },
            TestCondition { target: hms(23, 59, 59), now: hms( 0,  0,  4), effect: false, expect: false },
            TestCondition { target: hms( 0,  0,  0), now: hms(23, 59, 56), effect: true,  expect: false },
            TestCondition { target: hms( 0,  0,  0), now: hms(23, 59, 57), effect: true,  expect: true },
            TestCondition { target: hms( 0,  0,  0), now: hms(23, 59, 58), effect: true,  expect: true },
            TestCondition { target: hms( 0,  0,  0), now: hms(23, 59, 59), effect: true,  expect: true },
            TestCondition { target: hms( 0,  0,  0), now: hms( 0,  0,  0), effect: true,  expect: true },
            TestCondition { target: hms( 0,  0,  0), now: hms( 0,  0,  1), effect: true,  expect: true },
            TestCondition { target: hms( 0,  0,  0), now: hms( 0,  0,  2), effect: true,  expect: false },
            TestCondition { target: hms( 1,  0,  0), now: hms( 0, 59, 56), effect: true,  expect: false },
            TestCondition { target: hms( 1,  0,  0), now: hms( 0, 59, 57), effect: true,  expect: true },
            TestCondition { target: hms( 1,  0,  0), now: hms( 1,  0,  1), effect: true,  expect: true },
            TestCondition { target: hms( 1,  0,  0), now: hms( 1,  0,  2), effect: true,  expect: false },
            TestCondition { target: hms(23, 59, 59), now: hms(23, 59, 55), effect: true,  expect: false },
            TestCondition { target: hms(23, 59, 59), now: hms(23, 59, 56), effect: true,  expect: true },
            TestCondition { target: hms(23, 59, 59), now: hms( 0,  0,  0), effect: true,  expect: true },
            TestCondition { target: hms(23, 59, 59), now: hms( 0,  0,  1), effect: true,  expect: false },
        ];

        for test_condition in test_conditions.iter() {
            let actual = Scheduler::should_play_now(
                &test_condition.target,
                &test_condition.now,
                test_condition.effect,
            );
            assert_eq!(test_condition.expect, actual, "{}", test_condition);
        }
    }
}

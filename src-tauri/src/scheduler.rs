use crate::sound_coordinator::{SoundCoordinator, SCMessage};
use crate::ttelement;
use crate::println;

use chrono::Timelike;
use chrono::DateTime;
use chrono::Local;

pub enum SMessage {
    Overwrite(ttelement::TTElement),
    // Not impremented
    // this element is reserved for adding/deleting schedule.
    #[allow(unused)]
    Delete(ttelement::TTElement),
}

pub struct Scheduler {}
const MAX_FPS: f32 = 1.0;
impl Scheduler {
    pub fn activate(
        tx_sc: std::sync::mpsc::SyncSender<SCMessage>,
    ) -> std::sync::mpsc::SyncSender<SMessage> {
        use std::sync::mpsc;
        let (tx, rx) = mpsc::sync_channel::<SMessage>(1);
        let mut time_table: Vec<ttelement::TTElement> = Vec::new();
        let mut next_play: Option<ttelement::TTElement> = None;

        std::thread::spawn(move || {
            let sleep_milliseconds = (1000. / MAX_FPS) as u32;

            loop {
                let mut table_changed: bool = false;
                let now: DateTime<Local> = Local::now();

                println!("Scheduler: {}:", now);
                println!("[next:{:?}]", next_play);

                // Recv table change event
                let rv = rx.recv_timeout(std::time::Duration::from_millis(sleep_milliseconds as u64));
                if let Ok(msg) = rv {
                    println!("recved OK");
                    table_changed = Self::process_message(&mut time_table, msg);
                    next_play = None;
                } 

                // Update target to play
                if next_play.is_none() || table_changed {
                    next_play = Self::get_next_play(&time_table, &now);
                }

                // Play sound if the time is come
                if !next_play.is_none()
                    && Self::sub_minute(next_play.unwrap().time, &now) == 0 {

                    if next_play.unwrap().active {
                        let index = next_play.unwrap().time / 100;
                        println!("playing index: {:?}", index);
                        SoundCoordinator::play_full_set_list(&tx_sc, index, 100);
                    }

                    println!("NextPlay is set none");
                    next_play = None;
                }

                println!("");
            }
        });

        tx
    }

    pub fn edit(tx_s: &std::sync::mpsc::SyncSender<SMessage>, row: &ttelement::TTElement) {
        tx_s.send(SMessage::Overwrite(*row)).unwrap();
    }

    fn process_message(time_table: &mut Vec<ttelement::TTElement>, message: SMessage) -> bool {
        match message {
            SMessage::Overwrite(src) => {
                if let Some(row) = time_table.iter_mut().find(|e| e.time == src.time) {
                    println!("Overwrite record {:?}", src);
                    row.active = src.active;
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

        return true;
    }

    fn sub_minute(time: u32, now: &DateTime<Local>) -> i64 {
        let h = chrono::Duration::hours((time / 100).into());
        let m = chrono::Duration::minutes((time % 100).into());
        let now_h = chrono::Duration::hours(now.hour().into());
        let now_m = chrono::Duration::minutes(now.minute().into());

        let sub = (h + m) - (now_h + now_m);
        return sub.num_minutes();
    }

    fn get_next_play(time_table: &Vec<ttelement::TTElement>, now: &DateTime<Local>) -> Option<ttelement::TTElement> {
        if time_table.is_empty() { return None; }

        let target = *now + chrono::Duration::minutes(1);
        return Some(
            *time_table
            .iter()
            .min_by_key(|x| {
                ttelement::TTElement::sub(x.time, target.hour() * 100 + target.minute())
            })
            .unwrap(),
            );
    }
}

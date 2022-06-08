use crate::message::{SCMessage, SMessage};
use crate::sound_coordinator::SoundCoordinator;
use crate::ttelement;
use chrono::Timelike;

pub struct Scheduler {}

use chrono::DateTime;
use chrono::Local;

const MAX_FPS: f32 = 1.0;
impl Scheduler {
    pub fn activate(
        tx_sc: std::sync::mpsc::Sender<SCMessage>,
    ) -> std::sync::mpsc::Sender<SMessage> {
        use std::sync::mpsc;
        let (tx, rx) = mpsc::channel::<SMessage>();
        let mut time_table: Vec<ttelement::TTElement> = Vec::new();
        let mut next_play: Option<ttelement::TTElement> = None;

        std::thread::spawn(move || {
            let mut last_frame: DateTime<Local> = Local::now();

            loop {
                let now: DateTime<Local> = Local::now();

                // sleep fixed time and remove jitter.
                //
                let sleep_milliseconds = (1000. / MAX_FPS) as u32;
                //std::thread::sleep(std::time::Duration::new(0, sleep_milliseconds * 1000000));
                //let sleep_milliseconds = ((1000.0 / MAX_FPS) as i64) - (last_frame - now).num_milliseconds();

                print!("{}:", now);
                print!("{}:", last_frame);
                print!("[{}]", sleep_milliseconds);
                print!("[next:{:?}]", next_play);

                if sleep_milliseconds > 0 {
                    let rv = rx
                        .recv_timeout(std::time::Duration::from_millis(sleep_milliseconds as u64));
                    if let Ok(msg) = rv {
                        Self::process_message(&mut time_table, msg);
                        next_play = None;

                        println!("message received");
                    } else if let Err(e) = rv {
                        println!("sleep but no message {:?}", e);
                    }
                } else {
                    println!("Note: Slow down");
                    let rv = rx.try_recv();
                    if let Ok(msg) = rv {
                        Self::process_message(&mut time_table, msg);
                        next_play = None;
                    }
                };

                if next_play.is_none() {
                    let target = now + chrono::Duration::minutes(1);
                    next_play = Some(
                        *time_table
                            .iter()
                            .min_by_key(|x| {
                                ttelement::TTElement::sub(x.h, x.m, target.hour(), target.minute())
                            })
                            .unwrap(),
                    );
                } else {
                    let index = next_play.unwrap().h;
                    let h = chrono::Duration::hours(next_play.unwrap().h.into());
                    let m = chrono::Duration::minutes(next_play.unwrap().m.into());
                    let now_h = chrono::Duration::hours(now.hour().into());
                    let now_m = chrono::Duration::minutes(now.minute().into());

                    let sub = (h + m) - (now_h + now_m);

                    if sub.num_minutes() == 0 {
                        SoundCoordinator::play_full_set_list(&tx_sc, index, 100);
                        next_play = None;
                    }
                }

                last_frame = now;
            }
        });

        tx
    }

    pub fn edit(tx_s: &std::sync::mpsc::Sender<SMessage>, row: &ttelement::TTElement) {
        tx_s.send(SMessage::Overwrite(*row)).unwrap();
    }

    fn process_message(timetable: &mut Vec<ttelement::TTElement>, message: SMessage) {
        match message {
            SMessage::Overwrite(src) => {
                println!("{:?}", src);
                if let Some(row) = timetable.iter_mut().find(|e| e.h == src.h && e.m == src.m) {
                    row.active = src.active;
                } else {
                    timetable.push(src);
                }
            }
            SMessage::Delete(target) => {
                println!("{:?}", target);
                timetable.retain(|e| e.h == target.h && e.m == target.m);
            }
        };

        timetable.sort_by(|a, b| (a.time()).partial_cmp(&b.time()).unwrap());

        println!("{:?}", timetable);
    }
}

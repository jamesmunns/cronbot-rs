use cron;
use chrono;
use std::sync::mpsc::{TryRecvError, Receiver};
use bot::BotCommand;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::vec::Vec;

use std::thread::sleep;
use std::time::Duration;

// #[derive(Debug, Clone)]
pub struct SchedulerEvent {
    last_run: chrono::DateTime<chrono::UTC>,
    schedule: cron::Schedule,
    channel: String,
    message: String,
}

impl SchedulerEvent {
    pub fn new(schedule: cron::Schedule, channel: String, message: String) -> Self {
        SchedulerEvent {
            last_run: chrono::UTC::now(),
            schedule: schedule,
            channel: channel,
            message: message,
        }
    }
}

pub struct Scheduler {
    crontab: Vec<SchedulerEvent>,
    rx_cmd: Receiver<BotCommand>,
}

impl Scheduler {
    pub fn new(rx_cmd: Receiver<BotCommand>) -> Self {
        Scheduler {
            crontab: vec!(),
            rx_cmd: rx_cmd,
        }
    }

    pub fn handle_msgs(&mut self) {
        // THIS WONT WORK AFTER WE ADD SOMETHING OTHER THAN NEW
        self.crontab
            .append(&mut self.rx_cmd
                .try_iter()
                .filter_map(|m| message_handler(m))
                .collect::<Vec<SchedulerEvent>>());
    }

    pub fn handle_schedule(&mut self) {
        let now = chrono::UTC::now();
        // let msgs = self.crontab.iter().filter_map(|mut e| {
        //     if e.schedule.after(e.last_run).take(1)
        // })

        // Loop through events for each:
        for e in self.crontab.iter_mut() {
            // get the next run
            if let Some(next) = e.schedule.after(&e.last_run).next() {
                // if before now
                if next < now {
                    e.last_run = now;
                    println!("{}: {}", e.channel, e.message);
                    // print a message
                }
            }
        }
    }
}

pub fn forever(rx: Receiver<BotCommand>) {
    let delay = Duration::from_millis(500);
    let mut clock = Scheduler::new(rx);

    loop {
        sleep(delay);
        clock.handle_msgs();
        sleep(delay);
        clock.handle_schedule();
    }
}

// This is probably the wrong approach
pub fn message_handler(cmd: BotCommand) -> Option<SchedulerEvent> {
    use bot::BotCommand::*;
    match cmd {
        New(bc) => {
            if let Ok(sched) = cron::Schedule::from_str(&bc.cron) {
                return Some(SchedulerEvent::new(
                    sched,
                    bc.channel,
                    bc.message,
                ));
                // Do something with user?
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use bot;

    #[test]
    fn msg_handler_good_cron() {
        assert!(message_handler(BotCommand::New(bot::CronMessage {
            cron: "0/5 * * * * * *".to_string(),
            message: "<@U3UKBQXB3> hello".to_string(),
            channel: "C5ZTE1W5V".to_string(),
            user: "U3UKBQXB3".to_string(),
        })).is_some());
    }

    #[test]
    fn msg_handler_bad_cron() {
        assert!(message_handler(BotCommand::New(bot::CronMessage {
            cron: "asdf".to_string(),
            message: "<@U3UKBQXB3> hello".to_string(),
            channel: "C5ZTE1W5V".to_string(),
            user: "U3UKBQXB3".to_string(),
        })).is_none());
    }
}
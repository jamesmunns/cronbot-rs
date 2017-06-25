#![feature(box_patterns)]

extern crate cron;
extern crate chrono;
extern crate slack;
extern crate regex;

#[macro_use]
extern crate try_opt;

use std::str::FromStr;
use std::sync::mpsc::channel;
use std::thread;

mod scheduler;
mod bot;
use bot::BotCommand;

fn main() {
    let (msg_tx, msg_rx) = channel::<BotCommand>();


    let demo = "0/5 * * * * * *";
    let sched = cron::Schedule::from_str(demo).unwrap();
    let _: Vec<_> = sched.upcoming(chrono::UTC).take(1).map(|x| println!("{}", x)).collect();

    thread::spawn(move || {
        bot::botmain(msg_tx);
    });

    scheduler::forever(msg_rx);
}

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
use std::env;

mod scheduler;
mod bot;
use bot::{BotCommand, MyHandler};
use slack::{Event, RtmClient, Message};

fn main() {
    let (msg_tx, msg_rx) = channel::<BotCommand>();


    let demo = "0/5 * * * * * *";
    let sched = cron::Schedule::from_str(demo).unwrap();
    let _: Vec<_> = sched.upcoming(chrono::UTC).take(1).map(|x| println!("{}", x)).collect();



    let args: Vec<String> = env::args().collect();
    let api_key = match args.len() {
        0 | 1 => panic!("No api-key in args! Usage: cargo run --example slack_example -- <api-key>"),
        x => args[x - 1].clone(),
    };
    let mut handler = MyHandler{ msg_tx: msg_tx };
    let r = RtmClient::login(&api_key).unwrap();
    let response = r.sender().clone();

    thread::spawn(move || {
        let r = r.run(&mut handler);

        match r {
            Ok(_) => {}
            Err(err) => panic!("Error: {}", err),
        }
    });



    scheduler::forever(msg_rx, response);
}

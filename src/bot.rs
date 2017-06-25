use slack::{self, Event, RtmClient, Message};
use slack::api::MessageStandard;
use std::sync::mpsc::Sender;
use regex::Regex;

pub struct MyHandler {
    pub msg_tx: Sender<BotCommand>,
}

#[allow(unused_variables)]
impl slack::EventHandler for MyHandler {
    fn on_event(&mut self, cli: &RtmClient, event: Event) {
        println!("on_event(event: {:?})", event);
        match event {
            Event::Message(box Message::Standard(msg)) => {
                if let Some(cmd) = parse(msg.clone()) {
                    self.msg_tx.send(cmd).unwrap();
                }
            },
            _ => {},
        }
    }

    fn on_close(&mut self, cli: &RtmClient) {
        println!("on_close");
    }

    fn on_connect(&mut self, cli: &RtmClient) {
        println!("on_connect");
        // find the general channel id from the `StartResponse`
        // let general_channel_id = cli.start_response()
        //     .channels
        //     .as_ref()
        //     .and_then(|channels| {
        //                   channels
        //                       .iter()
        //                       .find(|chan| match chan.name {
        //                                 None => false,
        //                                 Some(ref name) => name == "bot-playground",
        //                             })
        //               })
        //     .and_then(|chan| chan.id.as_ref())
        //     .expect("general channel not found");
        // let _ = cli.sender().send_message(&general_channel_id, "Hello world! (rtm)");
        // Send a message over the real time api websocket
    }
}

#[derive(Debug, Clone)]
pub struct CronMessage {
    pub cron: String,
    pub message: String,
    pub channel: String,
    pub user: String,
}

#[derive(Debug, Clone)]
pub enum BotCommand {
    New(CronMessage),
}

fn parse(msg: MessageStandard) -> Option<BotCommand> {
    // TODO: Lazy Static
    // TODO: use a real parser here
    let re = Regex::new(r"^(new) `(.*?)` (.*)$").unwrap();

    let text = try_opt!(msg.text);
    let caps = try_opt!(re.captures(&text));
    let channel = try_opt!(msg.channel);
    let user = try_opt!(msg.user);
    let cron = try_opt!(caps.get(2));
    let message = try_opt!(caps.get(3));

    Some(BotCommand::New(CronMessage {
        cron: cron.as_str().to_string(),
        message: message.as_str().to_string(),
        channel: channel,
        user: user,
    }))
}
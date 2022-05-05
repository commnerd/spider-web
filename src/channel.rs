use std::rc::Rc;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;

pub struct Message { }

type ChannelPrim = (Sender<Message>, Receiver<Message>);

#[derive(Clone)]
pub struct Channel {
    channel: Rc<ChannelPrim>
}

impl Channel {
    pub fn new() -> Self {
        Channel{
            channel: Rc::new(mpsc::channel())
        }
    }

    pub fn listen(&self, target_channel: &Self) {
        loop {
            let msg = self.receive();
            let target = Rc::try_unwrap(target_channel.channel.clone()).ok().unwrap().0.clone();
            thread::spawn(move || {
                match target.clone().send(msg) {
                    Ok(rs) => println!("{:?}", rs),
                    Err(err) => println!("Error: {:?}", err),
                }
            });
        }
    }

    fn receive(&self) -> Message {
        let rcv = &self.channel.1;
        rcv.recv().ok().unwrap()
    }
    
    fn send(&self, msg: Message) {
        &self.channel.0.send(msg);
    }
}
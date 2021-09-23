use crossbeam_channel::{Receiver, TryIter};
use std::{io, thread};

pub struct StdinReceiver {
    receiver: Receiver<String>,
}

impl Default for StdinReceiver {
    fn default() -> Self {
        let (sender, receiver) = crossbeam_channel::unbounded();

        // read stdin to send as messages
        thread::spawn(move || {
            loop {
                let mut input = String::new();
                if let Err(e) = io::stdin().read_line(&mut input) {
                    eprintln!("failed to read line from stdin: {}", e);
                }
                // remove newline from input
                if input.ends_with('\n') {
                    input.pop();
                    if input.ends_with('\r') {
                        input.pop();
                    }
                }
                if let Err(e) = sender.send(input) {
                    eprintln!("failed to send stdin over channel: {}", e);
                };
            }
        });

        return Self { receiver };
    }
}

impl StdinReceiver {
    pub fn try_iter(&self) -> TryIter<'_, String> {
        self.receiver.try_iter()
    }
}

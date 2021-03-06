use std::comm::{Sender, Receiver};
use std::io::File;
use std::collections::HashMap;

enum Msg {

}

#[deriving(Clone)]
pub struct ClientTracker {
    client: String,
    total_traffic: u64,
    sender: Sender<Msg>
}

pub struct ClientTrackers {
    trackers: HashMap<String, ClientTracker>
}

impl ClientTrackers {
    pub fn new() -> ClientTrackers{
        ClientTrackers {
            trackers: HashMap::new()
        }
    }

    pub fn get(&mut self, client: String) -> ClientTracker {
        let trackers = &self.trackers.clone();
        let client_key = client.clone();
        let found_tracker = trackers.find(&client.clone());
        match found_tracker {
            Some(c) => return (*c).clone(),
            _ => {
                let tracker = ClientTracker::new(client);
                &self.add_tracker(client_key, tracker.clone());
                tracker
            }
        }
    }

    pub fn add_tracker(&mut self, client: String, tracker: ClientTracker) {
        self.trackers.insert(client, tracker);
    }
}

impl ClientTracker {
    pub fn new(client: String) -> ClientTracker {
        let (tx, rx) = channel();
        ClientTracker {
            client: client,
            total_traffic: 0,
            sender: tx
        }
    }

    pub fn increment(&self) {
        println!("Increment!");
    }
}

#[test]
fn can_initialize() {
    let tracker = ClientTracker::new("Hello".to_string());
}

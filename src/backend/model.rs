use std::sync::{Arc, Mutex};

use super::tcp::{start_listener, receiving_stream};

#[derive(Debug)]
pub struct Model {
    pub val: Mutex<String>,
    pub adjacent: Vec<Arc<Model>>,
}

impl Model {
    fn new(string: String, adjacent: Model) -> Self {
        let string = Mutex::new(string);
        let adjacent = Vec::new();

        Model { val: string, adjacent }
    }
}

fn add_string_rw(node: &Model) {
    let mut curr_val = node.val.lock().map_err(|err| -> Box<dyn std::error::Error> {
        eprintln!("[ERROR] COULD NOT GET THE NODE'S VAL: {}", err);
        err.into()
    }).unwrap();
    curr_val.push('!');
    for adj in node.adjacent.iter() {
        add_string_rw(&adj);
    }
}

pub fn entry() {
    let listener = start_listener().unwrap();
    receiving_stream(listener);
    // let (sender, receiver) = mpsc::sync_channel::<Node>(1);
    //
    // Send the data from other threads
    // And we can also process the data through RwLock
    // thread::spawn(|| {
    //
    // })
    // println!("Hello, world!");
}

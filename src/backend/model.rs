use super::tcp::{start_listener, receiving_stream};

pub fn entry() {
    let listener = start_listener().unwrap();
    receiving_stream(listener);
}

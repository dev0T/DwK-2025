use chrono::Utc;
use std::thread::sleep;
use std::time::{Duration, Instant};
use uuid::Uuid;

fn main() {
    let id = Uuid::new_v4();
    display_string(&id);
}

fn display_string(s: &Uuid) {
    let interval = Duration::from_secs(5);
    let mut next_time = Instant::now() + interval;

    loop {
        let timestamp: chrono::DateTime<Utc> = Utc::now();
        println!("{timestamp:?}: {s}");
        sleep(next_time - Instant::now());
        next_time += interval;
    }
}

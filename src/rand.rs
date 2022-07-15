use std::time::{SystemTime, UNIX_EPOCH};

fn get_unix_ts() -> u64 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .unwrap();
    since_the_epoch.as_secs()
}

pub struct Rand {
    value: u64,
}

impl Rand {
    pub fn new() -> Rand {
        Rand { value: get_unix_ts() }
    }

    pub fn next(&mut self) -> u64 {
        const M: u64 = 7542721;
        const A: u64 = 5895503;
        const C: u64 = 3843739;

        self.value = ((A * self.value) + C) % M;
        self.value
    }
}
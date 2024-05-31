use std::time::{SystemTime, Duration};

pub type Tick = u8; //< Tick is a second
pub type Waker = fn(Arg) -> Tick;
pub type Arg = *mut ();

pub const NULLARG: Arg = 0 as Arg;

struct Element {
    pub time: SystemTime,
    pub waker: Waker,
    pub arg: Arg
}

impl Element {
    pub fn new(waker: Waker, arg: Arg, count: Tick) -> Self {
        Self {
            waker,
            arg,
            time: SystemTime::now() + Duration::from_secs(count as u64)
        }
    }
}

static mut TICK_QUEUE: Vec <Element> = Vec::new();

pub fn add(waker: Waker, arg: Arg, count: Tick) {
    unsafe { TICK_QUEUE.push(Element::new(waker, arg, count)) }
}

pub fn check() {
    let mut i = 0;
    let now = SystemTime::now();
    unsafe { while i < TICK_QUEUE.len() {
        if TICK_QUEUE[i].time <= now {
            let ret = (TICK_QUEUE[i].waker)(TICK_QUEUE[i].arg);
            if ret == 0 {
                TICK_QUEUE.remove(i);
                continue
            } else {
                TICK_QUEUE[i].time = now + Duration::from_secs(ret as u64);
            }
        }
        i += 1
    } }
}

use core::panic;
use std::{
    cell::UnsafeCell,
    mem::MaybeUninit,
    sync::{
        atomic::{
            AtomicBool,
            Ordering::{Acquire, Relaxed, Release},
        },
        Arc,
    },
};

struct Channel<T> {
    message: UnsafeCell<MaybeUninit<T>>,
    ready: AtomicBool,
}

pub struct Sender<T> {
    channel: Arc<Channel<T>>,
}

pub struct Receiver<T> {
    channel: Arc<Channel<T>>,
}

unsafe impl<T> Sync for Channel<T> where T: Send {}

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let a = Arc::new(Channel {
        message: UnsafeCell::new(MaybeUninit::uninit()),
        ready: AtomicBool::new(false),
    });
    (Sender { channel: a.clone() }, Receiver { channel: a })
}

impl<T> Sender<T> {
    pub fn send(self, message: T) {
        unsafe {
            (*self.channel.message.get()).write(message);
        }
        self.channel.ready.store(true, Release);
    }
}

impl<T> Receiver<T> {
    pub fn is_ready(&self) -> bool {
        self.channel.ready.load(Relaxed)
    }

    pub fn receive(self) -> T {
        if !self.channel.ready.swap(false, Acquire) {
            panic!("no message available!");
        }
        unsafe { (*self.channel.message.get()).assume_init_read() }
    }
}

impl<T> Drop for Channel<T> {
    fn drop(&mut self) {
        if *self.ready.get_mut() {
            unsafe { self.message.get_mut().assume_init_drop() }
        }
    }
}
// pub struct Channel<T> {
//     message: UnsafeCell<MaybeUninit<T>>,
//     in_use: AtomicBool,
//     ready: AtomicBool,
// }

// unsafe impl<T> Sync for Channel<T> where T: Send {}

// impl<T> Drop for Channel<T> {
//     fn drop(&mut self) {
//         if *self.ready.get_mut() {
//             unsafe { self.message.get_mut().assume_init_drop() }
//         }
//     }
// }

// impl<T> Channel<T> {
//     pub fn new() -> Self {
//         Self {
//             message: UnsafeCell::new(MaybeUninit::uninit()),
//             in_use: AtomicBool::new(false),
//             ready: AtomicBool::new(false),
//         }
//     }

//     // panic when trying to send more than one message.
//     pub fn send(&self, message: T) {
//         if self.in_use.swap(true, Relaxed) {
//             panic!("can't send more than one message!");
//         }
//         unsafe {
//             (*self.message.get()).write(message);
//         }
//         self.ready.store(true, Release);
//     }

//     pub fn is_ready(&self) -> bool {
//         self.ready.load(Relaxed)
//     }

//     pub fn receive(&self) -> T {
//         if !self.ready.swap(false, Acquire) {
//             panic!("no message available");
//         }
//         // Safety: We've just checked (and reset) the ready flag.
//         unsafe { (*self.message.get()).assume_init_read() }
//     }
// }

use std::thread;

use spin_lock::SpinLock;

use crate::channel::Channel;

mod arc;
mod channel;
mod spin_lock;

fn main() {
    //SpinLockの動作確認
    //------------------------------------------------------------------------------ /
    let x = SpinLock::new(Vec::new());
    thread::scope(|s| {
        s.spawn(|| x.lock().push(1)); // Deref, DerefMutを実装していることで、Guardに対してVec::push()を直接呼び出すことができる。
        s.spawn(|| {
            let mut g = x.lock();
            g.push(2);
            g.push(2);
        });
    });
    let g = x.lock();
    assert!(g.as_slice() == [1, 2, 2] || g.as_slice() == [2, 2, 1]);
    // println!("{:?}", g.as_slice());
    //------------------------------------------------------------------------------ /

    //Channelの動作確認
    //------------------------------------------------------------------------------ /
    let mut channel = Channel::new();
    thread::scope(|s| {
        let (sender, receiver) = channel.split();
        s.spawn(move || {
            sender.send("hello world!");
        });
        assert_eq!(receiver.receive(), "hello world!");
    })
    //------------------------------------------------------------------------------ /
}

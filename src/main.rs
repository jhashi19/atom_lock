use std::thread;

use channel::Channel;
use spin_lock::SpinLock;

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
    let channel = Channel::new();
    let t = thread::current();
    thread::scope(|s| {
        s.spawn(|| {
            channel.send("hello world!");
            t.unpark();
        });
        while !channel.is_ready() {
            thread::park();
        }
        assert_eq!(channel.receive(), "hello world!")
    })

    //------------------------------------------------------------------------------ /
}

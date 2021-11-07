use crossbeam::{channel, scope};
use std::thread;

fn main() {
    let mut v = vec![0u32; 1000];

    {
        let (s, r) = channel::unbounded();

        for b in v.chunks_mut(2) {
            s.send(b).unwrap();
        }

        scope(|s| {
            for i in 0..500 {
                let r = r.clone();
                s.spawn(move |_| {
                    while let Ok(b) = r.try_recv() {
                        thread::sleep(std::time::Duration::from_secs(1));
                        b.fill(i);
                    }
                });
            }
        })
        .unwrap();
        drop(s);
    }

    for c in v.chunks(2) {
        assert_eq!(c[0], c[1]);
    }
}

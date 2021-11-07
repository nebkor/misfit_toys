use std::io::Write;
use std::{env, io};

use threadpool::ThreadPool;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Please supply a phrase to be badly printed.")
    }
    let string = args[1..].join(" ");
    let num_threads = string.len();

    let pool = ThreadPool::new(num_threads);
    for c in string.chars() {
        pool.execute(move || {
            print!("{}", c);
            let _ = io::stdout().flush();
        });
    }
    pool.join();
    println!();
}

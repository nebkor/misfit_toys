use clap::{App, Arg};
use rand::{distributions::StandardNormal, prelude::*};

const SEED: [u32; 4] = [0x193a_6754, 0xa8a7_d469, 0x9783_0e05, 0x113b_a7bb];

pub fn get_rng() -> SmallRng {
    let mut s: [u8; 16] = [0; 16];
    unsafe { s.copy_from_slice(SEED.align_to::<u8>().1) };
    SmallRng::from_seed(s)
}

fn gauss(n: usize, r: &mut SmallRng) -> f64 {
    let mut tot: f64 = 0.0;
    for _ in 0..n {
        tot += r.sample(StandardNormal);
    }
    tot
}

fn uniform(n: usize, r: &mut SmallRng) -> f64 {
    let mut tot: f64 = 0.0;
    for _ in 0..n {
        tot += r.gen::<f64>();
    }
    tot
}

fn main() {
    let mut r = get_rng();

    let f = gauss(4, &mut r);

    let matches = App::new("Random Value Benchmarker")
        .arg(
            Arg::with_name("TYPE")
                .required(true)
                .index(1)
                .takes_value(true)
                .possible_values(&["gauss", "uniform"]),
        )
        .arg(
            Arg::with_name("NUM_VALS")
                .required(true)
                .index(2)
                .takes_value(true),
        )
        .get_matches();

    let n: usize = matches.value_of("NUM_VALS").unwrap().parse().unwrap();

    let tot = match matches.value_of("TYPE").unwrap() {
        "gauss" => gauss(n, &mut r),
        "uniform" => uniform(n, &mut r),
        _ => panic!("who let you in here"),
    };

    println!("{}, {}", tot, tot / n as f64);
}

extern crate getopts;
extern crate rand;

use std::env;


use getopts::Options;
use rand::Rng;
use rand::StdRng;

pub struct Zipf {
    n: i64,             // Generated numbers range: [1, n]
    theta: f64,         // Skew factor.
    cdf_vec: Vec<f64>,  // Cumulative distribution values.
    rng: StdRng,        // Uniform random number generator.
}

impl Zipf {
    pub fn new(n: i64, theta: f64) -> Zipf {
        let mut zipf_dist = Zipf {
            n: n,
            theta: theta,
            rng: StdRng::new().unwrap(),
            cdf_vec: Vec::with_capacity(n as usize),
        };
        let mut sum: f64 = 0.0;
        for i in 0..n {
            let k = i + 1;
            sum += 1.0f64 / (k as f64).powf(theta);
            zipf_dist.cdf_vec.push(sum);
        }
        for i in 0..n {
            zipf_dist.cdf_vec[i as usize] /= sum;
        }
        return zipf_dist;
    }

    pub fn next_i64(&mut self) -> i64 {
        let rand = self.rng.next_f64();
        for i in 0..self.n {
            if rand < self.cdf_vec[i as usize] {
                return i + 1;
            }
        }
        return self.n;
    }

    pub fn fet_n(&self) -> i64 {
        return self.n;
    }

    pub fn get_theta(&self) -> f64 {
        return self.theta;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.reqopt("n", "range", "range of numbers created", "RANGE");
    opts.reqopt("t", "theta", "skew factor", "SKEW");
    opts.reqopt("s", "scale", "scale factor", "SCALE");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => { panic!(f.to_string()) }
    };

    let n: i64 = matches.opt_str("n").unwrap().parse().unwrap();
    let t: f64 = matches.opt_str("t").unwrap().parse().unwrap();
    let s: i64 = matches.opt_str("s").unwrap().parse().unwrap();

    let mut zipf = Zipf::new(n, t);
    for _ in 0..s {
        println!("{}", zipf.next_i64());
    }
}

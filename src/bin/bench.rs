extern crate starplayer;

use std::time::SystemTime;

use starplayer::StarAI;

const SIZE: usize = 5;
const LOOP_ITERATIONS: usize = 1;
const KOMI: isize = 1;
const TRIAL_TIME: u64 = 10;
const TRIALS: usize = 10;

fn print_statistics(iteration_records: Vec<usize>) {
    let mut mean = 0.0;
    for record in iteration_records.iter() {
        mean += *record as f64;
    }
    mean /= iteration_records.len() as f64;
    let mut stddev = 0.0;
    for record in iteration_records.iter() {
        stddev += (mean - *record as f64).powi(2);
    }
    stddev = (stddev / iteration_records.len() as f64).powf(0.5) / (TRIALS as f64).sqrt();
    println!("{} +- {}", mean, stddev);
}

fn main() {
    let mut iteration_records = Vec::new();
    for _ in 0..TRIALS {
        let mut ai = StarAI::new(SIZE);
        let start_time = SystemTime::now();
        let mut iterations = 0;
        loop {
            ai.calculate(LOOP_ITERATIONS, KOMI);
            iterations += LOOP_ITERATIONS;
            if start_time.elapsed().unwrap().as_secs() >= TRIAL_TIME {
                break;
            }
        }
        println!("Iterations: {}", iterations);
        iteration_records.push(iterations);
    }
    print_statistics(iteration_records);
}

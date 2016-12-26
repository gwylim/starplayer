extern crate starplayer;
extern crate time;

use std::io::{self, Write};
use std::str::FromStr;
use std::time::SystemTime;

use starplayer::StarAI;

// TODO: these type of constants shouldn't be duplicated here
const SIZE: usize = 7;
const MOVE_TIME: u64 = 1;
const KOMI: isize = 1;
const ITERATIONS: usize = 100;

fn try_input<F: FromStr>() -> Option<F> {
    let mut buffer = String::new();
    match io::stdin().read_line(&mut buffer) {
        Ok(_) => {
            // TODO: show error
            Some(buffer.trim().parse::<F>().unwrap_or_else(|_| {
                panic!("Failed to parse line: '{}'", buffer);
            }))
        },
        Err(_) => None,
    }
}

fn write_or_panic<F: ToString>(value: F) {
    io::stdout().write(value.to_string().as_bytes()).unwrap_or_else(|e| {
        panic!("Failed to write value: {}", e);
    });
    io::stdout().write(b"\n").unwrap();
    io::stdout().flush().unwrap();
}

fn make_move(ai: &mut StarAI) {
    let start_time = SystemTime::now();
    loop {
        ai.calculate(ITERATIONS, KOMI);
        if start_time.elapsed().unwrap().as_secs() > MOVE_TIME {
            break;
        }
    }
    let (x, y) = ai.best_move();
    ai.add_move(x, y);
    write_or_panic(x);
    write_or_panic(y);
}

fn main() {
    let mut ai = StarAI::new(SIZE);

    let is_first_player = try_input::<u8>().unwrap() == 0;

    if is_first_player {
        make_move(&mut ai);
    }

    loop {
        let x = try_input();
        let y = try_input();
        match (x, y) {
            (Some(x), Some(y)) => {
                ai.add_move(x, y);
                if ai.finished(KOMI) {
                    break;
                }
                make_move(&mut ai);
                if ai.finished(KOMI) {
                    break;
                }
            },
            _ => {
                break;
            }
        }
    }
}

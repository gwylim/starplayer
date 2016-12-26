extern crate starplayer;
extern crate time;

const SIZE: usize = 5;
const LOOP_ITERATIONS: usize = 64;
const KOMI: isize = 1;
const MOVE_TIME: u64 = 30;

use std::io;
use std::time::SystemTime;

use starplayer::StarAI;
use starplayer::player::PLAYERS;

fn make_move(ai: &mut StarAI) {
    let start_time = SystemTime::now();
    loop {
        ai.calculate(LOOP_ITERATIONS, KOMI);
        if start_time.elapsed().unwrap().as_secs() >= MOVE_TIME {
            break;
        }
    }
    let (x, y) = ai.best_move();
    ai.add_move(x, y);
}

fn read_line() -> io::Result<String> {
    let mut buffer = String::new();
    match io::stdin().read_line(&mut buffer) {
        Ok(_) => Ok(String::from(buffer.trim())),
        Err(e) => Err(e),
    }
}

fn print_scores(ai: &StarAI) {
    for player in PLAYERS.iter() {
        println!("{:?}: {}", player, ai.score(*player, KOMI));
    }
}

fn main() {
    let mut ai = StarAI::new(SIZE);

    let is_first_player = read_line().unwrap().parse::<u8>().unwrap() == 0;

    if is_first_player {
        make_move(&mut ai);
    }
    ai.print_board();

    loop {
        let line = read_line().unwrap();
        let coords = line.split(" ").collect::<Vec<&str>>();
        if coords.len() != 2 {
            println!("You must input two numbers");
            continue;
        }
        let x: usize = coords[0].parse().unwrap();
        let y: usize = coords[1].parse().unwrap();
        ai.add_move(x - 1, y - 1);
        ai.print_board();
        print_scores(&ai);

        make_move(&mut ai);
        ai.print_board();
        print_scores(&ai);
    }
}

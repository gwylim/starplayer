extern crate getopts;
extern crate starplayer;

use std::process::{Command, Stdio, ChildStdin, ChildStdout};
use std::env;
use std::path;
use std::io::{Write, BufReader, BufRead};
use std::str::FromStr;

use getopts::Options;

use starplayer::player::Player;
use starplayer::StarAI;

const SIZE: usize = 7;
const KOMI: isize = 1;

const DIR_A: &'static str = "test/a";
const DIR_B: &'static str = "test/b";

fn print_usage(program: &str) {
    println!("Usage: {} REV1 REV2", program);
}

fn checkout(dest: &str) {
    Command::new("rm")
        .arg("-rf")
        .arg(dest)
        .status()
        .unwrap_or_else(|e| {
            panic!("Failed to clear test directories: {}", e);
        });
    Command::new("git")
        .arg("clone")
        .arg(".")
        .arg(dest)
        .status()
        .unwrap_or_else(|e| {
            panic!("Failed to clone: {}", e);
        });
}

fn compile(dir: &str, rev: &str) {
    let prev_dir = env::current_dir().unwrap();
    env::set_current_dir(path::Path::new(dir)).unwrap();
    Command::new("git")
        .arg("checkout")
        .arg(rev)
        .stderr(Stdio::null())
        .status()
        .unwrap_or_else(|e| {
            panic!("Failed to checkout rev: {}", e);
        });
    Command::new("cargo")
        .arg("build")
        .arg("--bin")
        .arg("player")
        .arg("--release")
        .stderr(Stdio::null())
        .status()
        .unwrap_or_else(|e| {
            panic!("Failed to compile: {}", e);
        });
    env::set_current_dir(prev_dir).unwrap();
}

struct PlayerIO {
    stdin: ChildStdin,
    stdout: BufReader<ChildStdout>,
}

impl PlayerIO {
    fn prepare(dir: &str, rev: &str) {
        checkout(dir);
        compile(dir, rev);
    }

    fn new(dir: &str) -> PlayerIO {
        let child = Command::new(dir.to_string() + "/target/release/player")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap_or_else(|e| {
                panic!("Failed to spawn child: {}", e);
            });
        PlayerIO {
            stdin: child.stdin.unwrap(),
            stdout: BufReader::new(child.stdout.unwrap()),
        }
    }

    fn read<F: FromStr>(&mut self) -> F {
        let mut buffer = String::new();
        self.stdout.read_line(&mut buffer).unwrap();
        buffer.trim().parse::<F>().unwrap_or_else(|_| {
            panic!("Failed to parse read value");
        })
    }

    fn write<F: ToString>(&mut self, value: F) {
        self.stdin.write(value.to_string().as_bytes()).unwrap();
        self.stdin.write(b"\n").unwrap();
        self.stdin.flush().unwrap();
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let opts = Options::new();
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m },
        Err(f) => { panic!(f.to_string()) },
    };

    if matches.free.len() != 2 {
        print_usage(&program);
        return;
    }

    let mut a_wins = 0;
    let mut b_wins = 0;

    PlayerIO::prepare(DIR_A, &matches.free[0]);
    PlayerIO::prepare(DIR_B, &matches.free[1]);

    loop {
        let a_first_player = (a_wins + b_wins)%2 == 0;
        let mut ai = StarAI::new(SIZE);
        let mut pa = PlayerIO::new(DIR_A);
        let mut pb = PlayerIO::new(DIR_B);
        if a_first_player {
            pa.write(0);
            pb.write(1);
        } else {
            pb.write(0);
            pa.write(1);
        }
        while !ai.finished(KOMI) {
            // TODO: refactoring
            let (x, y) = if a_first_player == (ai.player_turn() == Player::First) {
                let x = pa.read();
                let y = pa.read();
                pb.write(x);
                pb.write(y);
                (x, y)
            } else {
                let x = pb.read();
                let y = pb.read();
                pa.write(x);
                pa.write(y);
                (x, y)
            };
            ai.add_move(x, y);
            ai.print_board();
            println!("Stats: {} - {}", a_wins, b_wins);
        }
        if a_first_player == (ai.winner(KOMI).unwrap() == Player::First) {
            a_wins += 1;
        } else {
            b_wins += 1;
        }
    }
}

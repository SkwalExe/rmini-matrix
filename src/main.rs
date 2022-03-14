#![allow(dead_code)]

use rand::Rng;
use std::process;

const RED: &str = "\x1b[91m";
const GREEN: &str = "\x1b[92m";
const YELLOW: &str = "\x1b[93m";
const BLUE: &str = "\x1b[94m";
const MAGENTA: &str = "\x1b[95m";
const CYAN: &str = "\x1b[96m";
const WHITE: &str = "\x1b[97m";
const RESET: &str = "\x1b[0m";
const BG_RED: &str = "\x1b[41m";
const BG_GREEN: &str = "\x1b[42m";
const BG_YELLOW: &str = "\x1b[43m";
const BG_BLUE: &str = "\x1b[44m";
const BG_MAGENTA: &str = "\x1b[45m";
const BG_CYAN: &str = "\x1b[46m";

fn matrix(n_chars: &usize, custom_chars: &String) {
    let mut line = String::new();

    for _ in 0..*n_chars {
        let random_index = rand::thread_rng().gen_range(0..custom_chars.len());
        line.push(custom_chars.chars().nth(random_index).unwrap());
    }

    println!("{}", line);
}

fn main() {
    let mut command = "matrix";
    let mut repeat = false;
    let mut n_chars = 10;
    let mut speed = 0.5;
    let mut custom_chars = String::from("01");

    let mut args: Vec<String> = std::env::args().collect();

    args.remove(0);

    while args.len() > 0 {
        match args[0].as_str() {
            "-h" | "--help" => {
                command = "help";
                args.remove(0);
            }
            "-v" | "--version" => {
                command = "version";
                args.remove(0);
            }
            "-s" | "--speed" => {
                if args.len() > 1 {
                    speed = match args[1].parse::<f32>() {
                        Ok(n) => n,
                        Err(_) => {
                            println!(
                                "{}[ x ] : Invalid argument after {} : {}{} {} {}",
                                RED, args[0], WHITE, BG_RED, args[1], RESET
                            );
                            process::exit(1);
                        }
                    };
                    args.remove(0);
                    args.remove(0);
                } else {
                    println!(
                        "{}[ x ] : Argument needed after : {}{} {} {}",
                        RED, WHITE, BG_RED, args[0], RESET
                    );
                    process::exit(1);
                }
            }
            "-l" | "--loop" => {
                repeat = true;
                args.remove(0);
            }
            "-a" | "--custom-chars" => {
                if args.len() > 1 {
                    custom_chars = args.remove(1);
                    args.remove(0);
                } else {
                    println!(
                        "{}[ x ] : Argument needed after : {}{} {} {}",
                        RED, WHITE, BG_RED, args[0], RESET
                    );
                    process::exit(1);
                }
            }
            "-c" | "--chars" => {
                if args.len() > 1 {
                    n_chars = match args[1].parse::<usize>() {
                        Ok(n) => n,
                        Err(_) => {
                            println!(
                                "{}[ x ] : Invalid argument after {} : {}{} {} {}",
                                RED, args[0], WHITE, BG_RED, args[1], RESET
                            );
                            process::exit(1);
                        }
                    };
                    args.remove(0);
                    args.remove(0);
                } else {
                    println!(
                        "{}[ x ] : Argument needed after : {}{} {} {}",
                        RED, WHITE, BG_RED, args[0], RESET
                    );
                    process::exit(1);
                }
            }

            _ => {
                println!(
                    "{}[ x ] : Invalid argument: {}{} {} {}",
                    RED, WHITE, BG_RED, args[0], RESET
                );
                process::exit(1);
            }
        }
    }

    match command {
        "matrix" => {
            if repeat {
                loop {
                    matrix(&n_chars, &custom_chars);

                    std::thread::sleep(std::time::Duration::from_millis((speed * 1000.0) as u64));
                }
            } else {
                matrix(&n_chars, &custom_chars);
            }
        }

        "version" => println!(
            "{}rmini-matrix, by Skwal => {}{}{}",
            MAGENTA,
            RED,
            env!("CARGO_PKG_VERSION"),
            RESET
        ),
        "help" => {
            println!("{}{} rmini-matrix {}", BG_MAGENTA, WHITE, RESET);
            println!("{}━━━━━━━━━━━━━━━━━{}", MAGENTA, RESET);
            println!("Author: {}SkwalExe{}", MAGENTA, RESET);
            println!("Github: {}https://github.com/SkwalExe{}", MAGENTA, RESET);
            println!("{}━━━━━━━━━━━━━━━━━{}", MAGENTA, RESET);
            println!(
                "{}Displays random 0 and 1 or custom chars in matrix style{}",
                MAGENTA, RESET
            );
            println!("{}━━━━━━━━━━━━━━━━━{}", MAGENTA, RESET);
            println!("Options : ");
            println!(
                "\t{}--version, -v: {}Prints the version of the program{}",
                MAGENTA, YELLOW, RESET
            );
            println!(
                "\t{}--help, -h: {}Prints this help message{}",
                MAGENTA, YELLOW, RESET
            );
            println!(
                "\t{}--speed, -s: {}Sets the delay in second between each line [D: 0.5]{}",
                MAGENTA, YELLOW, RESET
            );
            println!(
                "\t{}--loop, -l: {}Loops the matrix {}",
                MAGENTA, YELLOW, RESET
            );
            println!(
                "\t{}--chars, -c: {}Set the number of characters to output per line [D: 10]{}",
                MAGENTA, YELLOW, RESET
            );
            println!(
                "\t{}--custom-chars, -a: {}Set the characters to use [D: 01]{}",
                MAGENTA, YELLOW, RESET
            );
            println!("{}━━━━━━━━━━━━━━━━━{}", MAGENTA, RESET);
        }
        _ => {}
    }
}

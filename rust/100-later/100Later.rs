use std::io;
use std::time;

fn main() {
    let name = input("What is your name? ").expect("Something went wrong!");
    let age: u64 = input("How old are you? ")
        .expect("Something wrong!")
        .parse()
        .unwrap();
    let time = time::SystemTime::now();
    let secs_from_unix_epoch = time
        .duration_since(time::UNIX_EPOCH)
        .expect("Time went backwards");
    let year: u64 =(secs_from_unix_epoch.as_secs() / 60 / 60 / 24 / 365) + 1970;
    println!("Hello, {}", name);
    println!("You will turn 100 in the year: {}", (100 - age) + year);
}

fn input(message: &str) -> io::Result<String> {
    use std::io::Write;
    let mut buffer: String = String::new();
    print!("{}", message);
    io::stdout().flush()?;
    io::stdin().read_line(&mut buffer).unwrap();
    Ok(buffer.trim_end().to_string())
}

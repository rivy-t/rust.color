extern crate wincolor;

use wincolor::{Color, Console, Intense};

fn main() {
    let mut con = Console::stdout().unwrap();
    con.fg(Intense::Yes, Color::Cyan).unwrap();
    println!("This text will be intense cyan.");
    con.reset().unwrap();
    println!("This text will be normal.");
}

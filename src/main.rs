use std::env;
use std::fs::File;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Please provide a filename and display size");
        return;
    }

    println!("Filename is {}", args[1]);
    println!("Display size is {}", args[2]);

    let f = File::open(&args[1])?;
    OK(());
}

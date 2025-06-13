use std::{
    fs::{File, OpenOptions},
    io::{self, BufRead, BufReader, Write},
};

pub fn _basic_io() -> io::Result<()> {
    // Basic Output
    println!("Hello World");
    print!("No Newline");
    println!("Formatted: {}, {}", 42, "text");

    // Basic Input
    let mut input = String::new();
    print!("Enter name: ");
    io::stdout().flush()?; // Flush buffer to show prompt
    io::stdin().read_line(&mut input)?;
    let name = input.trim();
    println!("{}", name);

    // Reading numbers
    print!("Enter a number: ");
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let number: i32 = input.trim().parse().expect("Please enter valid number");
    println!("{}", number);

    // File writing
    let mut file = File::create("outpout.txt")?;
    writeln!(file, "Hello, File!")?;

    // Appending to a file
    let mut file = OpenOptions::new().append(true).open("output.txt")?;
    writeln!(file, "Appended text")?;

    // File Reading (all at once)
    let contents = std::fs::read_to_string("input.txt")?;
    println!("{}", contents);

    // Buffered file reading (line by line)
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        println!("{}", line?);
    }

    // Reading binary data
    let bytes = std::fs::read("input.txt")?;
    println!("raw bytes: {:?}", bytes);

    // Writing to stderr
    eprintln!("This is an error message");

    Ok(())
}

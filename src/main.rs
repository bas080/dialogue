use std::process::{Command, Stdio};
use std::io::{self, Write, BufReader, BufRead};

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <binary_name>", args[0]);
        std::process::exit(1);
    }
    
    let binary = &args[1];
    let mut child = Command::new(binary)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    
    let mut stdin = child.stdin.take().expect("Failed to open stdin");
    let stdout = child.stdout.take().expect("Failed to open stdout");
    let stderr = child.stderr.take().expect("Failed to open stderr");
    
    let stdout_reader = BufReader::new(stdout);

    // Create a thread to print the stdout
    std::thread::spawn(move || {
        for line in stdout_reader.lines() {
            match line {
                Ok(output) => println!("Output: {}", output),
                Err(e) => eprintln!("Error reading stdout: {}", e),
            }
        }
    });

    // Sending input to the binary interactively
    let mut input = String::new();
    while let Ok(bytes_read) = io::stdin().read_line(&mut input) {
        if bytes_read == 0 {
            break; // EOF
        }
        stdin.write_all(input.as_bytes())?;
        stdin.flush()?;
        input.clear();
    }
    
    // Wait for the child process to finish
    let _ = child.wait()?;
    Ok(())
}
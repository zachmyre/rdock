// CLI tool for building, tagging, and pushing docker containers.

// example) rdock ../.dockerfile
// where argument is only path, rdock finds the first build, tag, and push tags to use

// example) rdock -c sierra-api ../../.dockerfile
// where -c says find the comment where the next arg is found for each build, tag, and push. then last is the path to docker file.
// good for projects with multiple builds in same file

use std::process::Command;
use clap::Parser;

#[derive(Parser, Debug)]
struct Cli {
    path: std::path::PathBuf,
}

fn main() {
    // read file location
    let args = Cli::parse();
    // find dockerfile
    let content = std::fs::read_to_string(&args.path).expect("could not read file");
    let file_contents = content.lines();
    const COMMANDS: [&'static str; 3] = ["build ", "tag ", "push"];
    // show contents of dockerfile
    for line in file_contents {
        for substring in &COMMANDS {
            if line.contains(substring) {
                println!("Found substring: {}", substring);
                let parsed_line = &line[2..];
                let output = Command::new("docker")
        .arg(parsed_line)
        .output()
        .expect("Failed to execute command");
        println!("{:#?}", output);
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            println!("Docker Version: {}", stdout);
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            panic!("Error: {}", stderr);
        }
            }
        }   
        }
}

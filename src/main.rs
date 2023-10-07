// CLI tool for building, tagging, and pushing docker containers.

// example) rdock ../.dockerfile
// where argument is only path, rdock finds the first build, tag, and push tags to use

// example) rdock -c sierra-api ../../.dockerfile
// where -c says find the comment where the next arg is found for each build, tag, and push. then last is the path to docker file.
// good for projects with multiple builds in same file

use clap::Parser;

#[derive(Parser)]
struct Cli {
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
}

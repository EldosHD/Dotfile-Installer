use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
      
}

fn main() {
    let args = Cli::parse();
    println!("{:?}", args);
}

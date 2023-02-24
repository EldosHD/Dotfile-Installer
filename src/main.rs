use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// List all packages that can be installed
    #[arg(short, long)]
    list: bool,

    /// Upgrade system, install all packages and link dotfiles
    #[arg(short, long)]
    all: bool,

    /// Upgrade system, install all packages without linking dotfiles
    #[arg(short, long)]
    install: bool,

    /// Add PPAs
    #[arg(short, long)]
    ppas: bool,

    /// Upgrade system
    #[arg(short, long)]
    upgrade: bool,

    /// Link dotfiles
    #[arg(short = 'L', long)]
    link: bool,

    /// Don't link dotfiles. Just show what would be linked
    #[arg(short, long)]
    dry: bool,

    /// Install shell related packages
    #[arg(short, long)]
    shell: bool,

    /// Install general tools
    #[arg(short, long)]
    tools: bool,

    /// Install programming languages
    #[arg(short = 'P', long)]
    languages: bool,

    /// Install editors
    #[arg(short, long)]
    editors: bool,

    /// Install misc packages
    #[arg(short, long)]
    misc: bool,

    /// Run additional scripts
    #[arg(short = 'A', long)]
    additional: bool,

    /// Don't log anything
    #[arg(short, long)]
    no_log: bool,

    /// Specify a custom log file
    /// If not specified, the default log file (log.txt) is used
    #[arg(short = 'F', long, default_value = "log.txt")]
    log_file: std::path::PathBuf,

    /// Force all checks to pass
    #[arg(short, long)]
    force: bool,

    #[clap(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
}

fn main() {
    let mut args = Cli::parse();

    if args.list {
        // TODO: List all packages that can be installed
        panic!("Not implemented");
    }

    if args.all {
        args.install = true;
        args.link = true;
    }

    if args.install {
        args.ppas = true;
        args.upgrade = true;
        args.shell = true;
        args.tools = true;
        args.languages = true;
        args.editors = true;
        args.misc = true;
        args.additional = true;
    }

    if !args.no_log {
        if args.log_file.exists() {
            if !args.force {
                println!("Log file already exists. Use --force to overwrite");
                std::process::exit(1);
            } else {
                std::fs::remove_file(&args.log_file).unwrap();
            }
        }
    }

    println!("{:?}", args);
}

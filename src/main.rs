use clap::Parser;
use inzip::InZip;
use std::process;

#[derive(Parser)]
#[command(name = "inzip", about = "List content of a ZIP file")]
struct Cli {
    zip_file: String,

    #[arg(short = 'p', long = "present")]
    present: Option<String>,

    #[arg(short = 'l', long = "list")]
    list: bool,

    #[arg(short = 'e', long = "empty")]
    empty: bool,
}

fn main() {
    let cli = Cli::parse();

    let archive = InZip::from_file(&cli.zip_file).unwrap_or_else(|e| {
        eprintln!("Error opening zip file {}: {e}", cli.zip_file);
        process::exit(1);
    });

    if let Some(present) = cli.present {
        if archive.exists(&present) {
            println!("File {} is present in archive", present);
            return;
        }
        println!("File {} is NOT present", present);
        process::exit(1);
    }

    if cli.empty {
        if archive.is_empty() {
            println!("Archive is empty");
            return;
        }
        println!("Archive is NOT empty");
        process::exit(1);
    }

    let contents = archive.contents();
    if contents.is_empty() {
        println!("File {} is empty", cli.zip_file);
        return;
    }
    for entry in contents {
        println!("{entry}");
    }
}

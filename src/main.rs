use std::{fs, process::exit};
use yansi::Paint;
use clap::Parser;
use std::process::Command;

/// A simple utility to install replit's rui library
#[derive(Parser, Debug)]
#[command(author = "@JustCoding123", version = "0.0.1", about, long_about = None)]
struct Args {
    /// The output directory.
    #[arg(short = 'o', long, default_value = "./rui")]
    outdir: String,
}

fn raise_err(message: &str) {
    println!("{}", Paint::red(format!("  × Failed - {}", message)));
    exit(1);
}

fn run_command(command: String) {
    let res = Command::new("cmd")
        .args([ "/C", &command ])
        .output();

    if res.is_err() {
        raise_err(&format!("Failed to run command {}", command));
    }
}

#[cfg(target_family = "windows")]
fn get_temporary() -> String {
    return ".\\__installer_temp".to_owned();
}

#[cfg(target_family = "unix")]
fn get_temporary() -> String {
    return "./__installer_temp".to_owned();
}

fn main() {
    if cfg!(windows) && !Paint::enable_windows_ascii() {
        Paint::disable();
    }

    let args = Args::parse();
    let temporary_dir = get_temporary();

    println!("{} {} {} {}", Paint::blue("installing"), Paint::yellow("rui").bold(), Paint::blue("into"), Paint::yellow(&args.outdir).bold());
    run_command(format!("git clone --quiet https://github.com/replit/extensions/ {}", temporary_dir));
    run_command(format!("git -C {} checkout --quiet c5c3b73ac14b625f18fdeace6512765918c37cbf", temporary_dir));

    if fs::metadata(&args.outdir).is_ok() {
        let remove_res = fs::remove_dir_all(&args.outdir);
        
        if remove_res.is_err() {
            raise_err("Failed to remove old rui directory.");
        }
    }

    let rename_res = fs::rename(format!("{}/src/rui", temporary_dir), args.outdir);
    
    if rename_res.is_err() {
        raise_err("Failed to move the rui directory.");
    }

    let remove_res = fs::remove_dir_all(temporary_dir);
    
    if remove_res.is_err() {
        raise_err("Failed to remove the temporary directory");
    }

    println!("{}", Paint::green("  ✓ Done!"));
}
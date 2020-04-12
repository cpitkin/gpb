use quicli::prelude::*;
use structopt::StructOpt;
use std::thread;
use std::path::Path;

mod file_monitor;

/// GPB (Google Photos Backup CLI) - Convert Google Takeout files into something we can backup to Backblaze
#[derive(Debug, StructOpt)]
struct Cli {
    // Add a positional argument that the user has to supply:
    /// Path to a working directory
    #[structopt(default_value = "./", long)]
    path: String,
    // Quick and easy logging setup you get for free with quicli
    #[structopt(flatten)]
    verbosity: Verbosity,
}

fn main() -> CliResult {
    let _args = Cli::from_args();

    let full_watcher_path = format!("{}/zip_files", &_args.path);

    println!("Please place your zip files here: {:?}", full_watcher_path);

    let watch_dir = Path::new(&full_watcher_path);

    let handle = thread::spawn(move || {

        let file_name = file_monitor::watch_for_zip(watch_dir);
        match file_name {
            Ok(value) => println!("{:?}", value),
            Err(e) => println!("{:?}", e)
        }
    });

    handle.join().unwrap();

    Ok(())
}

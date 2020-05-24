use structopt::StructOpt;

use gpba::*;

/// GPBA (Google Photos Backup Assistant) - Convert Google Takeout files into something we can easily backup to a service
#[derive(Debug, StructOpt)]
struct Opt {
    // The number of occurrences of the `v/verbose` flag
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[structopt(short, long, parse(from_occurrences))]
    verbose: u8,

    /// Setup a new directory structure for the CLI
    #[structopt(short, long, parse(from_flag))]
    new: bool,

    /// Keep the compressed (ZIP) and expanded files
    #[structopt(short, long, parse(from_flag))]
    keep: bool,

    /// Path to use as the working directory
    #[structopt(name = "DIRECTORY", parse(from_str))]
    directory: String,
}

fn main() {
    let opt = Opt::from_args();

    let directories: Directories = Directories::new(opt.directory);
    if opt.new {
        Directories::create(&directories)
        .expect("Something went wrong creating the directories");
    }

    let compressed_files = gpba::get_dir_files(&directories.compressed).unwrap();

    for compressed_file in compressed_files {
        gpba::unzip_files(&compressed_file, &directories, &opt.verbose).expect("Can't unzip the file.");
        let expanded_files = gpba::get_dir_files(&directories.expanded).unwrap();

        for expanded_file in expanded_files {
            gpba::merge(&directories, &expanded_file, &opt.verbose);

            gpba::clean_up_expanded(&expanded_file)
                .expect("Failed to clean up files and directories!");


            if !opt.keep {
                gpba::clean_up_compressed(&compressed_file)
                    .expect("Failed to clean up files and directories!");
            }
        }
    }
}

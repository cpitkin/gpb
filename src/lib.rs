use std::io;
use std::fs::{create_dir_all, read_dir};
use std::fs;
use std::path::{Path, PathBuf};
use zip::*;

#[derive(Debug)]
pub struct Directories {
    pub compressed: String,
    pub expanded: String,
    pub merged: String,
}

impl Directories {
    pub fn new(dir: String) -> Directories {

        Directories {
            compressed: format!("./{}/compressed", dir),
            expanded: format!("./{}/expanded", dir),
            merged: format!("./{}/merged", dir),
        }
    }

    pub fn create(directories: &Directories)-> std::io::Result<()> {
        create_dir_all(&directories.compressed)?;
        create_dir_all(&directories.expanded)?;
        create_dir_all(&directories.merged)?;
        Ok(())
    }
}

pub fn get_dir_files(dir: &str) -> io::Result<Vec<PathBuf>> {
    let mut entries = read_dir(dir)?
        .map(|res|
            res
                .map(|e|
                e.path()
            )
        )
        .collect::<Result<Vec<_>, io::Error>>()?;

    // The order in which `read_dir` returns entries is not guaranteed. If reproducible
    // ordering is required the entries should be explicitly sorted.

    entries.sort();

    // The entries have now been sorted by their path.

    Ok(entries)
}

pub fn unzip_files (file: &PathBuf, directories: &Directories) -> std::io::Result<()> {

    let fname = std::path::Path::new(&file);
    let file = fs::File::open(&fname).unwrap();

    let mut archive = ZipArchive::new(file).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        // let outpath = file.sanitized_name();
        let outpath = Path::new(&directories.expanded).join(file.sanitized_name());

        {
            let comment = file.comment();
            if !comment.is_empty() {
                println!("File {} comment: {}", i, comment);
            }
        }

        if (&*file.name()).ends_with('/') {
            println!("File {} extracted to \"{}\"", i, outpath.as_path().display());
            create_dir_all(&outpath).unwrap();
        } else {
            println!("File {} extracted to \"{}\" ({} bytes)", i, outpath.as_path().display(), file.size());
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    create_dir_all(&p).unwrap();
                }
            }

            let mut outfile = fs::File::create(&outpath).unwrap();

            println!("{:?}", outfile);
            io::copy(&mut file, &mut outfile).unwrap();
        }

        // Get and Set permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
            }
        }
    }

    Ok(())
}

pub fn merge (directories: &Directories, file: &PathBuf) {

    let console_info = rusync::ConsoleProgressInfo::new();

    let options = rusync::SyncOptions::default();

    let source = std::path::Path::new(&file);
    let destination = std::path::Path::new(&directories.merged);

    println!("SOURCE: {:?}", source);
    println!("DESTINATION: {:?}", destination);

    let syncer = rusync::Syncer::new(&source, &destination, options, Box::new(console_info));
    let stats = syncer.sync();
    match stats {
        Err(err) => {
            eprintln!("Error when syncing: {}", err);
        }
        Ok(stats) => {
            println!("Transfered {} files", stats.copied);
        }
    }
}

pub fn clean_up (expanded_file: &PathBuf, compressed_file: &PathBuf) -> std::io::Result<()> {
    fs::remove_dir_all(&expanded_file)?;
    fs::remove_file(&compressed_file)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dir_tree() {
        let dir = "testing".to_string();

        let dir_path = format!("./{}/compressed", dir);

        let dir_struct = Directories::new(dir);

        assert_eq!(dir_path, dir_struct.compressed);
    }
}

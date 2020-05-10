use std::fs;

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

    pub fn create(dirs: &Directories)-> std::io::Result<()> {
        fs::create_dir_all(&dirs.compressed)?;
        fs::create_dir_all(&dirs.expanded)?;
        fs::create_dir_all(&dirs.merged)?;
        Ok(())
    }
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

use std::fs;
use std::io;

pub fn remove_sorted(dir: &str) -> io::Result<()> {
    let p = fs::read_dir(dir)?;
    
    for folder in p {
        let folder = folder?;
        let path = folder.path();

        if path.is_dir() {
            fs::remove_dir_all(&path)?;
        } else if path.is_file() {
            fs::remove_file(&path)?;
        } else {
            
        }
    }

    Ok(())
}

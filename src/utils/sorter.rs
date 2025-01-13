use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::Path;

fn get_files(dir: &str) -> io::Result<Vec<String>> {
    let mut files = Vec::new();
    
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            if let Some(file) = path.file_name().and_then(|name| name.to_str()) {
                files.push(file.to_string());
            }
        }
        else if path.is_dir() {
            if let Some(dir) = path.file_name().and_then(|name| name.to_str()) {
                files.push(dir.to_string());
            }
        }
    }
    Ok(files)
}

fn exists(dir: &str) -> bool {
    fs::metadata(dir).map(|metadata| metadata.is_dir()).unwrap_or(false)
}

fn copy_to(src: &str, dest: &str) -> io::Result<()> {
    let mut src_ = File::open(src)?;
    let mut contents = Vec::new();
    src_.read_to_end(&mut contents)?;

    let mut dest_ = File::create(dest)?;
    dest_.write_all(&contents)?;

    Ok(())
}

fn sort_item(item_path: &str, _path: &str) -> bool {
    let files = match get_files(item_path) {
        Ok(files) => files,
        Err(_) => return false, 
    };

    for file in files {
        let fp = Path::new(item_path).join(&file);

        if fp.is_dir() {
            if !sort_items(fp.to_str().unwrap().to_string()) {
                return false;
            }
        } else {
            if let Some(f_c) = file.chars().next() {
                let f_c = f_c.to_lowercase().to_string(); 

                let dest_folder = format!("./sorted/{}", f_c);
                
                if !exists(&dest_folder) {
                    if let Err(e) = fs::create_dir_all(&dest_folder) {
                        return false;
                    }
                }

                let dest_file_path = format!("{}/{}", dest_folder, file);

                if let Err(e) = fs::write(&dest_file_path, &file) {
                    return false;
                }

                if let Err(e) = copy_to(fp.to_str().unwrap(), &dest_file_path) {
                    return false;
                }
            }
        }
    }

    true
}

pub fn sort_items(path: String) -> bool {
    sort_item(&path, &path)
}

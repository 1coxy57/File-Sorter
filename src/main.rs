use std::fs;
use std::time::Instant;
use std::path::Path;
mod utils;
use std::io::{self, Write};
use utils::sorter::sort_items;
use utils::remove::remove_sorted;


fn print_banner() {
    println!(r#"
                                  d8b             d8b                                                      
                              88P             ?88                                                      
                             d88               88b                                                     
  88bd8b,d88b  d888b8b   d888888   d8888b      888888b ?88   d8P      d8888b d8888b ?88,  88P?88   d8P 
  88P'`?8P'?8bd8P' ?88  d8P' ?88  d8b_,dP      88P `?8bd88   88      d8P' `Pd8P' ?88 `?8bd8P'd88   88  
 d88  d88  88P88b  ,88b 88b  ,88b 88b         d88,  d88?8(  d88      88b    88b  d88 d8P?8b, ?8(  d88  
d88' d88'  88b`?88P'`88b`?88P'`88b`?888P'    d88'`?88P'`?88P'?8b     `?888P'`?8888P'd8P' `?8b`?88P'?8b 
                                                              )88                                   )88
                                                             ,d8P                                  ,d8P
                                                          `?888P'                               `?888P'
    "#)
}



fn create_sorted() -> io::Result<()> {
    let org_folder = Path::new("./sorted");

    if !org_folder.exists() {
        fs::create_dir_all(org_folder)?;
    } else {
    }

    Ok(())
}

fn main() {
    print_banner();
    let now = Instant::now();
    let mut option = String::new();

    if let Err(e) = create_sorted() {
        println!("Error while creating sorted folder.")
    }
 
    let _rem = remove_sorted("./sorted").unwrap();
    println!("Enter the directory you want to sort --> ");
    
    std::io::stdin().read_line(&mut option).unwrap_or_else(|err| {
        println!("Cannot read input {}", err);
        std::process::exit(1);
    });
    
    let dir = option.trim();

    if sort_items(dir.to_string()) { 
        let elapsed = now.elapsed();
        let elp_sec = elapsed.as_secs();
        let srt_str = format!("Sorted files in {}s", elp_sec);
        println!("{}", srt_str);
    } else {
        println!("Could not sort items. An error occured. Most likely directory doesn't exist.");
    }
}

use std::{io::Write, path::PathBuf};
use reqwest;

fn main() {
    let flag_test = false;

    let mut url = String::new();
    let mut base_file_name = String::new();
    let mut path_string = String::new();

    print!("Provide image url: ");
    let _ = std::io::stdout().flush();
    let _ = std::io::stdin().read_line(&mut url);
    url = url.trim_end().replace("001.jpg", "");
    print!("Provide base image name: ");
    let _ = std::io::stdout().flush();
    let _ = std::io::stdin().read_line(&mut base_file_name);
    base_file_name = base_file_name.trim_end().to_owned();
    print!("Provide path to save images: ");
    let _ = std::io::stdout().flush();
    let _ = std::io::stdin().read_line(&mut path_string);
    path_string = path_string.trim_end().to_owned();
    let _ = std::fs::create_dir_all(&path_string);

    let file_path = PathBuf::from(path_string);
    let mut response = reqwest::blocking::get(url.clone() + "001.jpg").unwrap();

    let mut counter = 1; 
    while response.status().is_success() && (counter == 1 || !flag_test) {
        // let mut file = std::fs::File::create(base_file_name.clone() + &counter.to_string() + ".jpg").unwrap();
        let mut file = std::fs::File::create(file_path.to_str().unwrap().to_owned() + "/" + &base_file_name.clone() + &counter.to_string() + ".jpg").unwrap();
        response.copy_to(&mut file).unwrap();
        counter += 1;
        response = reqwest::blocking::get(url.clone() + &format!("{:03}", counter) + ".jpg").unwrap();
    }
}

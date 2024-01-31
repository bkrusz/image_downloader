use reqwest;

fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer);
    let mut file = std::fs::File::create("image.jpg").unwrap();
    reqwest::blocking::get(buffer.trim_end()).unwrap().copy_to(&mut file).unwrap();
}

use extractor::check_file;

fn main() {
    println!("Hello, extra!");
    let file_path = String::from("./extractor");
    check_file(&file_path);
}
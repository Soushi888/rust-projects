fn main() {
  let mut args = std::env::args();

  let path = args.nth(1);
  match path {
    Some(path) => println!("{}", read_file(&path)),
    None => println!("Please supply a path!"),
  }
}

fn read_file(path: &str) -> String {
  let content_result = std::fs::read_to_string(path);

  match content_result {
    Ok(content) => content,
    Err(error) => format!("Problem opening the file: {}", error.kind()),
  }
}

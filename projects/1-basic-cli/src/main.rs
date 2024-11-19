use std::fs::File;
use std::io::{self, Read, Write};

fn main() -> io::Result<()> {
  let args: Vec<String> = std::env::args().collect();

  if args.len() < 2 {
    eprintln!("Usage: {} <filename>", args[0]);
    return Ok(());
  }

  let filename = &args[1];
  let content = read_file(filename)?;
  println!("{}", content);

  loop {
    print!("Do you want to read another file? (y/n): ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    match input.trim().to_lowercase().as_str() {
      "y" => {
        print!("Enter filename: ");
        io::stdout().flush()?;

        let mut filename = String::new();
        io::stdin().read_line(&mut filename)?;
        let filename = filename.trim();

        let content = read_file(filename)?;
        println!("{}", content);
      }
      "n" | "" => {
        println!("Bye!");
        break;
      }
      _ => println!("Invalid input. Please enter y or n."),
    }
  }

  Ok(())
}

/// Reads the contents of a file and returns them as a string.
///
/// ## Errors
///
/// Returns an error if there was a problem opening or reading the file.
fn read_file(filename: &str) -> io::Result<String> {
  let file = File::open(filename)?;
  let mut reader = io::BufReader::new(file);
  let mut content = String::new();
  reader.read_to_string(&mut content)?;
  Ok(content)
}

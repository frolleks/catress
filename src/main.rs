use std::env;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs;

// Define a custom error type
#[derive(Debug)]
struct CustomError {
    message: String,
}

impl CustomError {
    fn new(message: &str) -> CustomError {
        CustomError {
            message: message.to_string(),
        }
    }
}

impl Display for CustomError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for CustomError {}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() == 1 {
        let file_path = &args[0];
        let contents =
            fs::read_to_string(&file_path).map_err(|err| CustomError::new(&err.to_string()))?;
        println!("{}", contents);
    } else {
        for file_path in args {
            let contents =
                fs::read_to_string(&file_path).map_err(|err| CustomError::new(&err.to_string()))?;

            println!("{}\n", contents);
        }
    }

    Ok(())
}

mod error;
mod stack;

use error::Error;
use stack::Stack;
use std::io::{self, Write};

fn main() -> Result<(), Error> {
    let mut stack = Stack::new();
    loop {
        print!("► ");
        io::stdout().flush()?;
        let mut line = String::new();
        io::stdin().read_line(&mut line)?;
        for op in line.split_whitespace() {
            match op.parse().and_then(|op| stack.op(op)) {
                Ok(quit) => {
                    if quit {
                        return Ok(());
                    }
                }
                Err(error) => eprintln!("Error: {}", error),
            }
        }
    }
}

use std::io::{self, Read, Write};
use konami_cipher::*;

fn main() {
    let mut stdin_buffer = String::new();

    loop {
        #[derive(PartialEq)]
        enum Action {
            Unset,
            Encrypt,
            Decrypt
        }
        let mut action = Action::Unset;
        while action == Action::Unset {
            stdin_buffer.clear();
            print!("Do you want to encrypt or decrypt? ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut stdin_buffer).unwrap();
            action = match stdin_buffer.to_lowercase().trim_end() {
                "encrypt" => Action::Encrypt,
                "decrypt" => Action::Decrypt,
                _ => {println!("Please enter either encrypt or decrypt"); Action::Unset}
            };
        }
        
        stdin_buffer.clear();
        print!("Enter the message you would like to {}: ", match action {
            Action::Encrypt => "encrypt",
            Action::Decrypt => "decrypt",
            _ => panic!("Invalid action value")
        });
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut stdin_buffer).unwrap();
        let message = stdin_buffer.trim_end().to_owned();

        stdin_buffer.clear();
        print!("Enter your key: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut stdin_buffer).unwrap();
        let key = stdin_buffer.trim_end().to_owned();
        
        let output = if action == Action::Encrypt {
            encrypt(message, key)
        } else {
            decrypt(message, key)
        };
        match output {
            Ok(output) => println!("Output: {}\n", output),
            Err(error) => println!("Error calculating output: {:?}\n", error)
        }
    }
}

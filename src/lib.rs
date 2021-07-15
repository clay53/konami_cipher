use std::fmt::Write;

#[derive(Debug)]
pub enum KonamiParseError {
    Empty,
    TooManyUpDown(usize),
    UexpectedCharacter(usize, char)
}

pub fn konami_to_bytes(konami: String) -> Result<Vec<u8>, KonamiParseError> {
    let mut bytes = Vec::new();

    #[derive(Debug)]
    enum ParseState {
        Up(u8), // 0: up count
        Down(u8, u8), // 0: current value; 1: down count
        L(u8), // 0: current value
        R(u8), // 0: current value
        LR(u8), // 0: current value
        LL(u8), // 0: current value
        RR(u8), // 0: current value
        LRL(u8), // 0: current value
        LRLR(u8), // 0: current value
        B(u8), // 0: current value
        A(u8), // 0: current value
        BA(u8), // 0: current value
        BB(u8), // 0: current value
        AA(u8), // 0: current value
        AB(u8), // 0: current value
        S(u8), // 0: current value
        St(u8), // 0: current value
        Sta(u8), // 0: current value
        Star(u8), // 0: current value
    }
    
    let mut parse_state = ParseState::Up(0);
    for (i, c) in konami.chars().enumerate() {
        match parse_state {
            ParseState::Up(up_count) => {
                let current_up_value = match up_count {
                    0 => 3,
                    1 => 2,
                    2 => 0,
                    3 => 1,
                    _ => return Err(KonamiParseError::TooManyUpDown(i))
                };
                match c {
                    '^' => parse_state = ParseState::Up(up_count+1),
                    'v' => parse_state = ParseState::Down(current_up_value, 1),
                    '<' => parse_state = ParseState::L(current_up_value+12),
                    '>' => parse_state = ParseState::R(current_up_value+12),
                    _ => return Err(KonamiParseError::UexpectedCharacter(i, c))
                }
            },
            ParseState::Down(current_value, down_count) => {
                let current_down_value = match down_count {
                    0 => 3,
                    1 => 2,
                    2 => 0,
                    3 => 1,
                    _ => return Err(KonamiParseError::TooManyUpDown(i))
                }*4;
                match c {
                    'v' => parse_state = ParseState::Down(current_value, down_count+1),
                    '<' => parse_state = ParseState::L(current_value+current_down_value),
                    '>' => parse_state = ParseState::R(current_value+current_down_value),
                    _ => return Err(KonamiParseError::UexpectedCharacter(i, c))
                }
            },
            ParseState::L(current_value) => parse_state = match c {
                '>' => ParseState::LR(current_value),
                '<' => ParseState::LL(current_value),
                _ => return Err(KonamiParseError::UexpectedCharacter(i, c))
            },
            ParseState::R(current_value) => parse_state = match c {
                '>' => ParseState::RR(current_value),
                _ => return Err(KonamiParseError::UexpectedCharacter(i, c))
            },
            ParseState::LR(current_value) => parse_state = match c {
                '<' => ParseState::LRL(current_value),
                'b' => ParseState::B(current_value+16),
                'a' => ParseState::A(current_value+16),
                _ => return Err(KonamiParseError::UexpectedCharacter(i, c))
            },
            ParseState::LL(current_value) => parse_state = match c {
                'b' => ParseState::B(current_value+32),
                'a' => ParseState::A(current_value+32),
                _ => return Err(KonamiParseError::UexpectedCharacter(i, c))
            },
            ParseState::RR(current_value) => parse_state = match c {
                'b' => ParseState::B(current_value+48),
                'a' => ParseState::A(current_value+48),
                _ => return Err(KonamiParseError::UexpectedCharacter(i, c))
            },
            ParseState::LRL(current_value) => parse_state = match c {
                '>' => ParseState::LRLR(current_value),
                _ => return Err(KonamiParseError::UexpectedCharacter(i, c))
            },
            ParseState::LRLR(current_value) => parse_state = match c {
                'b' => ParseState::B(current_value),
                'a' => ParseState::A(current_value),
                _ => return Err(KonamiParseError::UexpectedCharacter(i, c))
            },
            ParseState::B(current_value) => parse_state = match c {
                'a' => ParseState::BA(current_value),
                'b' => ParseState::BB(current_value),
                _ => return Err(KonamiParseError::UexpectedCharacter(i, c))
            },
            ParseState::A(current_value) => parse_state = match c {
                'a' => ParseState::AA(current_value),
                'b' => ParseState::AB(current_value),
                _ => return Err(KonamiParseError::UexpectedCharacter(i, c))
            },
            ParseState::BA(current_value) => parse_state = match c {
                'S' => ParseState::S(current_value),
                _ => return Err(KonamiParseError::UexpectedCharacter(i, c))
            },
            ParseState::BB(current_value) => parse_state = match c {
                'S' => ParseState::S(current_value+64),
                _ => return Err(KonamiParseError::UexpectedCharacter(i, c))
            },
            ParseState::AA(current_value) => parse_state = match c {
                'S' => ParseState::S(current_value+128),
                _ => return Err(KonamiParseError::UexpectedCharacter(i, c))
            },
            ParseState::AB(current_value) => parse_state = match c {
                'S' => ParseState::S(current_value+192),
                _ => return Err(KonamiParseError::UexpectedCharacter(i, c))
            },
            ParseState::S(current_value) => parse_state = match c {
                't' => ParseState::St(current_value),
                _ => return Err(KonamiParseError::UexpectedCharacter(i, c))
            },
            ParseState::St(current_value) => parse_state = match c {
                'a' => ParseState::Sta(current_value),
                _ => return Err(KonamiParseError::UexpectedCharacter(i, c))
            },
            ParseState::Sta(current_value) => parse_state = match c {
                'r' => ParseState::Star(current_value),
                _ => return Err(KonamiParseError::UexpectedCharacter(i, c))
            },
            ParseState::Star(current_value) => parse_state = match c {
                't' => {
                    bytes.push(current_value);
                    ParseState::Up(0)
                },
                _ => return Err(KonamiParseError::UexpectedCharacter(i, c))
            }
        }
    }
    if bytes.is_empty() {
        return Err(KonamiParseError::Empty)
    } else {
        return Ok(bytes)
    }
}

pub fn byte_to_konami(mut byte: u8) -> String {
    let mut konami = String::new();
    let mut count_64 = 0;
    let mut count_16 = 0;
    let mut count_4 = 0;
    let mut count_1 = 0;

    while byte >= 64 {
        count_64 += 1;
        byte -= 64;
    }

    while byte >= 16 {
        count_16 += 1;
        byte -= 16;
    }

    while byte >= 4 {
        count_4 += 1;
        byte -= 4;
    }

    while byte >= 1 {
        count_1 += 1;
        byte -= 1;
    }

    konami.push_str(match count_1 {
        0 => "^^",
        1 => "^^^",
        2 => "^",
        3 => "",
        _ => panic!()
    });

    konami.push_str(match count_4 {
        0 => "vv",
        1 => "vvv",
        2 => "v",
        3 => "",
        _ => panic!()
    });

    konami.push_str(match count_16 {
        0 => "<><>",
        1 => "<>",
        2 => "<<",
        3 => ">>",
        _ => panic!()
    });

    konami.push_str(match count_64 {
        0 => "ba",
        1 => "bb",
        2 => "aa",
        3 => "ab",
        _ => panic!()
    });

    konami.push_str("Start");
    return konami
}

pub fn bytes_to_konami(bytes: Vec<u8>) -> String {
    let mut konami = String::new();
    for byte in bytes {
        write!(konami, "{}", byte_to_konami(byte)).unwrap();
    }
    return konami
}

pub fn encrypt(message: String, konami_offsets: String) -> Result<String, KonamiParseError> {
    let offsets = konami_to_bytes(konami_offsets)?;
    let mut offsets = offsets.iter().cycle();
    let mut encrypted = String::new();
    for b in message.into_bytes() {
        write!(encrypted, "{}", byte_to_konami(((b as u16 + offsets.next().unwrap().clone() as u16)%256) as u8)).unwrap();
    }
    return Ok(encrypted);
}

pub fn decrypt(konami_message: String, konami_offsets: String) -> Result<String, KonamiParseError> {
    let offsets = konami_to_bytes(konami_offsets)?;
    let mut offsets = offsets.iter().cycle();
    let bytes = konami_to_bytes(konami_message)?;
    let mut decrypted = String::new();
    for b in bytes {
        decrypted.push(((256 + b as u16 - offsets.next().unwrap().clone() as u16)%256) as u8 as char);
    }
    return Ok(decrypted)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn konami_to_bytes_1_0() {
        assert!(vec!(0).eq(&konami_to_bytes("^^vv<><>baStart".to_string()).unwrap()));
    }

    #[test]
    fn konami_to_bytes_2_0() {
        assert!(vec!(0, 0).eq(&konami_to_bytes("^^vv<><>baStart^^vv<><>baStart".to_string()).unwrap()));
    }

    #[test]
    fn konami_to_bytes_noup() {
        assert!(vec!(3).eq(&konami_to_bytes("vv<><>baStart".to_string()).unwrap()));
    }

    #[test]
    fn konami_to_bytes_nodown() {
        assert!(vec!(12).eq(&konami_to_bytes("^^<><>baStart".to_string()).unwrap()));
    }

    #[test]
    fn konami_to_bytes_noupdown() {
        assert!(vec!(15).eq(&konami_to_bytes("<><>baStart".to_string()).unwrap()));
    }

    #[test]
    fn konami_to_bytes_255_0() {
        assert!(vec!(255).eq(&konami_to_bytes(">>abStart".to_string()).unwrap()));
    }

    #[test]
    fn bytes_to_konami_0() {
        let konami = bytes_to_konami(vec!(0));
        eprintln!("{}", konami);
        assert!(konami.eq("^^vv<><>baStart"));
    }

    #[test]
    fn bytes_to_konami_255() {
        let konami = bytes_to_konami(vec!(255));
        eprintln!("{}", konami);
        assert!(konami.eq(">>abStart"));
    }

    #[test]
    fn konami_to_bytes_bytes_to_konami() {
        for i in 0..=255 {
            let bytes = vec!(i);
            eprintln!("Offsets: {:?}", bytes);
            let calculated_konami = bytes_to_konami(bytes.clone());
            let calculated_bytes = konami_to_bytes(calculated_konami.clone()).unwrap();
            eprintln!("calculated_konami: {}\ncalculated bytes: {:?}", calculated_konami, calculated_bytes);
            assert!(bytes.eq(&calculated_bytes));
        }
    }

    #[test]
    fn the_quick_brown_fox_0_255() {
        let message = "The quick brown fox jumped over the lazy dog.".to_string();
        let konami = "^^vv<><>baStart>>abStart".to_string();
        let encrypted = encrypt(message.clone(), konami.clone()).unwrap();
        let decrypted = decrypt(encrypted.clone(), konami.clone()).unwrap();
        eprintln!("Message: {}\nKonami: {}\nEncrypted: {}\nDecrypted: {}", message, konami, encrypted, decrypted);
        assert_eq!(message, decrypted);
    }
}
use std::string::String;
use rand::Rng;
use rand::seq::SliceRandom;
use rand::distributions::Uniform;

const UPPER: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const LOWER: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
const NUMBER: &[u8] = b"0123456789";

pub struct Args {
    length: usize,
    num_pw: usize,
    enabled_symbols: String
}

impl Args {
    pub(super) fn new(length: usize, num_pw: usize, enabled_symbols: String) -> Self {
        Self {
            length,
            num_pw,
            enabled_symbols
        }
    }
}

type GenerateResult<T> = Result<T, Box<dyn std::error::Error>>;

pub(crate) fn generate_password(args: &Args) -> GenerateResult<Vec<String>> {
    if args.length < 5 {
        return Err("Length of the password must be greater than 5 to protect secure.".into())
    }
    else if args.num_pw < 1 {
        return Err("Number of passwords must be greater than or equal to 1.".into())
    }

    let enabled_symbols = args.enabled_symbols.as_bytes();

    let mut passwords: Vec<String> = Vec::with_capacity(args.num_pw);
    let mut rng = rand::thread_rng();

    let upper_indices = Uniform::new(0, UPPER.len());
    let lower_indices = Uniform::new(0, LOWER.len());
    let number_indices = Uniform::new(0, NUMBER.len());

    let symbol_indices;
    if enabled_symbols.is_empty() {
        symbol_indices = Uniform::new(0, 1);
    } else {
        symbol_indices = Uniform::new(0, enabled_symbols.len());
    }


    for _ in 0..args.num_pw {
        let mut password: Vec<char> = Vec::with_capacity(args.length);

        let mut pass_len = args.length;

        let mut symbol_num = 0;
        if !enabled_symbols.is_empty() {
            symbol_num = rng.gen_range(1..=pass_len / 4);
        }
        pass_len -= symbol_num;
        let lower_num = rng.gen_range(1..=pass_len - 2);
        pass_len -= lower_num;
        let number_num = rng.gen_range(1..=pass_len - 1);
        pass_len -= number_num;
        let upper_num = pass_len;

        for _ in 0..upper_num {
            password.push(UPPER[rng.sample(upper_indices)] as char);
        }
        for _ in 0 .. lower_num {
            password.push(LOWER[rng.sample(lower_indices)] as char);
        }
        for _ in 0 .. number_num {
            password.push(NUMBER[rng.sample(number_indices)] as char);
        }
        for _ in 0 .. symbol_num {
            password.push(enabled_symbols[rng.sample(symbol_indices)] as char);
        }
        password.shuffle(&mut rng);

        passwords.push(password.into_iter().collect());
    }

    Ok(passwords)

}
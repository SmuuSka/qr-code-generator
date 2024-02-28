use std::env;
use sha2::{Digest, Sha256};
use qr_code::QrCode;
use qrcode_generator::QrCodeEcc;

fn gen_qr(hash: &String, flag: &bool) {
    let _path = format!("./target/release/{}.png", hash);
    if !flag {
        let qr_code = QrCode::new(hash).unwrap();
        print!("{}", qr_code.to_string(false,3));
        return;
    }
    qrcode_generator::to_png_to_file(&hash, QrCodeEcc::Low, 1024, _path).unwrap();
    return;
}

fn hash(word: &String) -> String {
    let mut _word = String::new();
    let mut flag = false;
    if word.len() < 5 {
        let mut hasher = Sha256::new();
        hasher.update(word);
        let result = hasher.finalize();
        let res = result[..].iter().map(|b| format!("{:02x}", b)).collect();
        gen_qr(&res, &flag);
        return res;
    }
    let sliced = &word[word.len() - 5..];
    if sliced == "--png" {
        _word = word[..word.len() - 6].to_string();
        flag = true;
    }
    else {
        _word = word.to_string();
    }
    let mut hasher = Sha256::new();
    hasher.update(&_word);
    let result = hasher.finalize();
    let res = result[..].iter().map(|b| format!("{:02x}", b)).collect();
    gen_qr(&res, &flag);
    return res;
}

fn read_args(args: Vec<String>) -> String {
    let mut word = String::new();
    return if args.len() < 2 {
        println!("No arguments given");
        String::from("")
    } else {
        for index in 1..args.len() {
            if index == args.len() - 1 {
                word.push_str(&args[index]);
            } else {
                word.push_str(&args[index]);
                word.push_str(" ");
            }
        }
        word
    }
}

#[cfg(not(tarpaulin_include))]
fn main() {
    let args: Vec<String> = env::args().collect();
    let result = hash(&read_args(args.clone()));
    println!("The QR Code PNG has been generated based on command-line arguments. Input arguments:\n {:?} ----> Result: {}", args, result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash() {
        let result = hash(&String::from("moi"));
        assert_eq!(result, "a4f0e1d1b5eb23e3482a14b4a9d4e8106e83f7887471e4a73c2557ade280bfe5");
    }

    #[test]
    fn test_hash_1() {
        let result = hash(&String::from("testi"));
        assert_eq!(result, "26e19f2b4dd93a3a7c49c3e785ec8932550af6aa6bea13078672a8c81508f18e");
    }

    #[test]
    fn test_hash_empty() {
        let result = hash(&String::from(""));
        assert_eq!(result, "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");
    }

    #[test]
    fn test_hash_with_many_words() {
        let result = hash(&String::from("testi testi testi"));
        assert_eq!(result, "407e5bacb511ffaa0a05f7d01d8b033755db188e0a893169a1f51cc42ff8f491");
    }

    #[test]
    fn test_hash_with_flag() {
        let result = hash(&String::from("moi --png"));
        assert_eq!(result, "a4f0e1d1b5eb23e3482a14b4a9d4e8106e83f7887471e4a73c2557ade280bfe5");
    }

    #[test]
    fn test_read_args_with_arguments() {
        let args: Vec<String> = vec![String::from("testi"), String::from("moi")];
        let result = read_args(args);
        println!("{}", result);
        assert_eq!(result, "moi");
    }
    #[test]
    fn test_read_args_with_arguments_2() {
        let args: Vec<String> = vec![String::from("testi"), String::from("moi"), String::from("moi")];
        let result = read_args(args);
        println!("{}", result);
        assert_eq!(result, "moi moi");
    }

    #[test]
    fn test_read_args_without_arguments() {
        let args: Vec<String> = vec![String::from("testi")];
        let result = read_args(args);
        println!("{}", result);
        assert_eq!(result, "");
    }
}
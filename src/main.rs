use std::env;
use sha2::{Digest, Sha256};
use qr_code::QrCode;
use qrcode_generator::QrCodeEcc;

fn gen_qr(hash: &String, flag: &bool) {
    //if the flag is false, generate the qr code in the terminal
    if !flag {
        let qr_code = QrCode::new(hash).unwrap();
        print!("{}", qr_code.to_string(false,3));
        return;
    }
    //otherwise, generate the qr as a png file
    let qr_code = format!("{}.png", hash);
    qrcode_generator::to_png_to_file(&hash, QrCodeEcc::Low, 1024, qr_code).unwrap();
    return;
}

fn hash(args: &String) -> String {
    //initialize the flag
    let mut flag = false;
    let mut hasher = Sha256::new();
    //if args length is less than 5, args does not contain --png flag
    if args.len() < 5 || &args[args.len() - 5..] != "--png"{
        hasher.update(args);
        let result = hasher.finalize();
        let res = result[..].iter().map(|b| format!("{:02x}", b)).collect();
        gen_qr(&res, &flag);
        //return the hash
        return res;
    }
    let mut _word = String::new();
    //else if args contains --png flag, remove the flag from the string
    _word = args[..args.len() - 6].to_string();
    //set the flag to true
    flag = true;
    hasher.update(&_word);
    let result = hasher.finalize();
    let res = result[..].iter().map(|b| format!("{:02x}", b)).collect();
    //generate the qr code
    gen_qr(&res, &flag);
    //return the hash
    return res;
}

fn read_args(args: Vec<String>) -> String {
    //initialize the word
    let mut args_string = String::new();
    //if there are no user arguments, return an empty string for the hash function
    //otherwise, concatenate the arguments into a single string
    return if args.len() < 2 {
        String::from("")
    } else {
        //start from the second argument
        //if the index is the last one, do not add a space
        //otherwise, add a space between the args
        for index in 1..args.len() {
            if index == args.len() - 1 {
                args_string.push_str(&args[index]);
            } else {
                args_string.push_str(&args[index]);
                args_string.push_str(" ");
            }
        }
        //return the word for the hash function
        args_string
    }
}

#[cfg(not(tarpaulin_include))]
fn main() {
    //list of arguments
    let args: Vec<String> = env::args().collect();
    //result of the hash function
    let result = hash(&read_args(args.clone()));
    println!("The QR Code has been generated based on command-line arguments. Input arguments:\n {:?} ----> Result: {}", args, result);
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
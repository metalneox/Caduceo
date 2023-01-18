extern crate itertools;
use itertools::Itertools;
use std::collections::HashMap;

//Internal function to shift characters
fn shift_word(s: &str,num: usize) -> String {
    s.chars().map(|c| {
        if c.is_alphabetic() {
            let base = if c.is_uppercase() { 'A' } else { 'a' };
            ( ((c as u8) - (base as u8) + num as u8) % 26 + (base as u8) ) as char
        } else {
            c
        }
    }).collect()
}

fn sub_strings(source: &str, sub_size: usize) -> Vec<String> {
    source
        .chars()
        .chunks(sub_size)
        .into_iter()
        .map(|chunk| chunk.collect::<String>())
        .collect::<Vec<_>>()
}

//only for decrypt
fn polybius_chess(alphabet: &str, size: usize) -> HashMap<usize, char> {
    let mut chess: HashMap<usize, char> = HashMap::new();

    let mut index = 11;
    let mut flag_loop = 0;

    for c in alphabet.chars() {
        chess.insert(index + flag_loop, c);
        //If size == 5  'i' and 'j'  same position
        //if c != 'i' && size == 5 {
        //    flag_loop += 1;
        //}

        flag_loop += 1;

        if flag_loop >= size {
            flag_loop = 0;
            index += 10;
        }
    }
    chess
}
fn polybius_chess2(alphabet: &str, size: usize) -> HashMap<char, usize> {
    let mut chess: HashMap<char, usize> = HashMap::new();
    //let size = 5;

    let mut index = 11;
    let mut flag_loop = 0;

    for c in alphabet.chars() {
        chess.insert(c, index + flag_loop);
        //If size == 5  'i' and 'j'  same position
        //if c != 'i' && size == 5 {
        //    flag_loop += 1;
        //}
        flag_loop += 1;

        if flag_loop >= size {
            flag_loop = 0;
            index += 10;
        }
    }
    let temp = chess.get(&'i').unwrap().clone();

    chess.insert('j',temp);
    chess
}

/// Polybius Square code encoder for more information [here](https://en.wikipedia.org/wiki/Polybius_square) .
/// 
/// Example of how to use this library.
/// ```
/// use crate::caduceo::crypto::ciphers::*;
///
/// let result = polybius_crypt("ciao", 5);
/// assert_eq!(result , "13241134")
/// ```
#[allow(dead_code)]
pub fn polybius_crypt(text: &str, size: usize) -> String {
    let chars = "abcdefghiklmnopqrstuvwxyz";

    let square = polybius_chess2(chars, size);
    let mut result = String::new();
    /*  Print hashman debug
    for (key, value) in &square {
        println!("{}: {}", key, value);
    }
    */
    for r in text.chars() {
        match square.get(&r) {
            Some(res) => result.push_str(&res.to_string()),
            _ => (),
        }
    }
    result
}

/// Polybius Square encoder for more information [here](https://en.wikipedia.org/wiki/Polybius_square) .
/// 
/// Example of how to use this library.
/// ```
/// use crate::caduceo::crypto::ciphers::*;
///
/// let result = polybius_decrypt("13241134", 5);
/// assert_eq!(result , "ciao")
/// ```
pub fn polybius_decrypt(text: &str, size: usize) -> String {
    //let size = 5;
                       // tolto j perchè i e j sono uguali
    let chars = "abcdefghiklmnopqrstuvwxyz";

    let square = polybius_chess(chars, size);
    let mut result = String::new();
    let temp = sub_strings(text, 2);

    for str in temp {
        match square.get(&str.parse::<usize>().unwrap()) {
            Some(res) => result.push_str(&res.to_string()),
            _ => (),
        }
    }
    result
}



//TODO Rimuovere codice duplicato tra polybius chess,nihilist crypt/decrypt
//TODO Oltre al codice c'è anche strutture di dati doppie
/// Nihilist encoder for more information [here](https://en.wikipedia.org/wiki/Nihilist_cipher) .
#[allow(dead_code)]
pub fn nihilist_crypt(text: &str, key: &str) -> String {
    //default size
    let size = 5;

    let mut text_crypt = vec![];
    let mut key_crypt = vec![];

    let mut result = vec![];

    //alphabet mischiato
    let chars = "zebrascdfghiklmnopqtuvwxy";

    let square = polybius_chess2(chars, size);

    // text
    for r in text.chars() {
        match square.get(&r) {
            Some(res) => text_crypt.push(res),
            _ => (),
        }
    }

    // key
    for r in key.chars() {
        match square.get(&r) {
            Some(res) => key_crypt.push(res),
            _ => (),
        }
    }

    //sommo la key+text
    let mut flag = 0;
    for a in text_crypt{ 
        if flag >= key.len(){flag = 0;}        

        result.push( a + key_crypt[flag]);
        flag += 1;
    }

    result.iter().join(" ")
}

//FIX ME Da fixare
/// Nihilist decoder for more information [here](https://en.wikipedia.org/wiki/Nihilist_cipher) .
#[allow(dead_code)]
pub fn nihilist_decrypt(text: &str, key: &str) -> String {
    //metto 5 come size default
    let size = 5;

    let mut temp:Vec<usize> = vec![];

    let mut key_crypt = vec![];

    //alphabet mischiato
    let chars = "zebrascdfghiklmnopqtuvwxy";

    let square = polybius_chess2(chars, size);

    let square2 = polybius_chess(chars, size);

    let splitato = text.split(" ").collect::<Vec<&str>>();

    let mut result = String::new();

    // key
    for r in key.chars() {
        match square.get(&r) {
            Some(res) => key_crypt.push(res),
            _ => (),
        }
    }

    ////sommo la key+text
    let mut flag = 0;
    for a in splitato{ 
        if flag >= key.len(){flag = 0;}        

        temp.push( a.parse::<usize>().unwrap() - key_crypt[flag]);
        flag += 1;
    }

    for str in temp {
        match square2.get(&str) {
            Some(res) => result.push_str(&res.to_string()),
            _ => (),
        }
    }

    result

}


/// Rot13 code for more information [here](https://en.wikipedia.org/wiki/ROT13) .
#[allow(dead_code)]
pub fn rot13(text: &str) -> String {
    shift_word(text, 13usize)
}

///Cesar code for more information [here](https://en.wikipedia.org/wiki/Caesar_cipher) .
#[allow(dead_code)]
pub fn cesar(text: &str) -> String {
    shift_word(text, 3usize)
}

///Vigenere cipher encoder for more information [here](https://en.wikipedia.org/wiki/Vigen%C3%A8re_cipher) .
#[allow(dead_code)]
pub fn vigenere_crypt(frase: &str, key: &str) -> String {
    let chars = "abcdefghijklmnopqrstuvwxyz";
    let mut result = "".to_owned();

    let mut flag = 0;
    for c in frase.chars() {
        let c_index = chars.find(c);
        if flag >= key.len() {
            flag = 0;
        }
        if c_index.is_none() {
            result.push(c);
        } else {
            let n = chars.find(key.as_bytes()[flag] as char);
            let mut temp = c_index.unwrap() + n.unwrap();
            while temp > 26 {
                temp -= 26;
            }
            result.push(chars.as_bytes()[temp] as char);
        }
        flag += 1;
    }

    result
}

///Vigenere cipher decoder for more information [here](https://en.wikipedia.org/wiki/Vigen%C3%A8re_cipher) .
#[allow(dead_code)]
pub fn vigenere_decrypt(frase: &str, key: &str) -> String {
    let chars = "abcdefghijklmnopqrstuvwxyz";
    let mut result = "".to_owned();

    let mut flag = 0;
    for c in frase.chars() {
        let c_index = chars.find(c);
        if flag >= key.len() {
            flag = 0;
        }
        if c_index.is_none() {
            result.push(c);
        } else {
            let n = chars.find(key.as_bytes()[flag] as char);
            //let mut temp = c_index.unwrap()-n.unwrap();
            let mut temp = 0;
            if c_index.unwrap() >= n.unwrap() {
                temp = c_index.unwrap() - n.unwrap();
            } else {
                temp = 26 + c_index.unwrap() - n.unwrap();
            }

            while temp > 26 {
                temp -= 26;
            }
            result.push(chars.as_bytes()[temp] as char);
        }
        flag += 1;
    }

    result
}

///Carbonaro code for more information [here](https://en.wikipedia.org/wiki/Carbonari) .
#[allow(dead_code)]
pub fn carbonaro(text: &str) -> String {
    /*
        Originale: A|B|C|D|E|F|G|H|I|L|M|N|O|P|Q|R|S|T|U|V|Z
        Criptato:  O|P|G|T|I|V|C|H|E|R|N|M|A|B|Q|L|Z|D|U|F|S
    */
    let alphabet = "abcdefghilmnopqrstuvz";
    let carbo_alphabet = "opgtivchernmabqlzdufs";

    let mut result = "".to_owned();

    for c in text.chars() {
        let c_index = alphabet.find(c);
        result.push(carbo_alphabet.as_bytes()[c_index.unwrap()] as char);
    }

    result
}

///Morse code decoder for more information [here](https://en.wikipedia.org/wiki/Atbash) .
#[allow(dead_code)]
pub fn atbash(text: &str) -> String {
    /*
        Testo in chiaro:      a b c d e f g h i l m n o p q r s t u v z
        Testo cifrato:        Z V U T S R Q P O N M L I H G F E D C B A
    */
    let alphabet = "abcdefghilmnopqrstuvz";
    let atbash_alphabet = alphabet.chars().rev().collect::<String>();

    let mut result = "".to_owned();

    for c in text.chars() {
        let c_index = alphabet.find(c);
        result.push(atbash_alphabet.as_bytes()[c_index.unwrap()] as char);
    }

    result
}

//MORSE
///Morse code decoder for more information [here](https://en.wikipedia.org/wiki/Morse_code) .
#[allow(dead_code)]
pub fn morse_decoder(text: &str) -> char {
    match text {
        ".-" => 'a',
        "-..." => 'b',
        "-.-." => 'c',
        "-.." => 'd',
        "." => 'e',
        "..-." => 'f',
        "--." => 'g',
        "...." => 'h',
        ".." => 'i',
        ".---" => 'j',
        "-.-" => 'k',
        ".-.." => 'l',
        "--" => 'm',
        "-." => 'n',
        "---" => 'o',
        ".--." => 'p',
        "--.-" => 'q',
        ".-." => 'r',
        "..." => 's',
        "-" => 't',
        "..-" => 'u',
        "...-" => 'v',
        ".--" => 'w',
        "-..-" => 'x',
        "-.--" => 'y',
        "--.." => 'z',
        "-----" => '0',
        ".----" => '1',
        "..---" => '2',
        "...--" => '3',
        "....-" => '4',
        "....." => '5',
        "-...." => '6',
        "--..." => '7',
        "---.." => '8',
        "----." => '9',
        ".-.-.-" => '.',
        "--..--" => ',',
        "---..." => ':',
        "..--.." => '?',
        "-...-" => '=',
        "-....-" => '-',
        "-.--." => '(',
        "-.--.-" => ')',
        ".-..-." => '"',
        ".----." => '\'',
        "-..-." => '/',
        //"..--.-" => "TODO sottolineateo feature futura"
        ".--.-" => '@',
        "-.-.--" => '!',
        _ => ' ',
    }
}

///Morse code encoder for more information [here](https://en.wikipedia.org/wiki/Morse_code) .
#[allow(dead_code)]
pub fn morse_coder(character: char) -> &'static str {
    match character {
        'a' => ".-",
        'b' => "-...",
        'c' => "-.-.",
        'd' => "-..",
        'e' => ".",
        'f' => "..-.",
        'g' => "--.",
        'h' => "....",
        'i' => "..",
        'j' => ".---",
        'k' => "-.-",
        'l' => ".-..",
        'm' => "--",
        'n' => "-.",
        'o' => "---",
        'p' => ".--.",
        'q' => "--.-",
        'r' => ".-.",
        's' => "...",
        't' => "-",
        'u' => "..-",
        'v' => "...-",
        'w' => ".--",
        'x' => "-..-",
        'y' => "-.--",
        'z' => "--..",
        '0' => "-----",
        '1' => ".----",
        '2' => "..---",
        '3' => "...--",
        '4' => "....-",
        '5' => ".....",
        '6' => "-....",
        '7' => "--...",
        '8' => "---..",
        '9' => "----.",
        '.' => ".-.-.-",
        ',' => "--..--",
        ':' => "---...",
        '?' => "..--..",
        '=' => "-...-",
        '-' => "-....-",
        '(' => "-.--.",
        ')' => "-.--.-",
        '"' => ".-..-.",
        '\'' => ".----.",
        '/' => "-..-.",
        //"..--.-" => "sottolineateo feature futura"
        '@' => ".--.-",
        '!' => "-.-.--",
        _ => " ",
    }
}


//TODO VIC Crypt and Decrypt
/// VIC encoder for more information [here](https://en.wikipedia.org/wiki/VIC_cipher) .
#[allow(dead_code)]
pub fn VIC_crypt(text: &str, key: &str) -> String {

    "".to_string()
}
/// VIC decoder for more information [here](https://en.wikipedia.org/wiki/VIC_cipher) .
#[allow(dead_code)]
pub fn VIC_decrypt(text: &str, key: &str) -> String {

    "".to_string()
}

//TODO Hill Crypt and Decrypt
/// Hill encoder for more information [here](https://en.wikipedia.org/wiki/Hill_cipher).
#[allow(dead_code)]
pub fn hill_crypt(text: &str, key: &str) -> String {

    "".to_string()
}
/// Hill decoder for more information [here](https://en.wikipedia.org/wiki/Hill_cipher).
#[allow(dead_code)]
pub fn hill_decrypt(text: &str, key: &str) -> String {

    "".to_string()
}

//TODO ADFGVX Crypt and Decrypt
/// ADFGVX encoder for more information [here](https://en.wikipedia.org/wiki/ADFGVX_cipher).
#[allow(dead_code)]
pub fn adfgvx_crypt(text: &str, key: &str) -> String {

    "".to_string()
}

/// ADFGVX decoder for more information [here](https://en.wikipedia.org/wiki/ADFGVX_cipher).
#[allow(dead_code)]
pub fn adfgvx_decrypt(text: &str, key: &str) -> String {

    "".to_string()
}


/// Affine cipher for more information [here](https://en.wikipedia.org/wiki/Affine_cipher).
#[allow(dead_code)]
pub fn affine_crypt((a, b): (usize, usize), text: &str) -> String {
    let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();

    let mut result:String = String::new();

    for ch in text.chars() {
       
        let pos = alphabet.iter().position(|&c| c == ch.to_ascii_lowercase() ).unwrap();

        let new = (a*pos + b) % 26;

       
        let current = alphabet[new as usize];

        if ch.is_uppercase(){
            result.push(current.to_ascii_uppercase())
        }else{
            result.push(current);
        }    

    }
    result
}


#[allow(dead_code)]
pub fn affine_decrypt((a, b): (i32, i32), text: &str) -> String {
    let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();

    let mut result:String = String::new();

    for ch in text.chars() {
       
        let pos = alphabet.iter().position(|&c| c == ch.to_ascii_lowercase() ).unwrap();

        //BUG: Module negative number make problems in python work        
        let new = a*(pos as i32 - b as i32) % 26i32;

        let current = alphabet[new as usize];

        if ch.is_uppercase(){
            result.push(current.to_ascii_uppercase())
        }else{
            result.push(current);
        }    

    }

    result
}



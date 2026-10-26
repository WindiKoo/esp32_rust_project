
const MORSE_CODES: [(&str, &str); 36] = [
    ("A", ".-"),
    ("B", "-..."),
    ("C", "-.-."),
    ("D", "-.."),
    ("E", "."),
    ("F", "..-."),
    ("G", "--."),
    ("H", "...."),
    ("I", ".."),
    ("J", ".---"),
    ("K", "-.-"),
    ("L", ".-.."),
    ("M", "--"),
    ("N", "-."),
    ("O", "---"),
    ("P", ".--."),
    ("Q", "--.-"),
    ("R", ".-."),
    ("S", "..."),
    ("T", "-"),
    ("U", "..-"),
    ("V", "...-"),
    ("W", ".--"),
    ("X", "-..-"),
    ("Y", "-.--"),
    ("Z", "--.."),
    ("1", ".----"),
    ("2", "..---"),
    ("3", "...--"),
    ("4", "....-"),
    ("5", "....."),
    ("6", "-...."),
    ("7", "--..."),
    ("8", "---.."),
    ("9", "----."),
    ("0", "-----")
];

/* 
pub fn word_to_morse_code(word: &str) -> Option<String> {
    let mut morse_code = String::new();
    for character in word.chars() {
        if let Some(code) = get_morse_code(character) {
            morse_code.push_str(code);
            morse_code.push(' '); // Add space between letters
        } else {
            return None; // Return None if any character is not found
        }
    }
    Some(morse_code.trim_end().to_string()) // Remove trailing space
}*/



pub fn get_morse_code(character: char) -> Option<&'static str> {
      let upper_char = character.to_ascii_uppercase();
      for &(letter, code) in MORSE_CODES.iter() {
          if letter.chars().next().unwrap() == upper_char {
              return Some(code);
          }
      }
      None
}
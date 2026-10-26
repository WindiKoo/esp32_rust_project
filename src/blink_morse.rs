use esp_hal::{
    delay::Delay,
    gpio::{Level, Output},
};
use esp_println::println;
use crate::morse_codes;

// Timing values (milliseconds)
const DOT_MS: u32 = 300;                        // Dot on duration (1 unit)
const DASH_MS: u32 = DOT_MS * 3;                // Dash on duration (3 units = 1800 ms)
const INTRA_CHAR_GAP_MS: u32 = DOT_MS;          // Off duration between dots/dashes (1 unit = 600 ms)
const LETTER_GAP_MS: u32 = DOT_MS * 3;          // Off duration between letters (3 units = 1800 ms)
const WORD_GAP_MS: u32 = DOT_MS * 7;            // Off duration between words (7 units = 4200 ms)

fn blink_symbol(symbol: char, led: &mut Output, delay: &Delay) {
  match symbol {
    '.' => {
      println!("  [.] DOT -> ON");
      led.set_level(Level::High);
      delay.delay_millis(DOT_MS);
    }
    '-' => {
      println!("  [-] DASH -> ON");
      led.set_level(Level::High);
      delay.delay_millis(DASH_MS);
    }
    _ => return,
  }

  // Turn the LED off after emitting a symbol
  led.set_level(Level::Low);
  delay.delay_millis(INTRA_CHAR_GAP_MS);
}

pub fn blink_text(text: &str, led: &mut Output, delay: &Delay) {
  for character in text.chars() {
    if character == ' ' {
      println!("--- WORD GAP ---");
      led.set_level(Level::Low);
      delay.delay_millis(WORD_GAP_MS);
      continue;
    }

    if let Some(code) = morse_codes::get_morse_code(character) {
      println!("Letter '{}' ({})", character, code);

      for symbol in code.chars() {
        blink_symbol(symbol, led, delay);
      }

      // Note: blink_symbol already waited INTRA_CHAR_GAP_MS (1 unit) with LED OFF.
      // A letter gap requires 3 units total OFF, so we wait the remaining 2 units.
      delay.delay_millis(LETTER_GAP_MS - INTRA_CHAR_GAP_MS);
    }
  }
}
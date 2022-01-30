/*
 * MIT LICENSE
 *
 * Copyright (c) 2022 Randoooom
 *
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct PasswordGenerator {
  numbers: bool,
  letters: bool,
  symbols: bool,
  length: usize,
}

impl Default for PasswordGenerator {
  fn default() -> Self {
    Self {
      numbers: true,
      letters: true,
      symbols: true,
      length: 32,
    }
  }
}

impl PasswordGenerator {
  pub fn generate(&self, generator: Option<Self>) -> String {
    let generator = match generator {
      Some(generator) => generator,
      None => self.clone()
    };

    // setup dataset
    let mut dataset = String::from("");
    // add letters
    if generator.letters {
      dataset.push_str("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ")
    }
    // add numbers
    if generator.numbers {
      dataset.push_str("0123456789")
    }
    // add symbols
    if generator.symbols {
      dataset.push_str("!@#$%^&*()-_+/")
    }

    // parse to char vec
    let dataset = dataset.chars().collect::<Vec<char>>();

    if dataset.len() == 0 {
      return String::from("");
    }

    // generate random numbers
    let numbers = (0..generator.length).map(|_| { rand::random::<u8>() }).collect::<Vec<u8>>();

    // iter through the numbers and add push the matching char into the password
    let password = numbers.iter().map(|number| { dataset[*number as usize % &dataset.len()] }).collect::<String>();

    password
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_default() {
    let generator = PasswordGenerator::default();
    assert_eq!(generator.symbols, true);
    assert_eq!(generator.letters, true);
    assert_eq!(generator.numbers, true);
    assert_eq!(generator.length, 32);
  }

  #[test]
  fn test_default_set() {
    let generator = PasswordGenerator::default();
    let set = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ!@#$%^&*()-_+/0123456789"
      .chars()
      .collect::<Vec<char>>();

    let password: String = generator.generate(None);

    password.chars().into_iter().for_each(|char| {
      assert!(set.contains(&char))
    });
  }

  #[test]
  fn test_numbers() {
    let generator = PasswordGenerator {
      length: 32,
      numbers: true,
      symbols: false,
      letters: false,
    };
    let set = "0123456789"
      .chars()
      .collect::<Vec<char>>();

    let password: String = generator.generate(None);

    password.chars().into_iter().for_each(|char| {
      assert!(set.contains(&char))
    });
  }

  #[test]
  fn test_letters() {
    let generator = PasswordGenerator {
      length: 32,
      numbers: false,
      symbols: false,
      letters: true,
    };
    let set = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
      .chars()
      .collect::<Vec<char>>();

    let password: String = generator.generate(None);

    password.chars().into_iter().for_each(|char| {
      assert!(set.contains(&char))
    });
  }

  #[test]
  fn test_symbols() {
    let generator = PasswordGenerator {
      length: 32,
      numbers: false,
      symbols: true,
      letters: false,
    };
    let set = "!@#$%^&*()-_+/"
      .chars()
      .collect::<Vec<char>>();

    let password: String = generator.generate(None);

    password.chars().into_iter().for_each(|char| {
      assert!(set.contains(&char))
    });
  }

  #[test]
  fn test_length() {
    let generator = PasswordGenerator {
      length: 26,
      numbers: true,
      symbols: true,
      letters: true,
    };

    let password: String = generator.generate(None);
    assert_eq!(26, password.len());
  }

  #[test]
  fn test_custom() {
    let generator = PasswordGenerator::default();
    let password = generator.generate(Some(PasswordGenerator {
      length: 24,
      symbols: true,
      letters: true,
      numbers: true
    }));

    assert_eq!(24, password.len());
  }
}


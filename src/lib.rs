//! playfair-rs: [Playfair cipher] implemented in Rust.
//!
//! > The Playfair cipher or Playfair square or Wheatstone-Playfair cipher is a manual symmetric encryption
//! > technique and was the first literal digram substitution cipher. The scheme was invented in 1854
//! > by Charles Wheatstone but bore the name of Lord Playfair for promoting its use.
//!
//! # Examples
//!
//! ## Encrypt
//!
//! ```
//! let encrypted = playfair_rs::encrypt("playfair example", "hide the gold in the tree stump", 'x').unwrap();
//! println!("{encrypted}"); // bmodzbxdnabekudmuixmmouvif
//! ```
//!
//! ## Decrypt
//!
//! ```
//! let decrypted = playfair_rs::decrypt("playfair example", "bmodzbxdnabekudmuixmmouvif").unwrap();
//! println!("{decrypted}"); // hidethegoldinthetrexestump
//! ```
//! [Playfair cipher]: <https://en.wikipedia.org/wiki/Playfair_cipher>

/// Generates a 5x5 key square from the given keyword.
///
/// Returned key is:
/// * 25 characters long.
/// * Contains valid ASCII characters.
/// * Contains no repeated characters.
fn generate_key(keyword: &str) -> Vec<u8> {
    let mut key = Vec::<u8>::with_capacity(25);

    // Inline function for adding a new character to the key.
    let update_key = |mut c: char, key: &mut Vec<u8>| {
        // Replace 'j' with 'i' to fit 5x5 square.
        if c == 'j' {
            c = 'i';
        }

        // Add a new character to the key.
        // Each character should be unique in the key.
        if !key.contains(&(c as u8)) {
            key.push(c as u8);
        }
    };

    // Iterate the characters in the keyword and update the key.
    keyword
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphabetic())
        .for_each(|c| {
            update_key(c, &mut key);
        });

    // Append the alphabet characters to fill the rest of the key.
    ('a'..='z').for_each(|c| update_key(c, &mut key));

    key
}

/// Enciphers the given plaintext using the Playfair cipher and returns the ciphertext.
///
/// # Example
///
/// ```
/// playfair_rs::encrypt("playfair example", "hide the gold in the tree stump", 'x');
/// ```
pub fn encrypt(keyword: &str, plaintext: &str, pad: char) -> Option<String> {
    // Generate a key.
    let key = generate_key(keyword);

    // Convert plaintext to lowercase and replace 'j' with 'i'.
    // Removes non-ASCII characters.
    let mut plaintext: Vec<u8> = plaintext
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphabetic())
        .map(|mut c| {
            if c == 'j' {
                c = 'i'
            }
            c as u8
        })
        .collect();

    // Loop over the characters 2 at a time and check for duplicates.
    (0..plaintext.len()).step_by(2).for_each(|i| {
        if plaintext.get(i + 1) == Some(&plaintext[i]) {
            // Insert `pad` to separate the duplicates.
            plaintext.insert(i + 1, pad as u8);
        }
    });

    // Append a padding at the end if we have an odd length.
    if plaintext.len() % 2 != 0 {
        plaintext.push(pad as u8);
    }

    // Iterate through the pairs and encipher.
    let mut ciphertext = Vec::new();
    for i in (0..plaintext.len()).step_by(2) {
        // Get the positions of the characters.
        // Needed for performing the operations on swapping or incrementing x and y values.
        let yx1 = key.iter().position(|&c| c == plaintext[i])?;
        let yx2 = key.iter().position(|&c| c == plaintext[i + 1])?;
        let (y1, x1) = (yx1 / 5, yx1 % 5);
        let (y2, x2) = (yx2 / 5, yx2 % 5);

        if y1 != y2 && x1 != x2 {
            // They are in different rows and columns.
            // We swap the x values and keep the same y values.
            ciphertext.push(key[y1 * 5 + x2]);
            ciphertext.push(key[y2 * 5 + x1]);
        } else if y1 == y2 {
            // They are in the same row.
            // We increment the x values by 1.
            ciphertext.push(key[y1 * 5 + (x1 + 1) % 5]);
            ciphertext.push(key[y2 * 5 + (x2 + 1) % 5]);
        } else if x1 == x2 {
            // They are in the same column.
            // We increment the y values by 1.
            ciphertext.push(key[(y1 + 1) % 5 * 5 + x1]);
            ciphertext.push(key[(y2 + 1) % 5 * 5 + x2]);
        }
    }

    String::from_utf8(ciphertext).ok()
}

/// Deciphers the given ciphertext using the Playfair cipher and returns the plaintext.
///
/// # Example
///
/// ```
/// playfair_rs::decrypt("playfair example", "bmodzbxdnabekudmuixmmouvif").unwrap();
/// ```
pub fn decrypt(keyword: &str, ciphertext: &str) -> Option<String> {
    // Ciphertext must have an even number of characters.
    if ciphertext.len() % 2 != 0 {
        return None;
    }

    // Convert ciphertext to lowercase and remove non-ASCII characters.
    let ciphertext = ciphertext
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphabetic())
        .map(|c| c as u8)
        .collect::<Vec<u8>>();

    // Generate the key.
    let key = generate_key(keyword);

    // Iterate through the pairs and decipher.
    let mut plaintext = Vec::new();
    for i in (0..ciphertext.len()).step_by(2) {
        // Get the positions of the characters.
        // Needed for performing the operations on swapping or decrementing x and y values.
        let yx1 = key.iter().position(|&c| c == ciphertext[i])?;
        let yx2 = key.iter().position(|&c| c == ciphertext[i + 1])?;
        let (y1, x1) = (yx1 / 5, yx1 % 5);
        let (y2, x2) = (yx2 / 5, yx2 % 5);

        if y1 != y2 && x1 != x2 {
            // They are in different rows and columns.
            // We swap the x values and keep the same y values.
            plaintext.push(key[y1 * 5 + x2]);
            plaintext.push(key[y2 * 5 + x1]);
        } else if y1 == y2 {
            // They are in the same row.
            // We decrement the x values by 1.
            plaintext.push(key[y1 * 5 + (x1 + 5 - 1) % 5]);
            plaintext.push(key[y2 * 5 + (x2 + 5 - 1) % 5]);
        } else if x1 == x2 {
            // They are in the same column.
            // We decrement the y values by 1.
            plaintext.push(key[(y1 + 5 - 1) % 5 * 5 + x1]);
            plaintext.push(key[(y2 + 5 - 1) % 5 * 5 + x2]);
        }
    }

    String::from_utf8(plaintext).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_key() {
        for (key, keyword) in &[
            ("abcdefghiklmnopqrstuvwxyz", "abcdefg"),
            ("abcdefghiklmnopqrstuvwxyz", "aabbccddee"),
            ("playfirexmbcdghknoqstuvwz", "playfire example"),
            ("abcdefghiklmnopqrstuvwxyz", "aabbccddee"),
            ("playfirexmbcdghknoqstuvwz", "play!!!fa123ir ex^&*ample"),
            ("iabcdefghklmnopqrstuvwxyz", "iiiiiiiiiiiiiiiiiiiiiiiiiiiii"),
        ] {
            assert_eq!(25, key.len());
            assert_eq!(key.as_bytes(), generate_key(keyword));
        }
    }

    #[test]
    fn test_encrypt() {
        for (keyword, plaintext, encrypted) in &[
            (
                "playfair test",
                "never gonna give you up",
                "otxrecqowtbexrexvzpi",
            ),
            ("test", "testing example", "esaekidayskrks"),
            (
                "playfair example",
                "hide the gold in the tree stump",
                "bmodzbxdnabekudmuixmmouvif",
            ),
            ("secretj", "rust is awesomej", "tqesgiheceuhsa"),
            ("1t2Q4GOrzPE", "mgk", "wenu"),
        ] {
            assert_eq!(*encrypted, encrypt(keyword, plaintext, 'x').unwrap());
        }
    }

    #[test]
    fn test_decrypt() {
        for (keyword, plaintext, encrypted) in &[
            (
                "playfair test",
                "nevergonnagiveyouxup",
                "otxrecqowtbexrexvzpi",
            ),
            ("test", "testingexample", "esaekidayskrks"),
            (
                "playfair example",
                "hidethegoldinthetrexestump",
                "bmodzbxdnabekudmuixmmouvif",
            ),
            ("secretj", "rustisawesomei", "tqesgiheceuhsa"),
            ("1t2Q4GOrzPE", "mgkx", "wenu"),
        ] {
            assert_eq!(*plaintext, decrypt(keyword, encrypted).unwrap());
        }
        assert!(decrypt("playfair", "oddnumberofchar").is_none());
    }
}

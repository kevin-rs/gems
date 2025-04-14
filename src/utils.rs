use anyhow::Result;
use base64::{engine::general_purpose::STANDARD, Engine as _};
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::thread;
use std::time::Duration;

/// Extracts text value from a partial JSON string.
///
/// # Arguments
///
/// * `partial_json` - A partial JSON string containing the desired text.
///
/// # Returns
///
/// An `Option<String>` containing the extracted text or `None` if not found.
///
/// # Examples
///
/// ```
/// use gems::utils::extract_text_from_partial_json;
///
/// let partial_json = r#"{"text": "sample_text""#;
/// let extracted_text = extract_text_from_partial_json(partial_json);
/// assert_eq!(extracted_text, Some("sample_text".to_owned()));
/// ```
pub fn extract_text_from_partial_json(partial_json: &str) -> Option<String> {
    if let Some(start_index) = partial_json.find("\"text\": \"") {
        if let Some(end_index) = partial_json[start_index + "\"text\": \"".len()..].find('\"') {
            let text_value = &partial_json[start_index + "\"text\": \"".len()
                ..start_index + "\"text\": \"".len() + end_index];
            return Some(text_value.to_owned());
        }
    }
    None
}

/// Types the given text with a cursor effect, printing each character with a delay.
///
/// # Arguments
///
/// * `text` - The text to be typed with the cursor effect.
/// * `delay` - The delay in milliseconds between printing each character.
///
/// # Examples
///
/// ```
/// use gems::utils::type_with_cursor_effect;
///
/// type_with_cursor_effect("Hello, World!", 50);
/// ```
pub fn type_with_cursor_effect(text: &str, delay: u64) {
    for char in text.chars() {
        print!("{}", char);
        std::io::stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(delay));
    }
}

/// Function to load and encode an image to base64.
///
/// This function takes a file path as input, reads the corresponding image file,
/// and encodes its data to base64 format. The resulting base64-encoded string is then
/// returned as a `Result`. If the operation is successful, the encoded string is wrapped
/// in an `Ok` variant; otherwise, an error is returned with information about the failure.
///
/// # Arguments
///
/// * `file_path` - The path to the image file to be loaded and encoded.
///
/// # Returns
///
/// A `Result` containing the base64-encoded string if successful, or an error if the
/// operation fails.
///
/// # Examples
///
/// ```
/// use gems::utils::load_and_encode_image;
///
/// match load_and_encode_image("/path/to/image.jpg") {
///     Ok(base64_string) => {
///         println!("Image successfully encoded: {}", base64_string);
///     }
///     Err(err) => {
///         eprintln!("Error loading and encoding image: {}", err);
///     }
/// }
/// ```
pub fn load_and_encode_image(file_path: &str) -> Result<String> {
    // Read the image file
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let base64_encoded = STANDARD.encode(&buffer);
    Ok(base64_encoded)
}

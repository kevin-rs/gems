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

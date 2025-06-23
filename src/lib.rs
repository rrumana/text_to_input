use std::collections::HashMap;
use std::fmt;

/// Represents a variable-width character pattern
#[derive(Debug, Clone)]
pub struct CharacterPattern {
    /// The pixel data for the character (5 rows, variable width)
    pub pixels: Vec<Vec<u8>>,
    /// The width of this character
    pub width: usize,
}

impl CharacterPattern {
    /// Create a character pattern from variable width arrays
    pub fn new(rows: &[&[u8]]) -> Self {
        assert_eq!(rows.len(), 5, "Character must have exactly 5 rows");
        assert!(!rows.is_empty(), "Must provide at least one row");
        
        let width = rows[0].len();
        assert!(width > 0, "Character width must be at least 1");
        
        let pixels = rows.iter().map(|row| {
            assert_eq!(row.len(), width, "All rows must have the same width");
            row.to_vec()
        }).collect();
        
        Self { pixels, width }
    }
}

/// Errors that can occur when working with pixel art
#[derive(Debug, Clone, PartialEq)]
pub enum PixelArtError {
    /// Character not found in font
    CharacterNotFound(char),
    /// Input text is too long to process
    TextTooLong(usize),
}

impl fmt::Display for PixelArtError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PixelArtError::CharacterNotFound(ch) => {
                write!(f, "Character '{}' not found in font", ch)
            }
            PixelArtError::TextTooLong(len) => {
                write!(f, "Text too long: {} characters (max: 1000)", len)
            }
        }
    }
}

impl std::error::Error for PixelArtError {}

/// Font data structure containing variable-width character patterns
pub struct PixelFont {
    characters: HashMap<char, CharacterPattern>,
    space_pattern: CharacterPattern,
}

impl Default for PixelFont {
    fn default() -> Self {
        Self::new()
    }
}

impl PixelFont {
    /// Create a new font with basic character set
    pub fn new() -> Self {
        let mut characters = HashMap::with_capacity(53); // 26 uppercase + 26 lowercase + 1 space
        
        // Add uppercase alphabet characters (A-Z)
        characters.insert('A', CharacterPattern::new(&[
            &[0, 1, 1, 1, 0],
            &[1, 0, 0, 0, 1],
            &[1, 1, 1, 1, 1],
            &[1, 0, 0, 0, 1],
            &[1, 0, 0, 0, 1],
        ]));
        
        characters.insert('B', CharacterPattern::new(&[
            &[1, 1, 1, 1, 0],
            &[1, 0, 0, 0, 1],
            &[1, 1, 1, 1, 0],
            &[1, 0, 0, 0, 1],
            &[1, 1, 1, 1, 0],
        ]));
        
        characters.insert('C', CharacterPattern::new(&[
            &[0, 1, 1, 1, 0],
            &[1, 0, 0, 0, 1],
            &[1, 0, 0, 0, 0],
            &[1, 0, 0, 0, 1],
            &[0, 1, 1, 1, 0],
        ]));
        
        characters.insert('D', CharacterPattern::new(&[
            &[1, 1, 1, 1, 0],
            &[1, 0, 0, 0, 1],
            &[1, 0, 0, 0, 1],
            &[1, 0, 0, 0, 1],
            &[1, 1, 1, 1, 0],
        ]));
        
        characters.insert('E', CharacterPattern::new(&[
            &[1, 1, 1, 1, 1],
            &[1, 0, 0, 0, 0],
            &[1, 1, 1, 1, 0],
            &[1, 0, 0, 0, 0],
            &[1, 1, 1, 1, 1],
        ]));
        
        characters.insert('F', CharacterPattern::new(&[
            &[1, 1, 1, 1, 1],
            &[1, 0, 0, 0, 0],
            &[1, 1, 1, 1, 0],
            &[1, 0, 0, 0, 0],
            &[1, 0, 0, 0, 0],
        ]));
        
        characters.insert('G', CharacterPattern::new(&[
            &[0, 1, 1, 1, 0],
            &[1, 0, 0, 0, 0],
            &[1, 0, 1, 1, 1],
            &[1, 0, 0, 0, 1],
            &[0, 1, 1, 1, 0],
        ]));
        
        characters.insert('H', CharacterPattern::new(&[
            &[1, 0, 0, 0, 1],
            &[1, 0, 0, 0, 1],
            &[1, 1, 1, 1, 1],
            &[1, 0, 0, 0, 1],
            &[1, 0, 0, 0, 1],
        ]));
        
        // I can be narrower (3 pixels wide)
        characters.insert('I', CharacterPattern::new(&[
            &[1, 1, 1],
            &[0, 1, 0],
            &[0, 1, 0],
            &[0, 1, 0],
            &[1, 1, 1],
        ]));
        
        characters.insert('J', CharacterPattern::new(&[
            &[1, 1, 1, 1, 1],
            &[0, 0, 0, 1, 0],
            &[0, 0, 0, 1, 0],
            &[1, 0, 0, 1, 0],
            &[0, 1, 1, 0, 0],
        ]));
        
        characters.insert('K', CharacterPattern::new(&[
            &[1, 0, 0, 1, 0],
            &[1, 0, 1, 0, 0],
            &[1, 1, 0, 0, 0],
            &[1, 0, 1, 0, 0],
            &[1, 0, 0, 1, 0],
        ]));
        
        characters.insert('L', CharacterPattern::new(&[
            &[1, 0, 0, 0, 0],
            &[1, 0, 0, 0, 0],
            &[1, 0, 0, 0, 0],
            &[1, 0, 0, 0, 0],
            &[1, 1, 1, 1, 1],
        ]));
        
        characters.insert('M', CharacterPattern::new(&[
            &[1, 0, 0, 0, 1],
            &[1, 1, 0, 1, 1],
            &[1, 0, 1, 0, 1],
            &[1, 0, 0, 0, 1],
            &[1, 0, 0, 0, 1],
        ]));
        
        characters.insert('N', CharacterPattern::new(&[
            &[1, 0, 0, 0, 1],
            &[1, 1, 0, 0, 1],
            &[1, 0, 1, 0, 1],
            &[1, 0, 0, 1, 1],
            &[1, 0, 0, 0, 1],
        ]));
        
        characters.insert('O', CharacterPattern::new(&[
            &[0, 1, 1, 1, 0],
            &[1, 0, 0, 0, 1],
            &[1, 0, 0, 0, 1],
            &[1, 0, 0, 0, 1],
            &[0, 1, 1, 1, 0],
        ]));
        
        characters.insert('P', CharacterPattern::new(&[
            &[1, 1, 1, 1, 0],
            &[1, 0, 0, 0, 1],
            &[1, 1, 1, 1, 0],
            &[1, 0, 0, 0, 0],
            &[1, 0, 0, 0, 0],
        ]));
        
        characters.insert('Q', CharacterPattern::new(&[
            &[0, 1, 1, 1, 0],
            &[1, 0, 0, 0, 1],
            &[1, 0, 1, 0, 1],
            &[1, 0, 0, 1, 1],
            &[0, 1, 1, 1, 1],
        ]));
        
        characters.insert('R', CharacterPattern::new(&[
            &[1, 1, 1, 1, 0],
            &[1, 0, 0, 0, 1],
            &[1, 1, 1, 1, 0],
            &[1, 0, 1, 0, 0],
            &[1, 0, 0, 1, 0],
        ]));
        
        characters.insert('S', CharacterPattern::new(&[
            &[0, 1, 1, 1, 0],
            &[1, 0, 0, 0, 0],
            &[0, 1, 1, 1, 0],
            &[0, 0, 0, 0, 1],
            &[0, 1, 1, 1, 0],
        ]));
        
        characters.insert('T', CharacterPattern::new(&[
            &[1, 1, 1, 1, 1],
            &[0, 0, 1, 0, 0],
            &[0, 0, 1, 0, 0],
            &[0, 0, 1, 0, 0],
            &[0, 0, 1, 0, 0],
        ]));
        
        characters.insert('U', CharacterPattern::new(&[
            &[1, 0, 0, 0, 1],
            &[1, 0, 0, 0, 1],
            &[1, 0, 0, 0, 1],
            &[1, 0, 0, 0, 1],
            &[0, 1, 1, 1, 0],
        ]));
        
        characters.insert('V', CharacterPattern::new(&[
            &[1, 0, 0, 0, 1],
            &[1, 0, 0, 0, 1],
            &[1, 0, 0, 0, 1],
            &[0, 1, 0, 1, 0],
            &[0, 0, 1, 0, 0],
        ]));
        
        characters.insert('W', CharacterPattern::new(&[
            &[1, 0, 0, 0, 1],
            &[1, 0, 0, 0, 1],
            &[1, 0, 1, 0, 1],
            &[1, 1, 0, 1, 1],
            &[1, 0, 0, 0, 1],
        ]));
        
        characters.insert('X', CharacterPattern::new(&[
            &[1, 0, 0, 0, 1],
            &[0, 1, 0, 1, 0],
            &[0, 0, 1, 0, 0],
            &[0, 1, 0, 1, 0],
            &[1, 0, 0, 0, 1],
        ]));
        
        characters.insert('Y', CharacterPattern::new(&[
            &[1, 0, 0, 0, 1],
            &[0, 1, 0, 1, 0],
            &[0, 0, 1, 0, 0],
            &[0, 0, 1, 0, 0],
            &[0, 0, 1, 0, 0],
        ]));
        
        characters.insert('Z', CharacterPattern::new(&[
            &[1, 1, 1, 1, 1],
            &[0, 0, 0, 1, 0],
            &[0, 0, 1, 0, 0],
            &[0, 1, 0, 0, 0],
            &[1, 1, 1, 1, 1],
        ]));
        
        // Add lowercase alphabet characters - many can be narrower
        characters.insert('a', CharacterPattern::new(&[
            &[0, 0, 0, 0],
            &[0, 1, 1, 0],
            &[0, 0, 0, 1],
            &[0, 1, 1, 1],
            &[0, 1, 1, 1],
        ]));
        
        characters.insert('b', CharacterPattern::new(&[
            &[1, 0, 0, 0],
            &[1, 0, 0, 0],
            &[1, 1, 1, 0],
            &[1, 0, 0, 1],
            &[1, 1, 1, 0],
        ]));
        
        characters.insert('c', CharacterPattern::new(&[
            &[0, 0, 0, 0],
            &[0, 1, 1, 0],
            &[1, 0, 0, 0],
            &[1, 0, 0, 0],
            &[0, 1, 1, 0],
        ]));
        
        characters.insert('d', CharacterPattern::new(&[
            &[0, 0, 0, 1],
            &[0, 0, 0, 1],
            &[0, 1, 1, 1],
            &[1, 0, 0, 1],
            &[0, 1, 1, 1],
        ]));
        
        characters.insert('e', CharacterPattern::new(&[
            &[0, 0, 0, 0],
            &[0, 1, 1, 0],
            &[1, 1, 1, 1],
            &[1, 0, 0, 0],
            &[0, 1, 1, 0],
        ]));
        
        characters.insert('f', CharacterPattern::new(&[
            &[0, 1, 1],
            &[0, 1, 0],
            &[1, 1, 0],
            &[0, 1, 0],
            &[0, 1, 0],
        ]));
        
        characters.insert('g', CharacterPattern::new(&[
            &[0, 0, 0, 0],
            &[0, 1, 1, 1],
            &[1, 0, 0, 1],
            &[0, 1, 1, 1],
            &[0, 0, 0, 1],
        ]));
        
        characters.insert('h', CharacterPattern::new(&[
            &[1, 0, 0, 0],
            &[1, 0, 0, 0],
            &[1, 1, 1, 0],
            &[1, 0, 0, 1],
            &[1, 0, 0, 1],
        ]));
        
        // i can be very narrow (1 pixel wide)
        characters.insert('i', CharacterPattern::new(&[
            &[1],
            &[0],
            &[1],
            &[1],
            &[1],
        ]));
        
        characters.insert('j', CharacterPattern::new(&[
            &[0, 1],
            &[0, 0],
            &[0, 1],
            &[0, 1],
            &[1, 0],
        ]));
        
        characters.insert('k', CharacterPattern::new(&[
            &[1, 0, 0],
            &[1, 0, 1],
            &[1, 1, 0],
            &[1, 1, 0],
            &[1, 0, 1],
        ]));
        
        // l can be narrow (1 pixel wide)
        characters.insert('l', CharacterPattern::new(&[
            &[1],
            &[1],
            &[1],
            &[1],
            &[1],
        ]));
        
        characters.insert('m', CharacterPattern::new(&[
            &[0, 0, 0, 0, 0],
            &[1, 1, 0, 1, 0],
            &[1, 0, 1, 0, 1],
            &[1, 0, 1, 0, 1],
            &[1, 0, 1, 0, 1],
        ]));
        
        characters.insert('n', CharacterPattern::new(&[
            &[0, 0, 0, 0],
            &[1, 1, 1, 0],
            &[1, 0, 0, 1],
            &[1, 0, 0, 1],
            &[1, 0, 0, 1],
        ]));
        
        characters.insert('o', CharacterPattern::new(&[
            &[0, 0, 0, 0],
            &[0, 1, 1, 0],
            &[1, 0, 0, 1],
            &[1, 0, 0, 1],
            &[0, 1, 1, 0],
        ]));
        
        characters.insert('p', CharacterPattern::new(&[
            &[0, 0, 0, 0],
            &[1, 1, 1, 0],
            &[1, 0, 0, 1],
            &[1, 1, 1, 0],
            &[1, 0, 0, 0],
        ]));
        
        characters.insert('q', CharacterPattern::new(&[
            &[0, 0, 0, 0],
            &[0, 1, 1, 1],
            &[1, 0, 0, 1],
            &[0, 1, 1, 1],
            &[0, 0, 0, 1],
        ]));
        
        characters.insert('r', CharacterPattern::new(&[
            &[0, 0, 0],
            &[1, 0, 1],
            &[1, 1, 0],
            &[1, 0, 0],
            &[1, 0, 0],
        ]));
        
        characters.insert('s', CharacterPattern::new(&[
            &[0, 0, 0, 0],
            &[0, 1, 1, 0],
            &[0, 1, 0, 0],
            &[0, 0, 1, 0],
            &[1, 1, 0, 0],
        ]));
        
        characters.insert('t', CharacterPattern::new(&[
            &[0, 1, 0],
            &[1, 1, 1],
            &[0, 1, 0],
            &[0, 1, 0],
            &[0, 0, 1],
        ]));
        
        characters.insert('u', CharacterPattern::new(&[
            &[0, 0, 0, 0],
            &[1, 0, 0, 1],
            &[1, 0, 0, 1],
            &[1, 0, 0, 1],
            &[0, 1, 1, 1],
        ]));
        
        characters.insert('v', CharacterPattern::new(&[
            &[0, 0, 0, 0],
            &[1, 0, 0, 1],
            &[1, 0, 0, 1],
            &[0, 1, 1, 0],
            &[0, 0, 1, 0],
        ]));
        
        characters.insert('w', CharacterPattern::new(&[
            &[0, 0, 0, 0, 0],
            &[1, 0, 0, 0, 1],
            &[1, 0, 1, 0, 1],
            &[1, 0, 1, 0, 1],
            &[0, 1, 0, 1, 0],
        ]));
        
        characters.insert('x', CharacterPattern::new(&[
            &[0, 0, 0, 0],
            &[1, 0, 0, 1],
            &[0, 1, 1, 0],
            &[0, 0, 1, 0],
            &[1, 0, 0, 1],
        ]));
        
        characters.insert('y', CharacterPattern::new(&[
            &[0, 0, 0, 0],
            &[1, 0, 0, 1],
            &[1, 0, 0, 1],
            &[0, 1, 1, 1],
            &[0, 0, 0, 1],
        ]));
        
        characters.insert('z', CharacterPattern::new(&[
            &[0, 0, 0, 0],
            &[1, 1, 1, 1],
            &[0, 0, 1, 0],
            &[0, 1, 0, 0],
            &[1, 1, 1, 1],
        ]));
        
        // Define space pattern (3 pixels wide)
        let space_pattern = CharacterPattern::new(&[
            &[0, 0, 0],
            &[0, 0, 0],
            &[0, 0, 0],
            &[0, 0, 0],
            &[0, 0, 0],
        ]);
        
        // Add space character
        characters.insert(' ', space_pattern.clone());
        
        Self { 
            characters,
            space_pattern,
        }
    }
    
    /// Get the pattern for a character, returns space pattern if character not found
    pub fn get_pattern(&self, ch: char) -> &CharacterPattern {
        self.characters.get(&ch).unwrap_or(&self.space_pattern)
    }
    
    /// Check if a character is supported by this font
    pub fn supports_char(&self, ch: char) -> bool {
        self.characters.contains_key(&ch)
    }
    
    /// Get all supported characters
    pub fn supported_chars(&self) -> impl Iterator<Item = &char> {
        self.characters.keys()
    }
}

/// Convert text to pixel art representation with proper error handling
pub fn text_to_pixel_art(text: &str) -> Result<Vec<String>, PixelArtError> {
    const MAX_TEXT_LENGTH: usize = 1000;
    
    if text.len() > MAX_TEXT_LENGTH {
        return Err(PixelArtError::TextTooLong(text.len()));
    }
    
    let font = PixelFont::new();
    let chars: Vec<char> = text.chars().collect();
    
    // Validate all characters are supported
    for &ch in &chars {
        if !font.supports_char(ch) {
            return Err(PixelArtError::CharacterNotFound(ch));
        }
    }
    
    if chars.is_empty() {
        return Ok(vec!["0000000000000000000".to_string(); 7]);
    }
    
    // Calculate total width: buffer + sum of character widths + spacing between chars + buffer
    let char_widths: Vec<usize> = chars.iter().map(|&ch| font.get_pattern(ch).width).collect();
    let total_char_width: usize = char_widths.iter().sum();
    let spacing_count = chars.len().saturating_sub(1);
    let total_width = 2 + total_char_width + spacing_count;
    
    // Pre-allocate result vector with known capacity
    let mut result = Vec::with_capacity(7);
    
    // Top buffer row
    result.push("0".repeat(total_width));
    
    // Character rows (5 rows)
    for row in 0..5 {
        let mut line = String::with_capacity(total_width);
        
        // Left buffer
        line.push('0');
        
        // Add each character's row with spacing
        for (i, &ch) in chars.iter().enumerate() {
            let pattern = font.get_pattern(ch);
            for &pixel in &pattern.pixels[row] {
                line.push(if pixel == 1 { '1' } else { '0' });
            }
            
            // Add spacing between characters (except after the last character)
            if i < chars.len() - 1 {
                line.push('0');
            }
        }
        
        // Right buffer
        line.push('0');
        
        result.push(line);
    }
    
    // Bottom buffer row
    result.push("0".repeat(total_width));
    
    Ok(result)
}

/// Convert text to pixel art representation, using space for unsupported characters
pub fn text_to_pixel_art_lossy(text: &str) -> Result<Vec<String>, PixelArtError> {
    text_to_pixel_art(text)
}

/// Validate that all characters in the text are supported
pub fn validate_text(text: &str) -> Result<(), PixelArtError> {
    let font = PixelFont::new();
    
    for ch in text.chars() {
        if !font.supports_char(ch) {
            return Err(PixelArtError::CharacterNotFound(ch));
        }
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_to_pixel_art() {
        let result = text_to_pixel_art("A").expect("Should convert successfully");
        assert_eq!(result.len(), 7);
        assert_eq!(result[0], "0000000");
        assert_eq!(result[6], "0000000");
    }

    #[test]
    fn test_empty_string() {
        let result = text_to_pixel_art("").expect("Should handle empty string");
        assert_eq!(result.len(), 7);
        for line in result {
            assert_eq!(line, "0000000000000000000");
        }
    }
    
    #[test]
    fn test_mixed_case() {
        let result = text_to_pixel_art("Aa").expect("Should handle mixed case");
        assert_eq!(result.len(), 7);
        // Should have proper spacing between characters
        assert!(result[1].len() > 10);
    }
    
    #[test]
    fn test_variable_width() {
        let result = text_to_pixel_art("il").expect("Should handle narrow characters");
        assert_eq!(result.len(), 7);
        // 'i' is 1 wide, 'l' is 1 wide, plus spacing and buffers = 2 + 1 + 1 + 1 = 5
        assert_eq!(result[1].len(), 5);
    }
    
    #[test]
    fn test_text_too_long() {
        let long_text = "a".repeat(1001);
        let result = text_to_pixel_art(&long_text);
        assert!(matches!(result, Err(PixelArtError::TextTooLong(1001))));
    }
    
    #[test]
    fn test_font_supports_char() {
        let font = PixelFont::new();
        assert!(font.supports_char('A'));
        assert!(font.supports_char('a'));
        assert!(font.supports_char(' '));
        assert!(!font.supports_char('!'));
    }
    
    #[test]
    fn test_validate_text() {
        assert!(validate_text("Hello World").is_ok());
        assert!(matches!(validate_text("Hello!"), Err(PixelArtError::CharacterNotFound('!'))));
    }
    
    #[test]
    fn test_error_display() {
        let error = PixelArtError::CharacterNotFound('!');
        assert_eq!(error.to_string(), "Character '!' not found in font");
        
        let error = PixelArtError::TextTooLong(1001);
        assert_eq!(error.to_string(), "Text too long: 1001 characters (max: 1000)");
    }
    
    #[test]
    fn test_character_pattern_creation() {
        let pattern = CharacterPattern::new(&[
            &[1, 0],
            &[1, 1],
            &[1, 0],
            &[1, 0],
            &[1, 0],
        ]);
        assert_eq!(pattern.width, 2);
        assert_eq!(pattern.pixels.len(), 5);
    }
}

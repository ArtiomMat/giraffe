
#[derive(Debug, Clone, PartialEq)]
pub enum ExpressionToken {
    Number(f32),
    Identifier(String),
    OpenBracket,
    CloseBracket,
    Plus,
    Minus,
    Star,
    Slash,
    Caret,
}

/// Symbols(except `_` and `.`) are a category.
/// Alphanumerics(letters, digits, and `.`) are a category.
/// Whitespace is a category.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CharCategory {
    Whitespace,
    Symbol,
    Alphanumeric,
}

// fn get_char_category(c: char) -> usize {
//     if c.is_numeric() 
// }

// /// Determines if c and other are fundamentally distinct for the purpose of
// /// tokenization.
// /// 
// /// With the above categories we can separate tokens from each other.
// fn is_char_categorized_with(c: char, other: char) -> bool {
    
// }

// pub fn tokenize(text: &str) -> Vec<ExpressionToken> {
//     let mut token_string = String::new();
//     let mut tokens = Vec::new();

//     for c in text.chars() {
        
//     }

//     tokens
// }

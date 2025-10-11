#[derive(Debug, Clone, PartialEq)]
pub enum Token {
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

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CharCategory {
    Whitespace,
    /// All symbols, excluding `_`.
    Symbol,
    /// All letters, including `_`.
    Alphabetic,
    /// All digits, excluding `.`, that's a symbol.
    Numeric,

    Unknown,
}

/// bit flags that indicate various properties about the current
/// token string we have. These help us understand precisely what
/// to translate the token string into as a full token.
type TokenTags = u32;

/// It starts with a `0`.
const TOKEN_TAGS_ZERO_PREFIXED: TokenTags = 0b0000_0000_0000_0000_0001;

/// It starts with or some digit.
const TOKEN_TAGS_DIGIT_PREFIXED: TokenTags = 0b0000_0000_0000_0010;

/// It starts with '_' or some letter.
const TOKEN_TAGS_LETTER_PREFIXED: TokenTags = 0b0000_0000_0000_0100;

/// It starts(and should end at that) with a symbol.
const TOKEN_TAGS_SYMBOL: TokenTags = 0b0000_0000_0000_1000;

/// It starts with `0x`.
const TOKEN_TAGS_HEX_PREFIXED: TokenTags = 0b0000_0000_0001_0000;

/// It starts with `0o`.
const TOKEN_TAGS_OCTAL_PREFIXED: TokenTags = 0b0000_0000_0010_0000;

/// It has `.` somewhere already.
const TOKEN_TAGS_HAS_PERIOD: TokenTags = 0b0000_0000_0100_0000;

fn get_char_category(c: char) -> CharCategory {
    if c.is_numeric() {
        CharCategory::Numeric
    } else if c.is_alphabetic() || c == '_' {
        CharCategory::Alphabetic
    } else if c.is_ascii_graphic() {
        CharCategory::Symbol
    } else if c.is_whitespace() {
        CharCategory::Whitespace
    } else {
        CharCategory::Unknown
    }
}

pub fn tokenize(text: &str) -> Vec<Token> {
    let mut token_string = String::new();
    let mut token_tags = 0;
    let mut tokens = Vec::new();

    for c in text.chars() {
        // let current_category = get_char_category(c);

        // // Symbols are never accumulated
        // if last_category == current_category && current_category != CharCategory::Symbol {
        //     token_string.push(c);
        //     continue;
        // }

        // // Special case for '.' if the previous was a number.
        // if c == '.' && last_category == CharCategory::Numeric {
        //     token_string.push(c);
        //     continue;
        // }

        // // Now we either must stop accumulating the token string, or we need some
        // // special logic to determine if we can keep going.

        // if !token_string.is_empty() {

        // }

        // match current_category {

        // }

        match c {
            'A'..='Z' | 'a'..='z' | '_' => {
                // if last_category != CharCategory::Alphabetic {}

                // last_category = CharCategory::Alphabetic;
            }
            // ' ' | '\t' => {
            //     if token_string.len() {

            //     }

            //     token_tags = 0;
            //     token_string.clear();
            // }
            _ => {}
        }
    }

    tokens
}

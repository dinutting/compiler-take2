#![allow(dead_code)]

#[derive(Debug, PartialEq)]
pub enum TokenType {
    Whitespace(usize),
    Constant(isize),
    IntKeyword,
    VoidKeyword,
    ReturnKeyword,
    Identifier(String),
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    Semicolon,
    Empty
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TokenType::Whitespace(_i) => write!(f, "Whitespace"),
            TokenType::Constant(i) => write!(f, "Constant: {}", i),
            TokenType::IntKeyword => write!(f, "IntKeyword"),
            TokenType::VoidKeyword => write!(f, "VoidKeyword"),
            TokenType::ReturnKeyword => write!(f, "ReturnKeyword"),
            TokenType::Identifier(s) => write!(f, "Identifier: {}", s),
            TokenType::OpenParen => write!(f, "OpenParen"),
            TokenType::CloseParen => write!(f, "CloseParen"),
            TokenType::OpenBrace => write!(f, "OpenBrace"),
            TokenType::CloseBrace => write!(f, "CloseBrace"),
            TokenType::Semicolon => write!(f, "Semicolon"),
            TokenType::Empty => write!(f, "Empty"),
            //_ => write!(f, "Weird TokenType Found!"),
        }
    }
}

/// Receives a reference to a string and examines the beginning of it for a valid token.
/// If one is found, it returns the corresponding TokenType
/// If one is not found, it provides a TokenType::Empty
/// 
/// # Example
/// ```
/// let len = source.len();
/// let mut pointer: usize = 0;
/// let mut tokens: Vec<TokenType> = Vec::new();
///
/// while pointer<len {
///     match mad_scan(&source[pointer..len]) {
///          TokenType::Whitespace(x) => {pointer+=x; tokens.push(TokenType::Whitespace(x))},
/// ...
/// ...
/// ```
fn mad_scan(source: &str) -> TokenType {

    // Grab the first character in source. This might be a dumb way of doing this, instead of just grabbing a slice
    // but the iterator does let me use the take_while, which is handy.
    // I would like to explore if there's ways to match on variable length slices
    match source.chars().next() {

        None => TokenType::Empty, // This should trigger no more scanning

        // if the beginning of the string is whitespace, count all the whitespace until a non whitespace character shows up.
        Some(c) if c.is_whitespace() => TokenType::Whitespace(source.chars()
                                                                           .take_while(|c| c.is_whitespace())
                                                                           .count() as usize),
        
        // if it's alphabetic, it might be a keyword, or it might be an identifier. Keywords should take precedent
        Some(c) if c.is_alphabetic() => { 
            match c {
                // this is a weird way of doing this, but it works
                'v' if &source[0..5] == "void " => TokenType::VoidKeyword,
                'i' if &source[0..4] == "int " => TokenType::IntKeyword,
                'r' if &source[0..7] == "return " => TokenType::ReturnKeyword,
                'r' if &source[0..7] == "return;" => TokenType::ReturnKeyword,
                // if it's not a keyword, assume it's an identifier
                // take all the alphanumeric until a non-alphanumeric is received.
                // ***TODO, account for dashes and underscores***
                _   => TokenType::Identifier(source.chars()
                                                            .take_while(|c| c.is_alphanumeric())
                                                            .collect::<String>()),
            }
        },
        // if it starts with a number, assume it's a numeric constant.
        // **TODO account for negative and decimal**
        Some(c) if c.is_numeric() => TokenType::Constant(source.chars()
                                                                       .take_while(|c| c.is_numeric())
                                                                       .collect::<String>()
                                                                       // **** TODO **** Address this unwrap
                                                                       .parse::<isize>().unwrap()
                                                               ),
        // if it isn't numeric, whitespace, or alphabetical, it better be punctuation.
        Some(c) if c.is_ascii_punctuation() => {
            match c {
                '{' => TokenType::OpenBrace,
                '(' => TokenType::OpenParen,
                '}' => TokenType::CloseBrace,
                ')' => TokenType::CloseParen,
                ';' => TokenType::Semicolon,
                _   => TokenType::Empty,
            }
        },
        Some(_) => TokenType::Empty,
    } 
    }

/// Lexes the target string and returns a Vector of TokenType
/// The utility of this is tightly coupled to the parameters attached to the varying TokenTypes.
/// as the lexer needs to know by how many characters it should move the pointer as each token is discovered.
/// Fixed width keywords such as `int` and `void` have fixed length pointer moves.
/// Where as variable length tokens such as whitespace and identifiers calculate how far the pointer should
/// move based on the length of the attached string.
/// Constant is an awkward tokentype, as it should ideally be stored in it's target token type, but has to temporarily
/// be converted to a string to find the length. This could be problematic later, and should probably be changed to a string
/// and then let the parser grab the number.
/// # Example
/// ```
/// let tokens: Vec<TokenType> = lexer("int main() { return 365; }");
/// for token in tokens {
///     println!("{}", token);
/// }
/// ```
fn lexer(source: &str) -> Vec<TokenType> {
    let len = source.len(); // **TODO DEAL WITH UNWRAPS**
    let mut pointer: usize = 0;
    let mut tokens: Vec<TokenType> = Vec::new();

    while pointer<len {
        match mad_scan(&source[pointer..len]) {
            TokenType::Whitespace(x) => {pointer+=x; tokens.push(TokenType::Whitespace(x))},
            TokenType::Identifier(s) => {pointer+=s.len(); tokens.push(TokenType::Identifier(s))},
            TokenType::Constant(i) => {pointer+=i.to_string().len(); tokens.push(TokenType::Constant(i))},
            
            TokenType::IntKeyword => {pointer+=3; tokens.push(TokenType::IntKeyword)},
            TokenType::VoidKeyword => {pointer+=4; tokens.push(TokenType::VoidKeyword)},
            TokenType::ReturnKeyword => {pointer+=6; tokens.push(TokenType::ReturnKeyword)},
            
            TokenType::CloseBrace => {pointer+=1; tokens.push(TokenType::CloseBrace)},
            TokenType::OpenBrace => {pointer+=1; tokens.push(TokenType::OpenBrace)},
            
            TokenType::CloseParen => {pointer+=1; tokens.push(TokenType::CloseParen)},
            TokenType::OpenParen => {pointer+=1; tokens.push(TokenType::OpenParen)},
            
            TokenType::Semicolon => {pointer+=1; tokens.push(TokenType::Semicolon)},

            TokenType::Empty => break,
            //_ => break,
        }
    }
    tokens
}

#[test]
fn test_lexer() {
    let tokens: Vec<TokenType> = lexer("int main() { return 365; }");
    for token in tokens {
        println!("{}", token);
    }
}

fn scan_for_whitespace(source: &str) -> (bool, i64) {

    match source.chars().next() {
        None => panic!("Scanning issue in scan for whitespace, string was empty"),
        // This next line looks at the first character and if it's a whitespace, it takes all subsequent whitespaces
        // until it hits a non whitespace character, counts how many were taken, and then passes that count
        Some(x) if x.is_whitespace() => (true, source.chars().take_while(|x| x.is_whitespace()).count() as i64),
        Some(_) => (false, 0),
    }
}

fn scan_for_void(source: &str) -> bool {
    if source.len() < 5 { false }
    else if &source[0..5]=="void " { true }
    else { false }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mad_scan_number(){
        let source = String::from("365 days");
        assert_eq!(mad_scan(&source), TokenType::Constant(365))
    }

    #[test]
    fn mad_scan_whitespace(){
        let source = String::from("      space tab space"); // note tab entered 4 spaces
        assert_eq!(mad_scan(&source), TokenType::Whitespace(6))
    }

    #[test]
    fn mad_scan_return(){
        let mut source = String::from("return;");
        assert_eq!(mad_scan(&source), TokenType::ReturnKeyword);
        source = String::from("return 32;");
        assert_eq!(mad_scan(&source), TokenType::ReturnKeyword);
    }

    #[test]
    fn mad_scan_identifier(){
        let source = String::from("variable123 = 10");
        assert_eq!(mad_scan(&source), TokenType::Identifier(String::from("variable123")))
    }

    #[test]
    fn start_with_void() {
        let source = String::from("void main");
        assert!(scan_for_void(&source));
    }

    #[test]
    fn does_not_start_with_void() {
        let source = String::from("int main()");
        assert!(!scan_for_void(&source));
    }

    #[test]
    fn does_not_start_with_whitespace() {
        let source = String::from("This is a new string");
        assert_eq!(scan_for_whitespace(&source), (false, 0));
    }

    #[test]
    fn start_with_spaces() {
        let source = String::from("  Two spaces.");
        assert_eq!(scan_for_whitespace(&source),(true, 2));
    }

    #[test]
    fn start_with_tab_and_space() {
        let source = String::from("  Tab and Space.");
        assert_eq!(scan_for_whitespace(&source),(true, 2));
    }
}
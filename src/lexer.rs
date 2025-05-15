#![allow(dead_code)]
/// An enumeration used for tracking the different types of 
/// tokens in the lexer. 
/// - When adding, or changing, the TokenTypes,
/// Make sure to update the generate_tmakers function.
/// 
/// I wanted to approach this as constant values, but the Regex
/// crate doesn't work for const functions. lazy_regex might work.
//#[derive(Copy, Clone, PartialEq)]

//"void main() { return; }"

#[derive(Debug)]
#[derive(PartialEq)]
pub enum TokenTypes {
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

impl std::fmt::Display for TokenTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TokenTypes::Whitespace(_i) => write!(f, "Whitespace"),
            TokenTypes::Constant(i) => write!(f, "Constant: {}", i),
            TokenTypes::IntKeyword => write!(f, "IntKeyword"),
            TokenTypes::VoidKeyword => write!(f, "VoidKeyword"),
            TokenTypes::ReturnKeyword => write!(f, "ReturnKeyword"),
            TokenTypes::Identifier(s) => write!(f, "Identifier: {}", s),
            TokenTypes::OpenParen => write!(f, "OpenParen"),
            TokenTypes::CloseParen => write!(f, "CloseParen"),
            TokenTypes::OpenBrace => write!(f, "OpenBrace"),
            TokenTypes::CloseBrace => write!(f, "CloseBrace"),
            TokenTypes::Semicolon => write!(f, "Semicolon"),
            TokenTypes::Empty => write!(f, "Empty"),
            //_ => write!(f, "Weird TokenType Found!"),
        }
    }
}

fn mad_scan(source: &str) -> TokenTypes {

    // Grab the first character in source. This might be a dumb way of doing this, instead of just grabbing a slice
    // but the iterator does let me use the take_while, which is handy.
    // I would like to explore if there's ways to match on variable length slices
    match source.chars().next() {

        None => TokenTypes::Empty, // This should trigger no more scanning

        // if the beginning of the string is whitespace, count all the whitespace until a non whitespace character shows up.
        Some(c) if c.is_whitespace() => TokenTypes::Whitespace(source.chars()
                                                                           .take_while(|c| c.is_whitespace())
                                                                           .count() as usize),
        
        // if it's alphabetic, it might be a keyword, or it might be an identifier. Keywords should take precedent
        Some(c) if c.is_alphabetic() => { 
            match c {
                // this is a weird way of doing this, but it works
                'v' if &source[0..5] == "void " => TokenTypes::VoidKeyword,
                'i' if &source[0..4] == "int " => TokenTypes::IntKeyword,
                'r' if &source[0..7] == "return " => TokenTypes::ReturnKeyword,
                'r' if &source[0..7] == "return;" => TokenTypes::ReturnKeyword,
                // if it's not a keyword, assume it's an identifier
                // take all the alphanumeric until a non-alphanumeric is received.
                // ***TODO, account for dashes and underscores***
                _   => TokenTypes::Identifier(source.chars()
                                                            .take_while(|c| c.is_alphanumeric())
                                                            .collect::<String>()),
            }
        },
        // if it starts with a number, assume it's a numeric constant.
        // **TODO account for negative and decimal**
        Some(c) if c.is_numeric() => TokenTypes::Constant(source.chars()
                                                                       .take_while(|c| c.is_numeric())
                                                                       .collect::<String>()
                                                                       // **** TODO **** Address this unwrap
                                                                       .parse::<isize>().unwrap()
                                                               ),
        // if it isn't numeric, whitespace, or alphabetical, it better be punctuation.
        Some(c) if c.is_ascii_punctuation() => {
            match c {
                '{' => TokenTypes::OpenBrace,
                '(' => TokenTypes::OpenParen,
                '}' => TokenTypes::CloseBrace,
                ')' => TokenTypes::CloseParen,
                ';' => TokenTypes::Semicolon,
                _   => TokenTypes::Empty,
            }
        },
        Some(_) => TokenTypes::Empty,
    } 
    }

fn lexer(source: &str) -> Vec<TokenTypes> {
    let len = source.len(); // **TODO DEAL WITH UNWRAPS**
    let mut pointer: usize = 0;
    let mut tokens: Vec<TokenTypes> = Vec::new();

    while pointer<len {
        match mad_scan(&source[pointer..len]) {
            TokenTypes::Whitespace(x) => {pointer+=x; tokens.push(TokenTypes::Whitespace(x))},
            TokenTypes::Identifier(s) => {pointer+=s.len(); tokens.push(TokenTypes::Identifier(s))},
            TokenTypes::Constant(i) => {pointer+=i.to_string().len(); tokens.push(TokenTypes::Constant(i))},
            
            TokenTypes::IntKeyword => {pointer+=3; tokens.push(TokenTypes::IntKeyword)},
            TokenTypes::VoidKeyword => {pointer+=4; tokens.push(TokenTypes::VoidKeyword)},
            TokenTypes::ReturnKeyword => {pointer+=6; tokens.push(TokenTypes::ReturnKeyword)},
            
            TokenTypes::CloseBrace => {pointer+=1; tokens.push(TokenTypes::CloseBrace)},
            TokenTypes::OpenBrace => {pointer+=1; tokens.push(TokenTypes::OpenBrace)},
            
            TokenTypes::CloseParen => {pointer+=1; tokens.push(TokenTypes::CloseParen)},
            TokenTypes::OpenParen => {pointer+=1; tokens.push(TokenTypes::OpenParen)},
            
            TokenTypes::Semicolon => {pointer+=1; tokens.push(TokenTypes::Semicolon)},

            TokenTypes::Empty => break,
            //_ => break,
        }
    }
    tokens
}

#[test]
fn test_lexer() {
    let tokens: Vec<TokenTypes> = lexer("int main() { return 365; }");
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
        assert_eq!(mad_scan(&source), TokenTypes::Constant(365))
    }

    #[test]
    fn mad_scan_whitespace(){
        let source = String::from("      space tab space"); // note tab entered 4 spaces
        assert_eq!(mad_scan(&source), TokenTypes::Whitespace(6))
    }

    #[test]
    fn mad_scan_return(){
        let mut source = String::from("return;");
        assert_eq!(mad_scan(&source), TokenTypes::ReturnKeyword);
        source = String::from("return 32;");
        assert_eq!(mad_scan(&source), TokenTypes::ReturnKeyword);
    }

    #[test]
    fn mad_scan_identifier(){
        let source = String::from("variable123 = 10");
        assert_eq!(mad_scan(&source), TokenTypes::Identifier(String::from("variable123")))
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
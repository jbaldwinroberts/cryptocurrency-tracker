use std::fmt::Write;

pub enum TokenType {
    Literal,
    Symbol
}

pub struct Token {
    t_type: TokenType,
    // For a literal, the value is literally the string as found
    // in the input.
    //
    // For a Symbol, this is the piece of data which is being extracted
    // from the symbol. E.g in ETH.price, `value` here would be `price`.
    value: String,
    // Only set in the case of TokenType being a Symbol.
    symbol: String
}

pub fn tokenise_format_str(fmt_str: String) -> Vec<Token> {
    // TODO: Take these from parameters
    let open_symbol = '{';
    let close_symbol = '}';

    let mut tokens: Vec<Token> = vec![];
    let mut temp_str = String::new();

    let mut in_symbol: bool = false;

    for c in fmt_str.chars() {
        match c {
            open_symbol => {
                tokens.push(literal_from_string(temp_str));
                temp_str = String::new();
            },
            close_symbol => {
                tokens.push(symbol_from_string(temp_str));

            }
            _ => {
                write!(&mut temp_str, "{}", c)
            }
        }
    }

    return tokens;
}

fn literal_from_string(literal: String) -> Token {
    return Token {
        t_type: TokenType::Literal,
        value: literal.clone(),
        symbol: "".to_string()
    }
}

fn symbol_from_string(sym: String) -> Token {
    let splits: Vec<String> = sym.split('.');
    return Token {
        t_type: TokenType::Symbol,
        value: splits[1],
        symbol: splits[0]
    }
}

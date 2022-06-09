pub enum Token {
    Operation(String),
    Value(String),
    FunctionDeclaration,
    VariableDeclaration,
    Identifier(String),
    EndOfLine,
}




pub fn stream_to_token(input: String) -> Option<Vec<Token>> {
    /* Takes a stream of chars split by whitespace and iterates through the stream. */
    let stream: Vec<char> = input.chars().collect();
    let mut current_token_value: &str = "";
    let mut token_collection: Vec<Token> = vec![];
    let operation_symbols = [
        "+", "-", "*", "/", "<", ">", "<=", ">=", "=", "==", "&&", "||"];

    for character in stream {
        match character {
            ' ' => 
                if current_token_value == "func" {
                    token_collection.append(Token::FunctionDeclaration);
                    current_token_value = "";
                }else if current_token_value == "var" {
                    token_collection.append(Token::VariableDeclaration);
                    current_token_value = "";
                }else if current_token_value == operation_symbols {
                    token_collection.append(Token::Operation(current_token_value));
                    current_token_value = "";
                }else{
                    let last_token_peak = token_collection.last().unwrap();
                    match last_token_peak {
                        Token::Operation(_) => token_collection.append(Token::Value(current_token_value)),
                        Token::FunctionDeclaration => token_collection.append(Token::Identifier(current_token_value)),
                        Token::VariableDeclaration => token_collection.append(Token::Identifier(current_token_value)),
                    }
                    current_token_value = "";
                }
            'a' ..= 'z' | 'A' ..= 'Z' => current_token_value + character,
            '1' ..= '9' => current_token_value + character,
            _ => None,
        }
    }
    return Some(token_collection);
}
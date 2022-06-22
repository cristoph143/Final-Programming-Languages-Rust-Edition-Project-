struct Dictionary {
    index: i32,
    words: String,
    data_type: String,
}
impl Dictionary {
    fn new(index: i32, words: String, data_type: String) -> Dictionary {
        Dictionary {
            index: index,
            words: words.to_string(),
            data_type: data_type.to_string()
        }
    }
}

fn main() {
    // accept string input from the users
    println!("Enter a string: ");
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    // split the string by "\s+"
    let input_split = input.split_whitespace();
    //TODO: If there are still remaining time, configure the spacing so that it will tokenize correctly
    let dictionary = Vec::new();
    // iterate over the split string
    for word in input_split {
        // call get_data_type function and save the result
        let data_types = get_data_type(word);
        let cnt = 1;
        // save word and data_type to Dictionary
        dictionary = dictionary.push(Dictionary::new(cnt, word.to_string(), data_types.to_string()));
        cnt += 1;
    }
    // loop and print the index, words and data_type
    for i in 0..dictionary.len() {
        println!("{} {} {}", dictionary[i].index, dictionary[i].words, dictionary[i].data_type);
    }



}

// Lexical Analyzer

// return string of data_type from the given str
fn get_data_type(word: &str) -> String {
    // return "keyword" if the string is either equal to "int", "float", "char", "bool", "string" or "double"
    match word {
        "int" | "float" | "char" | "bool" | "string" | "double" => "keyword".to_string(),
        // return "equals_op" if the string is a "="
        "=" => "equals_op".to_string(),
        // return "operator" if the string is a "+", "-" or "*", or "/"
        "+" | "-" | "*" | "/" => "operator".to_string(),
        // return "single quote" if the string is a "\'"
        "\'" => "single_quote".to_string(),
        // return "double quote" if the string is a "\""
        "\"" => "double_quote".to_string(),
        // return "left_paren" if the string is a "("
        "(" => "left_paren".to_string(),
        // return "right_paren" if the string is a ")"
        ")" => "right_paren".to_string(),
        // return "Terminator" if the string is a ";"
        ";" => "Terminator".to_string(),
        // return "Identifier" if the string is [a-zA-Z][a-zA-Z0-9_]*
        _ if word.chars().all(|c| c.is_alphabetic() || c == '_') => "Identifier".to_string(),
        // return "constant" if the string is number
        _ if word.chars().all(|c| c.is_numeric()) => "constant".to_string(),
        // else return "null"
        _ => "null".to_string(),
    }
}

#[test]
fn test_get_data_type() {
    assert_eq!(get_data_type("int"), "keyword");
    assert_eq!(get_data_type("float"), "keyword");
    assert_eq!(get_data_type("char"), "keyword");
    assert_eq!(get_data_type("bool"), "keyword");
    assert_eq!(get_data_type("string"), "keyword");
    assert_eq!(get_data_type("double"), "keyword");
    assert_eq!(get_data_type("="), "equals_op");
    assert_eq!(get_data_type("+"), "operator");
    assert_eq!(get_data_type("-"), "operator");
    assert_eq!(get_data_type("*"), "operator");
    assert_eq!(get_data_type("/"), "operator");
    assert_eq!(get_data_type("\'"), "single_quote");
    assert_eq!(get_data_type("\""), "double_quote");
    assert_eq!(get_data_type("("), "left_paren");
    assert_eq!(get_data_type(")"), "right_paren");
    assert_eq!(get_data_type(";"), "Terminator");
    assert_eq!(get_data_type("_"), "Identifier");
    assert_eq!(get_data_type("_123"), "Identifier");
}

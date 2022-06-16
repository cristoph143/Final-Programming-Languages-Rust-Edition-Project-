use std::collections::HashMap;

struct Dictionary{
    words: String,
    data_type: String
}
impl Dictionary{
    fn new(words: String, data_type: String) -> Dictionary{
        Dictionary{
            words: words.to_string(),
            data_type: data_type.to_string()
        }
    }
}

fn main(){
    // accept string input from the users
    println!("Enter a string: ");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    // split the string by "\s+"
    let input_split = input.split_whitespace();
    //TODO: If there are still remaining time, configure the spacing so that it will tokenize correctly
    // initialize a map
    let mut map = HashMap::new();
    // iterate over the split string
    for word in input_split{
        // call get_data_type function and save the result
        let data_types = get_data_type(word);
        // save word and data_type to Dictionary
        let dictionary = Dictionary::new(word.to_string(), data_types.to_string());
        // save the dictionary to the map with current index
        map.insert(map.len(), dictionary);
    }
    // sort the map by index
    let mut sorted_map: Vec<(&usize, &Dictionary)> = map.iter().collect::<Vec<(&usize, &Dictionary)>>();
    sorted_map.sort_by(|a, b| a.0.cmp(&b.0));
    print_map(&map, sorted_map);
}

fn print_map(map: &HashMap<usize, Dictionary>, sorted_map: Vec<(&usize, &Dictionary)>) {
    // print the map
    print!("------------------------------------------------");
    print!("Unsorted Map!");
    print!("------------------------------------------------\n");
    // print the map index and dictionary word and data_type
    for (index, dictionary) in map.iter(){
        println!("{} {} {}", index, dictionary.words, dictionary.data_type);
    }
    // print the sorted map
    print!("------------------------------------------------");
    print!("Sorted Map!");
    print!("------------------------------------------------\n");
    for (index, dictionary) in sorted_map.iter(){
        println!("{} {} {}", index, dictionary.words, dictionary.data_type);
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
        _ if word.chars().all(|c| c.is_alphanumeric() || c == '_') => "Identifier".to_string(),
        // return "constant" if the string is number
        _ if word.chars().all(|c| c.is_numeric()) => "constant".to_string(),
        // else return "null"
        _ => "null".to_string(),
    }
}


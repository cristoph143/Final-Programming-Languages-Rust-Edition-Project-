use std::collections::HashMap;

struct Dictionary {
    words: String,
    data_type: String,
}
impl Dictionary {
    fn new(words: String, data_type: String) -> Dictionary {
        Dictionary {
            words: words.to_string(),
            data_type: data_type.to_string(),
        }
    }
}

// struct BNF {
//     rules: String,
//     data_type: String,
// }

// impl BNF {
//     fn new(rules: String, data_type: String) -> BNF {
//         BNF {
//             rules: rules.to_string(),
//             data_type: data_type.to_string(),
//         }
//     }
//     // add rule
//     fn add_rule(&mut self, rule: String) {
//         self.rules = self.rules + &rule;
//     }
//     // get_rule
//     fn get_rule(&self, index: usize) -> String {
//         let mut rules = self.rules.split("\n");
//         let mut i = 0;
//         for rule in rules {
//             if i == index {
//                 return rule.to_string();
//             }
//             i += 1;
//         }
//         return "".to_string();
//     }
// }

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
    // initialize a map
    let mut map = HashMap::new();
    let mut data_types;
    // iterate over the split string
    for word in input_split {
        // call get_data_type function and save the result
        data_types = get_data_type(word);
        // if data_types is "null", then split the word until it is not null
        // if data_types == "null" {
        //     let mut temp = word.to_string();
        //     while data_types == "null" {
        //         temp = temp + " " + &word;
        //         data_types = get_data_type(&temp);
        //     }
        //     data_types = data_types.to_string();
        // }
        // save word and data_type to Dictionary
        let dictionary = Dictionary::new(word.to_string(), data_types.to_string());
        // save the dictionary to the map with current index
        map.insert(map.len(), dictionary);
    }

    print_map(&map);
    // FIXME: if length is lesser than 2, exit program
    // check if index value is of map is 0
    if map.get(&0).unwrap().data_type != "keyword" {
        println!("Syntax error, Keyword not Found!");
        // exit the program
        std::process::exit(0);
    }
    // check if index value is of map is 1
    if map.get(&1).unwrap().data_type != "Identifier" {
        println!("Syntax error on token[1], invalid VariableDeclarator");
        // exit the program
        std::process::exit(0);
    }
    // if the total length of map is less than 2
    if map.len() < 3 {
        println!("{}", map.len());
        println!("Syntax error, Code line is too short");
        // exit the program
        std::process::exit(0);
    }
    // loop and print the map starting the key index of  map = 2
    for mut i in 2..map.len() {
        println!("{}?", map.get(&i).unwrap().words);
        // if current data_type is equal to single_quote or double_quote
        if map.get(&i).unwrap().data_type == "single_quote"
            || map.get(&i).unwrap().data_type == "double_quote"
        {
            // init count to 1
            let quote_count; //including first quote
            quote_count = quotes(&map, i);
            println!("{} qu", quote_count);
            // call check_end
            check_end(&map, quote_count, i);
        }
        // data_type is equal to equal_ops
        if map.get(&i).unwrap().data_type == "equal_ops" {
            if map.get(&i).unwrap().data_type == "single_quote"
            || map.get(&i).unwrap().data_type == "double_quote"
            {
                // init count to 1
                let quote_count; //including first quote
                quote_count = quotes(&map, i);
                println!("{} qu", quote_count);
                // call check_end
                check_end(&map, quote_count, i);
            }
        }
        // if current data_type is equal to "left_paren"
        else if map.get(&i).unwrap().data_type == "left_paren" {
            // if the index is 2
            if i == 2 {
                // return error
                println!("Syntax error, This method requires a body instead of a semicolon");
                // exit the program
                std::process::exit(0);
            }
            balance_paren(&map, i);
            i += 1;
            // check if the next index is an operator or right_paren
            if map.get(&i).unwrap().data_type == "operator"
                && map.get(&i).unwrap().data_type == "right_paren"
            {
                // return error
                println!("Syntax error, Should have identifier or constant");
                // exit the program
                std::process::exit(0);
            }
        }
        // operator
        else if map.get(&i).unwrap().data_type == "operator" {
            // if index is 2
            if i == 2 {
                // return error
                println!("Syntax error on token \"{}\", delete this token", map.get(&i).unwrap().words);
                // exit the program
                std::process::exit(0);
            }
        }
        // if current data_type is equal to "right_paren"
        else if map.get(&i).unwrap().data_type == "right_paren" {
            // if the index is 2
            if i == 2 {
                // return error
                println!("Syntax error, Syntax error on token \")\", delete this token");
                // exit the program
                std::process::exit(0);
            }
            // FIXME: confuse with error according with )
        }
    }
    // symbolTable(&map);
}

fn sytax_analyzer(map: &HashMap<usize, Dictionary>) {
    /*
    var_dec ::= data_type dec ;
    data_type ::= int | float | char | boolean | string
    dec ::= exp [= exp] | exp, dec
    exp ::= exp(+|*|/|-)exp | '('exp')' | a-z-A-Z [exp] | 0 - 9 [exp] | " exp " | ' exp ' | exp . exp
    identifier:(a-z-A-Z)[a-z-A-Z-0-9_]
    */
    // let input = "
    //     <var_dec> ::= <data_type> <dec> ;
    //     <data_type> ::= int | float | char | boolean | string
    //     <dec> ::= <exp> [= <exp>] | <exp>, <dec>
    //     <exp> ::= <exp>(+|*|/|-)<exp> | '('<exp>')' | a-z-A-Z [<exp>] | 0 - 9 [<exp>] | \" <exp> \" | ' <exp> ' | <exp> . <exp>
    //     <identifier>:(a-z-A-Z)[a-z-A-Z-0-9_]
    // "
    // let grammar = bnf::parse(input);
    
    // println!("{:#?}", grammar);

    // // create new map of sample rules above
    // let mut rules = HashMap::new();
    // // add rules to the map
    // rules.insert("var_dec", "data_type dec ;");
    // rules.insert("data_type", "int | float | char | boolean | string");
    // rules.insert("dec", "exp [= exp] | exp, dec");
    // rules.insert("exp", "exp(+|*|/|-)exp | '('exp')' | a-z-A-Z [exp] | 0 - 9 [exp] | \" exp \" | ' exp ' | exp . exp");
    // rules.insert("identifier", "(a-z-A-Z)[a-z-A-Z-0-9_]");
    
    // let bnf::Rule::<()>::program(ref program) = bnf::Rule::<()>::parse(rules.as_slice());
    // println!("{:?}", program);

}

fn quotes(map: &HashMap<usize, Dictionary>, i: usize) -> usize {
    let mut quote_count = 1;
    println!("s i {} quote {}", i, quote_count);
    for j in i + 1..map.len() {
        if map.get(&j).unwrap().data_type == "single_quote"
            || map.get(&j).unwrap().data_type == "double_quote"
        {
            quote_count += 1;
        }
    }
    println!("quote_count: {} ==> i: {}", quote_count,i);
    quote_count
}
/*
creaate a symbol table with binary tree search
fn symbolTable(map: &HashMap<usize, Dictionary>) {
    // create a symbol table
    let mut symbol_table = HashMap::new();
    // iterate over the map
    for i in map.keys() {
        // if the current data_type is equal to "Identifier"
        if map.get(&i).unwrap().data_type == "Identifier" {
            *i += 1;
            // check if next index is equal to "equal_ops"
            if map.get(&i).unwrap().data_type == "equal_ops" {
                *i += 2;
                // if the next index is equal to "single_quote" or "double_quote"
                if map.get(&i).unwrap().data_type == "single_quote"
                    || map.get(&i).unwrap().data_type == "double_quote"
                {
                    // iterate until the data_type is equal to "single_quote" and "double_quote"
                    // and concat the values between the first index of "single_quote" or "double_quote"
                    //  and the second index "double_quote or "single_quote"
                    let mut value = String::new();
                    *i += 2;
                    while map.get(&i).unwrap().data_type != "single_quote"
                        && map.get(&i).unwrap().data_type != "double_quote"
                    {
                        value.push_str(&map.get(&i).unwrap().words);
                        // increment i
                        *i += 1;
                    }
                    // push the value into the symbol table
                    symbol_table.insert(map.get(&i).unwrap().words.clone(), value);
                }
            }
        }
    }
    // print the symbol table
    // iterate over the symbol table
    for (key, value) in symbol_table.iter_mut() {
        println!("{} = {}", key, value);
    }
}
*/

fn check_end(map: &HashMap<usize, Dictionary>, quote_count: usize, i: usize) {
    // FIXME: quote_count index error ex. if 4 quotes, if quote_count is 1 it return 3, if 2 it return 5
    // add 1 to quote_count
    // if count is odd then print error
    if quote_count % 2 == 1 {
        println!(
            "String literal is not properly closed by a quote {}",
            quote_count
        );
        // exit the program
        std::process::exit(0);
    }
    // last index of map
    if i == map.len() - 1 {
        if map.get(&i).unwrap().data_type == "Terminator"
        {
            println!("Syntax error, insert \";\" to complete BlockStatements");
            // exit the program
            std::process::exit(0);
        }

    }
}

fn balance_paren(map: &HashMap<usize, Dictionary>, i: usize){
    let mut left_paren_count = 1;
    let mut right_paren_count = 0;
    for j in i + 1..map.len() {
        if map.get(&j).unwrap().data_type == "left_paren" {
            left_paren_count += 1;
        }
        if map.get(&j).unwrap().data_type == "right_paren" {
            right_paren_count += 1;
        }
    }
    if left_paren_count != right_paren_count {
        // return error and exit
        println!("Syntax error, Unbalanced Parenthesis i {}: {} == {} ",i, left_paren_count, right_paren_count);
        std::process::exit(0);
    }
}


fn print_map(map: &HashMap<usize, Dictionary>) {
    // print the map
    print!("------------------------------------------------");
    print!("Unsorted Map!");
    print!("------------------------------------------------\n");
    // print the map index and dictionary word and data_type
    for (index, dictionary) in map.iter() {
        println!("{} {} {}", index, dictionary.words, dictionary.data_type);
    }
    // sort the map by index
    let mut sorted_map: Vec<(&usize, &Dictionary)> =
        map.iter().collect::<Vec<(&usize, &Dictionary)>>();
    sorted_map.sort_by(|a, b| a.0.cmp(&b.0));
    // print the sorted map
    print!("------------------------------------------------");
    print!("Sorted Map!");
    print!("------------------------------------------------\n");
    for (index, dictionary) in sorted_map.iter() {
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

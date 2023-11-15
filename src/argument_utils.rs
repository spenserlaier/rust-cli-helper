use std::collections::{HashSet, HashMap};

#[derive(Debug, PartialEq, Eq, Hash, Clone )]
pub enum ArgName {
    ArgName(String)
}
#[derive(Debug, PartialEq)]
pub enum ArgVal {
    ArgValString(String),
    ArgValUsize(usize),
    ArgValNone,
}
#[derive(Clone)]
pub enum ArgType {
    ArgTypeString,
    ArgTypeUsize,
    ArgTypeNoValue
}
#[derive(Debug, PartialEq)]
pub enum Argument{
    PairedArgument(ArgName, ArgVal),
    SingleArgument(ArgName),
}
//TODO: introduce convenience parsing of dash prefixes
//TODO: introduce inline paired arguments ex --name=bobjones
pub fn construct_arg_type(input: String, arg_type: Option<String>)-> (ArgName, ArgType){
    let arg_name = ArgName::ArgName(input.clone());
    if input.len() >= 1 {
        if input.chars().nth(0) == Some('-') {
            panic!("Cannot have leading dashes in an option name");
        }
        else if input.contains("=") {
            panic!("Cannot have equals signs in an option name");
        }
        let arg_type = match arg_type{
            None => ArgType::ArgTypeNoValue,
            Some(arg_str) => {
                match arg_str.as_str() {
                    "usize" => ArgType::ArgTypeUsize,
                    "string" => ArgType::ArgTypeString,
                    x => {
                        panic!("unrecognized argument type: {}", x);
                    }
                }
            }
        };
        (arg_name, arg_type)
    }
    else{
        panic!("Cannot have an empty string as an argument");
    }
    
}
fn parse_arg_value(input_str: &str, arg_type: ArgType) -> ArgVal {
    match arg_type {
        ArgType::ArgTypeUsize => {
                if let Ok(parsed_usize) = input_str.parse::<usize>() {
                    ArgVal::ArgValUsize(parsed_usize)
                }
                else{
                    panic!("unable to parse usize from provided argument: {} ", input_str);
                }
        }
        ArgType::ArgTypeString => {
            ArgVal::ArgValString(input_str.to_string())
        }
        ArgType::ArgTypeNoValue => {
           ArgVal::ArgValNone
        }
    }
}
fn parse_arg_type(input_str: &str, stored_types: &HashMap<ArgName, ArgType>) -> Result<ArgType, String > {
    if let Some(arg_type) = stored_types.get(&ArgName::ArgName(input_str.to_string())) {
        Ok(arg_type.clone())
    }
    else{
        Err(String::from("unable to parse argument type"))
    }
}
fn parse_single_argument(arg_name: &str, arg_type: ArgType, arg_val: &str) -> Argument{
    let parsed_arg_name = ArgName::ArgName(arg_name.to_string());
    let parsed_arg_val = parse_arg_value(arg_val, arg_type);
    match parsed_arg_val {
        ArgVal::ArgValNone => {
            Argument::SingleArgument(parsed_arg_name)
        }
        not_none_type => {
            Argument::PairedArgument(parsed_arg_name, not_none_type)
        }
    }
}
fn remove_leading_dashes(arg: &str) -> &str {
    // strip leading dashes
    let mut idx = 0;
    while arg.chars().nth(idx) == Some('-') {
        idx += 1;
    }
    let stripped_arg = &arg[idx..];
    if arg.len() == 0 {
        panic!("invalid argument length. Arguments cannot contain only dashes")
    }
    stripped_arg
}
fn is_embedded_arg(arg: &str) -> bool {
    if arg.contains("=") &&
        arg.chars().filter(|&c| c == '=').count() == 1{
        let mut components = arg.split("=");

    }
    false
}
fn parse_embedded_arg(arg: &str, stored_types: &HashMap<ArgName, ArgType>) -> Option<Argument> {
    if arg.contains("=") {
        if arg == "=" {
            panic!("An inputted argument cannot contain an uncoupled equals sign");
        }
        if arg.chars().filter(|&c| c == '=').count() >= 2 {
            panic!("An inputted argument cannot contain more than one equals sign");
        }
        else{
            let mut parsed_arg_iter = arg.split("=");
            let unparsed_arg_name = parsed_arg_iter.nth(0).unwrap().to_string();
            let unparsed_arg_value = parsed_arg_iter.nth(1).unwrap().to_string();
            let arg_type = parse_arg_type(&unparsed_arg_name, stored_types).unwrap();
            let parsed_argument = parse_single_argument(&unparsed_arg_name, arg_type, &unparsed_arg_value);
            return Some(parsed_argument);
        }
    }
    None
}
fn parse_isolated_arg(arg: &str, arg_types: &HashMap<ArgName, ArgType>) -> Argument{
    let parsed_arg_name = ArgName::ArgName(arg.to_string());
    if arg_types.contains_key(&parsed_arg_name) {
        Argument::SingleArgument(parsed_arg_name)
    }
    else{
        panic!("unrecognized argument name");
    }
}
pub fn parse_arguments(input: Vec<String>, arg_types: HashMap<ArgName, ArgType>) -> Vec<Argument> {
    let mut idx = 0;
    let mut parsed_arguments: Vec<Argument> = Vec::new();
    while idx < input.len() {
        let mut increment = 1;
        let current_arg_name = remove_leading_dashes(input.get(idx).unwrap());
        if is_embedded_arg(current_arg_name) {
            let embedded_arg = parse_embedded_arg(current_arg_name, &arg_types);
            parsed_arguments.push(embedded_arg.unwrap());
        }
        else{
            if idx < input.len() -1 {
                let parsed_arg_type = parse_arg_type(current_arg_name, &arg_types).unwrap();
                let raw_arg_value = input.get(idx+1).unwrap();
                let argument = parse_single_argument(current_arg_name, parsed_arg_type, raw_arg_value);
                parsed_arguments.push(argument);
                increment = 2;
            }
            else{
                //panic!("A non-embedded argument expected a value but wasn't provided with one");
                let parsed_isolated_arg = parse_isolated_arg(current_arg_name, &arg_types);
                parsed_arguments.push(parsed_isolated_arg);
            }
        }
        idx += increment;
    }
    parsed_arguments
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recognize_single_argument() {
        let mut args_hashmap = HashMap::new();
        let arg_name = String::from("sample_string_arg");
        let arg_type = construct_arg_type(arg_name.clone(), None);
        args_hashmap.insert(arg_type.0.clone(), arg_type.1);
        let parsed_args = parse_arguments(vec![arg_name.clone()], args_hashmap);
        assert_eq!(parsed_args, vec![Argument::SingleArgument(arg_type.0.clone())]);
    }
    #[test]
    fn recognize_paired_argument() {
        let mut args_hashmap = HashMap::new();
        let arg_name = String::from("sample_paired_string_arg");
        let arg_val = String::from("string_val");
        let arg_type = construct_arg_type(arg_name.clone(), Some(String::from("string")));
        args_hashmap.insert(arg_type.0.clone(), arg_type.1);
        let parsed_args = parse_arguments(vec![arg_name.clone(), arg_val.clone()], args_hashmap);
        assert_eq!(parsed_args, vec![Argument::PairedArgument(arg_type.0.clone(), 
                                                              ArgVal::ArgValString(arg_val.clone())
                                                              )]);
    }
}

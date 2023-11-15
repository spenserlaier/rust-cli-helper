use std::collections::{HashSet, HashMap};

#[derive(Debug, PartialEq, Eq, Hash, Clone )]
pub enum ArgName {
    ArgName(String)
}
#[derive(Debug, PartialEq)]
pub enum ArgVal {
    ArgValString(String),
    ArgValUsize(usize),
    ArgValBool(bool),
    ArgValNone,
}
#[derive(Clone)]
pub enum ArgType {
    ArgTypeString,
    ArgTypeUsize,
    ArgTypeBool,
    ArgTypeNoValue,
}
#[derive(Debug, PartialEq)]
pub enum Argument{
    PairedArgument(ArgName, ArgVal),
    SingleArgument(ArgName),
}

pub fn construct_arg_type(arg_type: Option<String>) -> ArgType{
    let parsed_arg_type = match arg_type{
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
    parsed_arg_type
}
pub fn construct_arg_name(arg_name: String) -> ArgName {
    if arg_name.len() >= 1 {
        if arg_name.chars().nth(0) == Some('-') {
            panic!("Cannot have leading dashes in an option name");
        }
        else if arg_name.contains("=") {
            panic!("Cannot have equals signs in an option name");
        }
        let parsed_arg_name = ArgName::ArgName(arg_name.clone());
        parsed_arg_name
    }
    else{
        panic!("Cannot have an empty string as an argument name");
    }
}

pub fn construct_arg_tuple(arg_name: String, arg_type: Option<String>)-> (ArgName, ArgType){
    let constructed_arg_name = construct_arg_name(arg_name);
    let constructed_arg_type = construct_arg_type(arg_type);
    (constructed_arg_name, constructed_arg_type)
}
fn construct_arg_val(input_str: &str, arg_type: ArgType) -> ArgVal {
    match arg_type {
        ArgType::ArgTypeUsize => {
                if let Ok(parsed_usize) = input_str.parse::<usize>() {
                    ArgVal::ArgValUsize(parsed_usize)
                }
                else{
                    panic!("unable to parse usize from provided argument: {} ", input_str);
                }
        }
        ArgType::ArgTypeBool => {
            if let Ok(parsed_bool) = input_str.parse::<bool>() {
                ArgVal::ArgValBool(parsed_bool)
            }
            else{
                panic!("unable to parse bool from provided argument: {} ", input_str);
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
fn get_arg_type(input_str: &str, stored_types: &HashMap<ArgName, ArgType>) -> Result<ArgType, String > {
    if let Some(arg_type) = stored_types.get(&ArgName::ArgName(input_str.to_string())) {
        Ok(arg_type.clone())
    }
    else{
        Err(String::from("unable to parse argument type"))
    }
}
fn parse_single_argument(arg_name: &str, arg_type: ArgType, arg_val: &str) -> Argument{
    let parsed_arg_name = ArgName::ArgName(arg_name.to_string());
    let parsed_arg_val = construct_arg_val(arg_val, arg_type);
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
        let components = arg.split("=");
        if components.count() == 2 {
            return true
        }
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
            let unparsed_arg_value = parsed_arg_iter.nth(0).unwrap().to_string();
            // recall that nth() consumes the values, so calling nth(0) repeatedly returns
            // different values
            let arg_type = get_arg_type(&unparsed_arg_name, stored_types).unwrap();
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
pub fn insert_argument_type(arg_name: &str, arg_type: &str, arg_types: &mut HashMap<ArgName, ArgType>){
    let arg_tuple = construct_arg_tuple(String::from(arg_name), Some(String::from(arg_type)));
    arg_types.insert(arg_tuple.0, arg_tuple.1);
}
pub fn initialize_arg_types_hashmap() -> HashMap<ArgName, ArgType> {
    let arg_types_hashmap = HashMap::new();
    arg_types_hashmap
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
                let parsed_arg_type = get_arg_type(current_arg_name, &arg_types).unwrap();
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
        let arg_type = construct_arg_tuple(arg_name.clone(), None);
        args_hashmap.insert(arg_type.0.clone(), arg_type.1);
        let parsed_args = parse_arguments(vec![arg_name.clone()], args_hashmap);
        assert_eq!(parsed_args, vec![Argument::SingleArgument(arg_type.0.clone())]);
    }
    #[test]
    fn recognize_paired_argument() {
        let mut args_hashmap = HashMap::new();
        let arg_name = String::from("sample_paired_string_arg");
        let arg_val = String::from("string_val");
        let arg_type = construct_arg_tuple(arg_name.clone(), Some(String::from("string")));
        args_hashmap.insert(arg_type.0.clone(), arg_type.1);
        let parsed_args = parse_arguments(vec![arg_name.clone(), arg_val.clone()], args_hashmap);
        assert_eq!(parsed_args, vec![Argument::PairedArgument(arg_type.0.clone(), 
                                                              ArgVal::ArgValString(arg_val.clone())
                                                              )]);
    }
    #[test]
    fn recognize_embedded_argument() {
        let embedded_argument = String::from("--test=32");
        let mut args_hashmap :HashMap<ArgName, ArgType> = HashMap::new();
        let arg_name = String::from("test");
        let (constructed_arg_name, constructed_arg_type) = construct_arg_tuple(arg_name.clone(), Some(String::from("usize")));
        args_hashmap.insert(constructed_arg_name.clone(), constructed_arg_type.clone());
        let parsed_args = parse_arguments(vec![embedded_argument], args_hashmap);
        assert_eq!(parsed_args, vec![Argument::PairedArgument(constructed_arg_name, 
                                                              ArgVal::ArgValUsize(32))]);
    }
    #[test]
    fn recognize_argument_using_helpers() {
        let arg_name = "sample_arg_name";
        let arg_val = "24";
        let mut arg_types = initialize_arg_types_hashmap();
        insert_argument_type("sample_arg_name", "usize", &mut arg_types);
        let args_vec = vec![String::from("--sample_arg_name"), String::from("24")];
        let parsed_args = parse_arguments(args_vec, arg_types);
        assert_eq!(parsed_args, vec![Argument::PairedArgument(
                ArgName::ArgName(String::from(arg_name)),
                ArgVal::ArgValUsize(24)
                )]);
    }
}

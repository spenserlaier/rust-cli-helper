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
pub fn parse_arguments(input: Vec<String>, arg_types: HashMap<ArgName, ArgType>) -> Vec<Argument> {
    let mut idx = 0;
    let mut parsed_arguments: Vec<Argument> = Vec::new();
    while idx < input.len() {
        let arg_name = input.get(idx).unwrap();
        if arg_name.chars().nth(0) == Some('-') && arg_name.chars().nth(idx) == Some('-') {
            //remove leading dashes
            idx += 1;
        }
        let parsed_arg_name: ArgName;
        let parsed_arg_val: ArgVal;
        if arg_name.contains("=") {
            if arg_name.chars().filter(|&c| c == '=').count() >= 2 {
                //TODO: since args are inputted by the user, should probably return an error
                //instead of panicking
                panic!("An inputted argument cannot contain more than one equals sign");
            }
            else{
                let mut parsed_arg_iter = arg_name.split("=");
                parsed_arg_name = ArgName::ArgName(parsed_arg_iter.nth(0).unwrap().to_string());
            }
        }
        else{
            parsed_arg_name = ArgName::ArgName(arg_name.to_string());
        }

        let arg_type = parse_arg_type(arg_name, &arg_types).unwrap();
        match arg_type {
            ArgType::ArgTypeUsize => {
                if let Some(arg_val_str) = input.get(idx+1) {
                    let arg_val = parse_arg_value(arg_val_str, arg_type);
                    let new_arg = Argument::PairedArgument(ArgName::ArgName(arg_name.clone()), arg_val);
                    parsed_arguments.push(new_arg);
                }
                else{
                    panic!("Missing required value for parameter {}", arg_name.to_string());
                }
                idx += 1;
            },
            ArgType::ArgTypeString => {
                if let Some(arg_val_str) = input.get(idx+1) {
                        let new_arg = Argument::PairedArgument(
                            ArgName::ArgName(arg_name.to_string()),
                            ArgVal::ArgValString(arg_val_str.to_string()));
                        parsed_arguments.push(new_arg);
                }
                else{
                    panic!("Missing required value for parameter {}", arg_name.to_string());
                }
                idx += 1;
            },
            ArgType::ArgTypeNoValue => {
                let new_arg = Argument::SingleArgument(ArgName::ArgName(arg_name.to_string()));
                parsed_arguments.push(new_arg);
            },
        }
        idx += 1;
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

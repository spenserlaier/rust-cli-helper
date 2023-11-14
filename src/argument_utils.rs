use std::collections::{HashSet, HashMap};

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum ArgName {
    ArgName(String)
}
pub enum ArgVal {
    ArgValString(String),
    ArgValUsize(usize),
}
pub enum ArgType {
    ArgTypeString,
    ArgTypeUsize,
    ArgTypeNoValue
}
pub enum Argument{
    PairedArgument(ArgName, ArgVal),
    SingleArgument(ArgName),
}
pub fn construct_arg_type(input: String, arg_type: Option<String>)-> (ArgName, ArgType){
    let arg_name = ArgName::ArgName(input);
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
fn parse_arg_value(input_str: &str, arg_type: ArgType) {
    //TODO: helper function for parsing argument values
}
pub fn parse_arguments(input: Vec<String>, arg_types: HashMap<ArgName, ArgType>) -> Vec<Argument> {
    let mut idx = 0;
    let mut parsed_arguments: Vec<Argument> = Vec::new();
    while idx < input.len() {
        let arg_name = input.get(idx).unwrap();
        if let Some(arg_type) = arg_types.get(&ArgName::ArgName(arg_name.to_string())) {
            match arg_type {
                ArgType::ArgTypeUsize => {
                    if let Some(arg_val_str) = input.get(idx+1) {
                            if let Ok(parsed_usize) = arg_val_str.parse::<usize>() {
                                let new_arg = Argument::PairedArgument(
                                    ArgName::ArgName(arg_name.to_string()),
                                    ArgVal::ArgValUsize(parsed_usize));
                                parsed_arguments.push(new_arg);
                            }
                            else{
                                panic!("Unable to parse usize for argument {} from provided string: {}", 
                                       arg_name,
                                       arg_val_str);
                            }
                    }
                    else{
                        panic!("Missing required value for parameter {}", arg_name.to_string());
                    }
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
                },
                ArgType::ArgTypeNoValue => {
                    let new_arg = Argument::SingleArgument(ArgName::ArgName(arg_name.to_string()));
                    parsed_arguments.push(new_arg);
                },
            }

        }
        else{
            panic!("unrecognized argument name");
        }
        idx += 1;
    }
    Vec::new()
}

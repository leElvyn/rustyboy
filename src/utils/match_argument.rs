use crate::enums::enumsProcessingUnit::{ArgumentTypes};

pub fn match_arg(argument:&str) -> ArgumentTypes{
    if argument.len() == 4 {
        return ArgumentTypes::MemoryAddress
    }
    else if argument.is_ascii() {
        return ArgumentTypes::RegisterAddress
    }
    else if { let chars_are_numeric: Vec<bool> = argument.chars().map(|c|c.is_numeric()).collect();
    let b = !chars_are_numeric.contains(&false); 
    b} {
        return ArgumentTypes::NumericalValue
    }
    else {
        panic!()
    }
}
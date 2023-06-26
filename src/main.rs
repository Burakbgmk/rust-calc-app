use std::io;
fn main() -> io::Result<()> {
    println!("\x1B[2J\x1B[1;1H");
    
    let stdin = io::stdin();
    println!("Calculator");
    let mut i = 0;
    while i < 20 {
        let mut input = String::new();
        println!("Type any operation you like!");
        let _ = stdin.read_line(&mut input);
        input = input.chars().filter(|c| !c.is_whitespace()).collect();
        let result = parse_calculations(input.to_string());
        println!("RESULT IS : {}",result);
        i += 1;
    }

    Ok(())
}


fn parse_calculations(calculation_string: String) -> String {
    let opts = vec!['*','/','+','-'];
    
    let mut _op = String::new();
    if calculation_string.chars().any(|r| r.eq(&'*')) {
        _op = "*".to_string();
    } else if calculation_string.chars().any(|r| r.eq(&'/')) {
        _op = "/".to_string();
    } else {
        _op = "+-".to_string();
    }
    let operator_map: Vec<i32> = map_operators(&calculation_string, "+-*/");
    if operator_map.len().eq(&0) || (calculation_string[0..1].eq("-") && operator_map.len().eq(&1)) {
        return calculation_string;
    }
    
    let length = calculation_string.len();
    let mut i = 0;
    while i < length {
        let index:i32 = i as i32;
        let c = calculation_string.chars().nth(i).unwrap();
        if opts.contains(&c) {
            let current_index_at = operator_map.iter().position(|&r| r.eq(&index)).unwrap();
            let mut before_index = 0;
            if current_index_at != 0 {
                before_index = operator_map[current_index_at-1] as usize;
                before_index += 1;
            }
            let mut after_index = length;
            if operator_map.len()-1 != current_index_at {
                after_index = operator_map[current_index_at+1] as usize;
                
            }
            if calculation_string[0..1].to_string().eq("-") && operator_map.len().eq(&2) {
                after_index = length;
            }
            let opt_str = calculation_string[before_index..after_index].to_string();
            let result;
            if c.eq(&'*') && _op.eq("*") {
                result = Operation::Multiplication { op_str: opt_str }.operate().to_string();
            } else if c.eq(&'/') && _op.eq("/") {
                result = Operation::Division { op_str: opt_str }.operate().to_string();
            } else if c.eq(&'+') && _op.eq("+-") {
                result = Operation::Addition { op_str: opt_str }.operate().to_string();
            } else if c.eq(&'-') && _op.eq("+-") {
                result = Operation::Substruction { op_str: opt_str }.operate().to_string();
            } else {
                i+=1;
                continue;
            }
            let new_calc_str = calculation_string[0..before_index].to_string() + &result + &calculation_string[after_index..calculation_string.len()].to_string();
            return parse_calculations(new_calc_str);
            
        }
        i += 1;
    }
    return "".to_string();
    
}

fn map_operators(calculation_string: &str, _op: &str) -> Vec<i32> {
    let mut operator_map:Vec<i32> = Vec::new();
    let mut index = 0;
    
    for c in calculation_string.chars() {
        if _op.contains(c) {
            operator_map.push(index);
        }
        index += 1;
    }
    return operator_map;
}



enum Operation {
    Addition{op_str: String},
    Substruction{op_str: String},
    Multiplication{op_str: String},
    Division{op_str: String}
}

impl Operation {
    fn operate(&self) -> String {
        
        match self {
            Operation::Addition {op_str} => {
                let index_of_operator = op_str.chars().into_iter().position(|r| r.eq(&'+')).unwrap();
                let x = op_str[0..index_of_operator].parse::<f64>().unwrap();
                let y = op_str[index_of_operator+1..op_str.len()].parse::<f64>().unwrap();
                let z = x as f64 + y as f64;
                return z.to_string()
            },
            Operation::Substruction {op_str} => {
                let index_of_operator = op_str[1..].chars().into_iter().position(|r| r.eq(&'-')).unwrap() + 1;
                let x = op_str[0..index_of_operator].parse::<f64>().unwrap();
                let y = op_str[index_of_operator+1..op_str.len()].parse::<f64>().unwrap();
                let z = x as f64 - y as f64;
                return z.to_string()
            },
            Operation::Multiplication {op_str} => {
                let index_of_operator = op_str.chars().into_iter().position(|r| r.eq(&'*')).unwrap();
                let x = op_str[0..index_of_operator].parse::<f64>().unwrap();
                let y = op_str[index_of_operator+1..op_str.len()].parse::<f64>().unwrap();
                let z = x as f64 * y as f64;
                return z.to_string()
            },
            Operation::Division {op_str} => {
                let index_of_operator = op_str.chars().into_iter().position(|r| r.eq(&'/')).unwrap();
                let x = op_str[0..index_of_operator].parse::<f64>().unwrap();
                let y = op_str[index_of_operator+1..op_str.len()].parse::<f64>().unwrap();
                let z = x as f64 / y as f64;
                return z.to_string()
            }
        }
        
    }
}

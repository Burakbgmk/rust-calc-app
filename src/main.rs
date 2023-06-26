
fn main() {

    let result = parse_calculations("33/2+7".to_string());
    println!("RESULT IS : {}",result);


}

fn map_operators(calculation_string: &str) -> Vec<i32> {
    let mut operator_map:Vec<i32> = Vec::new();
    let mut index = 0;
    for c in calculation_string.chars() {
        if c.eq(&'*') || c.eq(&'/') || c.eq(&'+') || c.eq(&'-') {
            operator_map.push(index);
        }
        index += 1;
    }
    return operator_map;
}



fn parse_calculations(calculation_string: String) -> String {
    let opts = vec!['*','/','+','-'];
    let operator_map: Vec<i32> = map_operators(&calculation_string);
    println!("operators mapped with length: {}",operator_map.len());
    if operator_map.len().eq(&0) || calculation_string[0..1].eq("-") {
        println!("DONE");
        return calculation_string;
    }

    let length = calculation_string.len();
    println!("calculation string is {} and length is {}",calculation_string,length);
    let mut i = 0;
    while i < length {
        let index:i32 = i as i32;
        let c = calculation_string.chars().nth(i).unwrap();
        println!("Iteration {} is started and character is {}",index,c);
        if opts.contains(&c) {
            let current_index_at = operator_map.iter().position(|&r| r.eq(&index)).unwrap();
            let mut before_index = 0;
            if current_index_at != 0 {
                before_index = operator_map[current_index_at-1] as usize;
                before_index += 1;
            }
            let mut after_index = length;
            if operator_map.len() > 1 {
                after_index = operator_map[current_index_at+1] as usize;

            }
            let opt_str = calculation_string[before_index..after_index].to_string();
            let result;
            println!("Before index: {} - After index: {}",before_index,after_index);
            println!("optstr is {}",opt_str);
            if c.eq(&'*') {
                result = Operation::Multiplication { op_str: opt_str }.operate().to_string();
            } else if c.eq(&'/') {
                result = Operation::Division { op_str: opt_str }.operate().to_string();
            } else if c.eq(&'+') {
                result = Operation::Addition { op_str: opt_str }.operate().to_string();
            } else if c.eq(&'-') {
                result = Operation::Substruction { op_str: opt_str }.operate().to_string();
            } else {
                continue;
            }
            println!("operation is done");
            let new_calc_str = calculation_string[0..before_index].to_string() + &result + &calculation_string[after_index..calculation_string.len()].to_string();
            println!("new string is {}",new_calc_str);
            return parse_calculations(new_calc_str);
            
        }
        i += 1;
    }
    return "".to_string();

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
                let index_of_operator = op_str.chars().into_iter().position(|r| r.eq(&'-')).unwrap();
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
                println!("sçdöfm");
                let x = op_str[0..index_of_operator].parse::<f64>().unwrap();
                let y = op_str[index_of_operator+1..op_str.len()].parse::<f64>().unwrap();
                println!("x: {} and y : {}",x,y);
                let z = x as f64 / y as f64;
                return z.to_string()
            }
        }
        
    }
}

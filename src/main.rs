use std::collections::HashMap;

fn main() {
    // println!("Hello, world!");
    // let add = Operation::Addition { x: 3, y: 5 };
    // let sub = Operation::Substruction { op_str: "7-5".to_string() };
    // let mult = Operation::Multiplication { op_str: "2*5".to_string() };
    // let div = Operation::Division { op_str: "9/3".to_string() };
    // println!("{}",add.operate());
    // println!("{}",sub.operate());
    // println!("{}",mult.operate());
    // println!("{}",div.operate());
    // let mapp = map_operators("2+5*8/3".to_string());
    // for ele in mapp {
    //     println!("Key is {}, Value is {}",ele.0,ele.1);
    // }

}

fn map_operators(calculation_string: String) -> HashMap<i32, String> {
    let mut operator_map: HashMap<i32, String> = HashMap::new();
    let mut index = 0;
    for c in calculation_string.chars() {
        if c.eq(&'*') || c.eq(&'/') || c.eq(&'+') || c.eq(&'-') {
            let operator = c.to_string();
            operator_map.insert(index,operator);
        }
        index += 1;
    }
    return operator_map;
}



fn parse_calculation_string(calculation_string: String) {
    

}

enum Symbol {
    Add,
    Substract,
    Multiply,
    Divide
}

impl Symbol {
    fn get(&self) -> String {
        match self {
            Symbol::Add => "+".to_string(),
            Symbol::Substract => "-".to_string(),
            Symbol::Multiply => "*".to_string(),
            Symbol::Divide => "/".to_string()
        }
    }
}


enum Operation {
    Addition{x: i32, y: i32},
    Substruction{x: i32, y: i32},
    Multiplication{x: i32, y: i32},
    Division{x: i32, y: i32}
}

impl Operation {
    fn operate(&self) -> String {
        match self {
            Operation::Addition {x,y} => {
                // let x = op_str.chars().nth(0).unwrap().to_digit(10).unwrap();
                // let y = op_str.chars().nth(2).unwrap().to_digit(10).unwrap();
                let z = x + y;
                return z.to_string()
            },
            Operation::Substruction {x,y} => {
                // let x = op_str.chars().nth(0).unwrap().to_digit(10).unwrap();
                // let y = op_str.chars().nth(2).unwrap().to_digit(10).unwrap();
                let z = x - y;
                return z.to_string()
            },
            Operation::Multiplication {x,y} => {
                // let x = op_str.chars().nth(0).unwrap().to_digit(10).unwrap();
                // let y = op_str.chars().nth(2).unwrap().to_digit(10).unwrap();
                let z = x * y;
                return z.to_string()
            },
            Operation::Division {x,y} => {
                // let x = op_str.chars().nth(0).unwrap().to_digit(10).unwrap();
                // let y = op_str.chars().nth(2).unwrap().to_digit(10).unwrap();
                let z = x / y;
                return z.to_string()
            }
        }
        
    }
}

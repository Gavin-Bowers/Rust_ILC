#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack: Vec<i32> = Vec::new();
    for input in inputs.iter() {
        match input {
            CalculatorInput::Value(val) => 
                stack.push(*val),
            CalculatorInput::Add => 
                if stack.len() >= 2 {
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    stack.push(a + b);
                } else {
                    return None;
                },
            CalculatorInput::Multiply => 
                if stack.len() >= 2 {
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    stack.push(a * b);
                } else {
                    return None;
                },
            CalculatorInput::Subtract => 
                if stack.len() >= 2 {
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    stack.push(b - a);
                } else {
                    return None;
                },
            CalculatorInput::Divide => 
                if stack.len() >= 2 {
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    stack.push(b / a);
                } else {
                    return None;
                },
        }
    }
    if stack.len() == 1 {
        return Some(stack.pop().unwrap());
    } else {
        return None;
    }
}

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
    for input in inputs {
        match input {
            CalculatorInput::Value(value)=> stack.push(*value),
            _ => {
                if stack.len() < 2 { return None; }
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();

                let value = match input {
                    CalculatorInput::Add => a + b,
                    CalculatorInput::Subtract => a - b,
                    CalculatorInput::Multiply => a * b,
                    CalculatorInput::Divide => a / b,
                    _ => panic!("should not be reached"),
                };
                stack.push(value);
            }
        }
    }
    if stack.len() != 1 {
        return None;
    }
    return stack.pop()
}

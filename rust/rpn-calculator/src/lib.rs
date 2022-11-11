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
    inputs_calculator(inputs, &mut stack);
    if stack.len() != 1 {
        return None;
    }
    return stack.pop()
}

fn inputs_calculator(inputs: &[CalculatorInput], stack: &mut Vec<i32>) {
    for input in inputs {
        match input {
            CalculatorInput::Value(value) => stack.push(*value),
            _ => {
                if stack.len() < 2 {
                    stack.truncate(0);
                    break;
                }
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();

                let value = value_calculator(input, b, a);
                stack.push(value);
            }
        }
    }
}

fn value_calculator(input: &CalculatorInput, b: i32, a: i32) -> i32 {
    match input {
        CalculatorInput::Add => a + b,
        CalculatorInput::Subtract => a - b,
        CalculatorInput::Multiply => a * b,
        CalculatorInput::Divide => a / b,
        _ => panic!("should not be reached"),
    }
}

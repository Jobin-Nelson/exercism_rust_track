#[derive(Debug)]
enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    use CalculatorInput::*;

    let mut stack: Vec<i32> = Vec::new();
    for input in inputs {
        match input {
            Value(num) => stack.push(*num),
            Add => {
                let left: i32 = stack.pop()?;
                let right: i32 = stack.pop()?;
                stack.push(left + right);
            }
            Subtract => {
                let left: i32 = stack.pop()?;
                let right: i32 = stack.pop()?;
                stack.push(right - left);
            }
            Multiply => {
                let left: i32 = stack.pop()?;
                let right: i32 = stack.pop()?;
                stack.push(left * right);
            }
            Divide => {
                let left: i32 = stack.pop()?;
                let right: i32 = stack.pop()?;
                stack.push(right / left);
            }
        }
    }
    if stack.len() > 1 {
        None
    } else {
        stack.pop()
    }
}

fn main() {
    use CalculatorInput::*;
    
    let input1 = [Value(2), Value(2), Add];
    let input2 = [Value(4), Value(8), Add, Value(7), Value(5), Subtract, Divide];

    assert_eq!(evaluate(&input1), Some(4));
    assert_eq!(evaluate(&input2), Some(6));
}

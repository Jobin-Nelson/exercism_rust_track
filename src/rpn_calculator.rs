pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
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

#[cfg(test)]
mod test {
    use super::*;
    use super::CalculatorInput::*;

    #[test]
    fn evaluate_four() {
        let input = [Value(2), Value(2), Add];
        assert_eq!(evaluate(&input), Some(4));
    }

    #[test]
    fn evaluate_six() {
        let input = [Value(4), Value(8), Add, Value(7), Value(5), Subtract, Divide];
        assert_eq!(evaluate(&input), Some(6));
    }

    #[test]
    fn evaluate_9() {
        let input = [Value(3), Value(3), Multiply];
        assert_eq!(evaluate(&input), Some(9));
    }
}

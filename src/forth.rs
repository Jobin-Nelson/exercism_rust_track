pub type Value = i32;
pub type Result = std::result::Result<(), Error>;

#[derive(Debug, Eq, Hash, PartialEq, Copy, Clone)]
pub enum ForthOperation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Dup,
    Drop,
    Swap,
    Over,
    Number(Value),
    Call(usize),
}

#[derive(Debug)]
struct Definition {
    name: String,
    body: Vec<ForthOperation>,
}

pub struct Forth {
    internal_stack: Vec<Value>,
    commands: Vec<Definition>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

impl Forth {
    pub fn new() -> Forth {
        Forth {
            internal_stack: vec![],
            commands: vec![],
        }
    }

    pub fn stack(&self) -> &[Value] {
        &self.internal_stack
    }

    pub fn integer_operation(&mut self, fun: fn(i32, i32) -> i32) -> Result {
        if self.internal_stack.len() < 2 {
            return Err(Error::StackUnderflow);
        };
        let op2: i32 = self.internal_stack.pop().unwrap();
        let op1: i32 = self.internal_stack.pop().unwrap();
        if op2 == 0 && fun(32, 2) == 16 {
            return Err(Error::DivisionByZero);
        }
        self.internal_stack.push(fun(op1, op2));
        return Ok(());
    }

    pub fn duplicate(&mut self) -> Result {
        if self.internal_stack.len() < 1 {
            return Err(Error::StackUnderflow);
        };
        let num = self.internal_stack.last().unwrap();
        self.internal_stack.push(*num);
        Ok(())
    }

    pub fn drop(&mut self) -> Result {
        if self.internal_stack.len() < 1 {
            return Err(Error::StackUnderflow);
        };
        self.internal_stack.pop();
        Ok(())
    }

    pub fn swap(&mut self) -> Result {
        let len = self.internal_stack.len();
        if len < 2 {
            return Err(Error::StackUnderflow);
        };
        self.internal_stack.swap(len - 2, len - 1);
        Ok(())
    }

    pub fn over(&mut self) -> Result {
        if self.internal_stack.len() < 2 {
            return Err(Error::StackUnderflow);
        };
        let num = self
            .internal_stack
            .get(self.internal_stack.len() - 2)
            .unwrap();
        self.internal_stack.push(*num);
        Ok(())
    }

    pub fn execute(&mut self, operand: ForthOperation) -> Result {
        match operand {
            ForthOperation::Add => self.integer_operation(|a, b| a + b),
            ForthOperation::Subtract => self.integer_operation(|a, b| a - b),
            ForthOperation::Multiply => self.integer_operation(|a, b| a * b),
            ForthOperation::Divide => self.integer_operation(|a, b| a / b),
            ForthOperation::Dup => self.duplicate(),
            ForthOperation::Drop => self.drop(),
            ForthOperation::Swap => self.swap(),
            ForthOperation::Over => self.over(),
            ForthOperation::Number(num) => {
                self.internal_stack.push(num);
                Ok(())
            }
            ForthOperation::Call(pointer) => {
                let commands: Vec<ForthOperation> =
                    self.commands[pointer].body.iter().map(|o| *o).collect(); // copy the enum
                for command in commands {
                    self.execute(command)?;
                }
                Ok(())
            }
        }
    }

    pub fn find_command_index(&self, input: &String) -> Option<usize> {
        for (index, definition) in self.commands.iter().enumerate().rev() {
            if definition.name == input.to_string() {
                return Some(index);
            }
        }
        None
    }

    pub fn string_to_command(&self, input: String) -> std::result::Result<ForthOperation, Error> {
        let check = self.find_command_index(&input);
        match check {
            Some(pointer) => return Ok(ForthOperation::Call(pointer)),
            None => {}
        };
        match input.parse::<i32>() {
            Ok(num) => Ok(ForthOperation::Number(num)),
            Err(_) => match input.trim() {
                "+" => Ok(ForthOperation::Add),
                "-" => Ok(ForthOperation::Subtract),
                "*" => Ok(ForthOperation::Multiply),
                "/" => Ok(ForthOperation::Divide),
                "dup" => Ok(ForthOperation::Dup),
                "drop" => Ok(ForthOperation::Drop),
                "swap" => Ok(ForthOperation::Swap),
                "over" => Ok(ForthOperation::Over),
                _ => Err(Error::UnknownWord),
            },
        }
    }

    pub fn string_to_commands(
        &self,
        input: String,
    ) -> std::result::Result<Vec<ForthOperation>, Error> {
        let commands = input
            .split(' ')
            .map(|i| self.string_to_command(i.to_string()).unwrap())
            .collect();
        Ok(commands)
    }

    pub fn eval(&mut self, input: &str) -> Result {
        let mut building: String = "".to_string();
        let mut working_new_word = false;
        let mut learning_command = false;
        let mut new_word = "".to_string();
        let lowercased = input.to_lowercase();
        for c in lowercased.chars() {
            if c == ' ' && building.trim().len() > 0 {
                building = building.trim().to_string();
                if working_new_word {
                    new_word = building;
                    building = "".to_string();
                    working_new_word = false;
                    learning_command = true;
                    continue;
                } else if learning_command == false {
                    self.execute(self.string_to_command(building)?)?;
                    building = "".to_string();
                    continue;
                }
            }
            if c == ':' {
                working_new_word = true;
                continue;
            }
            if c == ';' {
                learning_command = false;
                building = building.trim().to_string();
                if new_word.chars().all(char::is_numeric) {
                    return Err(Error::InvalidWord);
                }
                let existing_commands = self.string_to_commands(building)?;
                let memorize = Definition {
                    name: new_word,
                    body: existing_commands,
                };
                self.commands.push(memorize);
                new_word = "".to_string();
                building = "".to_string();
                continue;
            }
            building.push(c);
        }
        if working_new_word || learning_command {
            return Err(Error::InvalidWord);
        }
        if building.len() > 0 {
            building = building.trim().to_string();
            self.execute(self.string_to_command(building)?)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_input_no_stack() {
        assert_eq!(Vec::<Value>::new(), Forth::new().stack());
    }

    #[test]
    fn numbers_just_get_pushed_onto_the_stack() {
        let mut f = Forth::new();
        assert!(f.eval("1 2 3 4 5").is_ok());
        assert_eq!(vec![1, 2, 3, 4, 5], f.stack());
    }

    #[test]
    fn can_add_two_numbers() {
        let mut f = Forth::new();
        assert!(f.eval("1 2 +").is_ok());
        assert_eq!(vec![3], f.stack());
    }

    #[test]
    fn addition_error() {
        let mut f = Forth::new();
        assert_eq!(Err(Error::StackUnderflow), f.eval("1 +"));
        assert_eq!(Err(Error::StackUnderflow), f.eval("+"));
    }

    #[test]
    fn can_subtract_two_numbers() {
        let mut f = Forth::new();
        assert!(f.eval("3 4 -").is_ok());
        assert_eq!(vec![-1], f.stack());
    }

    #[test]
    fn can_define_word_that_uses_word_with_the_same_name() {
        let mut f = Forth::new();
        assert!(f.eval(": foo 10 ;").is_ok());
        assert!(f.eval(": foo foo 1 + ;").is_ok());
        assert!(f.eval("foo").is_ok());
        assert_eq!(vec![11], f.stack());
    }
}

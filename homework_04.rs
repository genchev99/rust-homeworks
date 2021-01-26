#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RuntimeError {
    DivideByZero,
    StackUnderflow,
    InvalidCommand,
    NoInstructions,
}

use std::collections::VecDeque;

#[derive(Debug, Default)]
pub struct Interpreter {
    pub instructions: VecDeque<String>,
    pub stack: Vec<i32>,
    pub reverse_instructions: Vec<Vec<String>>,
    pub memory_instructions: Vec<String>,
}

impl Interpreter {
    /// Конструира нов интерпретатор с празен стек и никакви инструкции.
    pub fn new() -> Self {
        return Self {
            instructions: VecDeque::new(),
            stack: Vec::new(),
            reverse_instructions: Vec::new(),
            memory_instructions: Vec::new(),
        };
    }

    /// Добавя инструкции от дадения списък към края на `instructions`. Примерно:
    ///
    /// interpreter.add_instructions(&[
    ///     "PUSH 1",
    ///     "PUSH 2",
    ///     "ADD",
    /// ]);
    ///
    /// Инструкциите не се интерпретират, само се записват.
    ///
    pub fn add_instructions(&mut self, instructions: &[&str]) {
        for instruction in instructions {
            self.instructions.push_back(instruction.to_string());
        }
    }

    /// Връща mutable reference към инструкцията, която ще се изпълни при
    /// следващия `.forward()` -- първата в списъка/дека.
    ///
    pub fn current_instruction(&mut self) -> Option<&mut String> {
        return self.instructions.front_mut();
    }

    /// Интерпретира първата инструкция в `self.instructions` по правилата описани по-горе. Записва
    /// някаква информация за да може успешно да се изпълни `.back()` в по-нататъшен момент.
    ///
    /// Ако няма инструкции, връща `RuntimeError::NoInstructions`. Другите грешки идват от
    /// обясненията по-горе.
    ///
    pub fn forward(&mut self) -> Result<(), RuntimeError> {
        let next_instruction = self.current_instruction();

        if next_instruction == None {
            return Err(RuntimeError::NoInstructions);
        }


        let instruction_parts: Vec<&str> = next_instruction.unwrap().split_whitespace().collect();

        if instruction_parts.len() == 0 {
            return Err(RuntimeError::InvalidCommand);
        }

        return match instruction_parts[0] {
            "PUSH" => {
                if instruction_parts.len() != 2 {
                    return Err(RuntimeError::InvalidCommand);
                }

                let num = instruction_parts[1].parse::<i32>();
                let num = match num {
                    Ok(num) => num,
                    Err(_) => return Err(RuntimeError::InvalidCommand),
                };

                self.stack.push(num);
                self.memory_instructions.push("PUSH ".to_string() + &num.to_string());
                self.instructions.drain(0..1);
                self.reverse_instructions.push(vec!["POP".to_string()]);

                Ok(())
            },
            "POP" => {
                if instruction_parts.len() != 1 {
                    return Err(RuntimeError::InvalidCommand);
                }

                let head = self.stack.pop();

                if head == None {
                    return Err(RuntimeError::StackUnderflow);
                }

                self.memory_instructions.push("POP".to_string());
                self.instructions.drain(0..1);
                self.reverse_instructions.push(vec!["PUSH ".to_string() + &head.unwrap().to_string()]);

                Ok(())
            },
            "ADD" => {
                if instruction_parts.len() != 1 {
                    return Err(RuntimeError::InvalidCommand);
                }

                let a = self.stack.pop();
                let b = self.stack.pop();

                if a == None || b == None {
                    return Err(RuntimeError::StackUnderflow);
                }

                self.stack.push(a.unwrap() + b.unwrap());
                self.memory_instructions.push("ADD".to_string());
                self.instructions.drain(0..1);
                self.reverse_instructions.push(vec![
                    "POP".to_string(),
                    "PUSH ".to_string() + &b.unwrap().to_string(),
                    "PUSH ".to_string() + &a.unwrap().to_string(),
                ]);

                Ok(())
            },
            "MUL" => {
                if instruction_parts.len() != 1 {
                    return Err(RuntimeError::InvalidCommand);
                }

                let a = self.stack.pop();
                let b = self.stack.pop();

                if a == None || b == None {
                    return Err(RuntimeError::StackUnderflow);
                }

                self.stack.push(a.unwrap() * b.unwrap());
                self.memory_instructions.push("MUL".to_string());
                self.instructions.drain(0..1);
                self.reverse_instructions.push(vec![
                    "POP".to_string(),
                    "PUSH ".to_string() + &b.unwrap().to_string(),
                    "PUSH ".to_string() + &a.unwrap().to_string(),
                ]);

                Ok(())
            },
            "SUB" => {
                if instruction_parts.len() != 1 {
                    return Err(RuntimeError::InvalidCommand);
                }

                let a = self.stack.pop();
                let b = self.stack.pop();

                if a == None || b == None {
                    return Err(RuntimeError::StackUnderflow);
                }

                self.stack.push(a.unwrap() - b.unwrap());
                self.memory_instructions.push("SUB".to_string());
                self.instructions.drain(0..1);
                self.reverse_instructions.push(vec![
                    "POP".to_string(),
                    "PUSH ".to_string() + &b.unwrap().to_string(),
                    "PUSH ".to_string() + &a.unwrap().to_string(),
                ]);

                Ok(())
            },
            "DIV" => {
                if instruction_parts.len() != 1 {
                    return Err(RuntimeError::InvalidCommand);
                }

                let a = self.stack.pop();
                let b = self.stack.pop();

                if a == None || b == None {
                    return Err(RuntimeError::StackUnderflow);
                }

                if b == Some(0) {
                    return Err(RuntimeError::DivideByZero);
                }

                self.stack.push(a.unwrap() / b.unwrap());
                self.memory_instructions.push("DIV".to_string());
                self.instructions.drain(0..1);
                self.reverse_instructions.push(vec![
                    "POP".to_string(),
                    "PUSH ".to_string() + &b.unwrap().to_string(),
                    "PUSH ".to_string() + &a.unwrap().to_string(),
                ]);


                Ok(())
            },
            _ => Err(RuntimeError::InvalidCommand),
        };
    }

    /// Вика `.forward()` докато не свършат инструкциите (може и да се имплементира по други
    /// начини, разбира се) или има грешка.
    ///
    pub fn run(&mut self) -> Result<(), RuntimeError> {
        loop {
            match self.forward() {
                Err(RuntimeError::NoInstructions) => return Ok(()),
                Err(e) => return Err(e),
                _ => (),
            }
        }
    }

    /// "Обръща" последно-изпълнената инструкция с `.forward()`. Това може да се изпълнява отново и
    /// отново за всяка инструкция, изпълнена с `.forward()` -- тоест, не пазим само последната
    /// инструкция, а списък/стек от всичките досега.
    ///
    /// Ако няма инструкция за връщане, очакваме `RuntimeError::NoInstructions`.
    ///
    pub fn back(&mut self) -> Result<(), RuntimeError> {
        let last_instruction = self.memory_instructions.pop();
        if last_instruction == None {
            return Err(RuntimeError::NoInstructions);
        }

        self.instructions.push_front(last_instruction.unwrap());

        let next_reverse_instruction_set = self.reverse_instructions.pop();
        if next_reverse_instruction_set == None {
            return Err(RuntimeError::NoInstructions);
        }

        let split = next_reverse_instruction_set.unwrap();
        for command in split {
            let instruction_parts: Vec<&str> = command.split_whitespace().collect();

            match instruction_parts[0] {
                "PUSH" => {
                    let num = instruction_parts[1].parse::<i32>();
                    let num = match num {
                        Ok(num) => num,
                        Err(_) => return Err(RuntimeError::InvalidCommand),
                    };

                    self.stack.push(num);
                },
                "POP" => {
                    self.stack.pop();
                },
                _ => return Err(RuntimeError::InvalidCommand),
            }
        }

        return Ok(());
    }
+
}


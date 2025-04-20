pub enum Operator {
    Add,
    Substract,
    Divide,
    Multiply,
}

pub struct Calculator {
    first_number: f64,
    second_number: f64,
    operator: Operator,
}

impl Calculator {
    pub fn new(first_number: f64, second_number: f64, operator: Operator) -> Self {
        Self {
            first_number,
            second_number,
            operator,
        }
    }

    pub fn calculate(&self) -> Result<f64, &'static str> {
        match self.operator {
            Operator::Add => Ok(self.first_number + self.second_number),
            Operator::Substract => Ok(self.first_number - self.second_number),
            Operator::Multiply => Ok(self.first_number * self.second_number),
            Operator::Divide => {
                if self.second_number == 0.0 {
                    Err("Division par zéro impossible")
                } else {
                    Ok(self.first_number / self.second_number)
                }
            }
        }
    }

    pub fn get_first_number(&self) -> f64 {
        self.first_number
    }

    pub fn get_second_number(&self) -> f64 {
        self.second_number
    }

    pub fn get_operator(&self) -> &str {
        match self.operator {
            Operator::Add => "+",
            Operator::Substract => "-",
            Operator::Multiply => "*",
            Operator::Divide => "/",
        }
    }
}

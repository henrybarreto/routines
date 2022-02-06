use crate::condition::Condition;

#[derive(Debug)]
pub struct Conditional<'a> {
    pub data: &'a str,
    pub condition: (Condition, &'a str),
}
impl<'a> Conditional<'a> {
    pub fn check(&self) -> bool {
        match self.condition.0 {
            Condition::Equal => self.data == self.condition.1,
            Condition::Great => {
                let data = self
                    .data
                    .parse::<i32>()
                    .expect("Could not convert from string to number");
                let condition = self
                    .condition
                    .1
                    .parse::<i32>()
                    .expect("Could not convert from string to number");

                data > condition
            }
            Condition::Less => {
                let data = self
                    .data
                    .parse::<i32>()
                    .expect("Could not convert from string to number");
                let condition = self
                    .condition
                    .1
                    .parse::<i32>()
                    .expect("Could not convert from string to number");

                data < condition
            }
        }
    }
}

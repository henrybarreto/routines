use crate::conditions::Conditions;

#[derive(Debug)]
pub struct Conditional {
    pub data: String,
    pub condition: (Conditions, String),
}
impl Conditional {
    pub fn check(&self) -> bool {
        match self.condition.0 {
            Conditions::Equal => self.data== self.condition.1,
            Conditions::Great => {
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
            Conditions::Less => {
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
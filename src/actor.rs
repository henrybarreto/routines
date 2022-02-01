use std::process::Command;

pub struct Actor {
    pub path: String,
}

impl Actor {
    fn check_outputs(mut list: Vec<i32>) -> i32 {
        let head: Option<&i32> = list.get(0);
        if head.is_none(){
            return 0;
        }

        let head = *head.unwrap();
        list.remove(0);

         if head != 0 {
            Self::check_outputs(list)
        } else {
            255
        }
    }

    fn run(&self, func: &str) -> Option<i32> {
        let output = Command::new("bash")
            .arg(&self.path)
            .arg(func)
            .output()
            .expect("Could not execute a command");

        output.status.code()
    }

    fn get_code_from_run(option: Option<i32>) -> i32 {
         if let Some(code) = option {
            if code != 0 {
                code
            } else {
                0
            }
        } else {
            255
        }
    }

    fn before(&self) -> Option<i32> {
        self.run("before")
    }
    fn after(&self) -> Option<i32> {
        self.run("after")
    }

    pub fn up(&self) -> i32 {
         Self::check_outputs(Vec::from([
            Self::get_code_from_run(self.before()),
            Self::get_code_from_run(self.run("up")),
            Self::get_code_from_run(self.after()),
        ]))
    }
    pub fn down(&self) -> i32 {
         Self::check_outputs(Vec::from([
            Self::get_code_from_run(self.before()),
            Self::get_code_from_run(self.run("down")),
            Self::get_code_from_run(self.after()),
        ]))
    }
}
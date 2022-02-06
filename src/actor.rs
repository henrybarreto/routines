use logs::trace;
use std::process::Command;

pub struct Actor<'a> {
    pub path: &'a str,
}

impl<'a> Actor<'a> {
    pub fn new(path: &'a str) -> Self {
        return Actor { path };
    }

    fn check_outputs(mut list: Vec<i32>) -> i32 {
        let head: Option<&i32> = list.get(0);
        if head.is_none() {
            return 0;
        }

        let head = *head.unwrap();
        list.remove(0);

        if head == 0 {
            Self::check_outputs(list)
        } else {
            255
        }
    }

    fn run(&self, func: &str) -> Option<i32> {
        let output = Command::new("sh")
            .arg(&self.path)
            .arg(func)
            .output()
            .expect("Could not execute a command");

        // debug!(format!("the exit code from the run is: {}", output.status.code().expect("Could not get the exit status from action script")));
        // debug!(String::from_utf8_lossy(&output.stdout));
        // debug!(String::from_utf8_lossy(&output.stderr));

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
        trace!("running before function...");
        self.run("before")
    }
    fn after(&self) -> Option<i32> {
        trace!("running after function...");
        self.run("after")
    }

    pub fn up(&self) -> i32 {
        trace!("running up function");
        Self::check_outputs(Vec::from([
            Self::get_code_from_run(self.before()),
            Self::get_code_from_run(self.run("up")),
            Self::get_code_from_run(self.after()),
        ]))
    }
    pub fn down(&self) -> i32 {
        trace!("running down function");
        Self::check_outputs(Vec::from([
            Self::get_code_from_run(self.before()),
            Self::get_code_from_run(self.run("down")),
            Self::get_code_from_run(self.after()),
        ]))
    }
}

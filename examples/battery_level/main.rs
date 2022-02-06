use std::env;
use logs::{error, trace, warn};
use routines::actor::Actor;
use routines::condition::Condition;
use routines::conditional::Conditional;
use routines::monitor::Monitor;
use std::sync::{mpsc, Arc};

fn main() {
    let args: Vec<String> = env::args().collect();
    let default_path = &String::from("examples/battery_level/battery_save.sh");
    let path = args.get(1).unwrap_or(default_path);

    let monitor = Monitor::new(5);
    let (sender, receiver): (mpsc::Sender<Arc<str>>, mpsc::Receiver<Arc<str>>) = mpsc::channel();
    let actor = Actor::new(path);
    let mut state: bool = false;

    monitor.battery_level(sender.clone());
    loop {
        let pointer: Arc<str> = receiver
            .recv()
            .expect("Could not receive data from the thread");
        let data = pointer
            .get(..)
            .expect("Could not get the data from the pointer");
        let conditional = Conditional {
            data: &data,
            condition: (Condition::Less, "100"),
        };

        if conditional.check() {
            trace!("condition is true");
            if state == false {
                trace!("state is false");
                trace!("up...");
                let result = actor.up();
                if result != 0 {
                    error!("up function returned a not equal to zero value");
                    warn!("retrying to run up function");
                } else {
                    state = true;
                }
                trace!("{} is the exit from the up function on script", result);
            }
        } else {
            trace!("condition is false");
            if state == true {
                trace!("state is true");
                trace!("downing...");
                let result = actor.down();
                if result != 0 {
                    error!("down function returned a not equal to zero value");
                    warn!("retrying to run down function");
                } else {
                    state = false;
                }
                trace!("{} is the exit from the down function on script", result);
            }
        }
    }
}

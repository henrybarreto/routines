use routines::conditional::Conditional;
use routines::conditions::Conditions;
use routines::monitor::Monitor;

use std::borrow::Borrow;
use std::sync::mpsc;

fn main() {
    let monitor = Monitor::new(1);

    let (tf, rf): (mpsc::Sender<String>, mpsc::Receiver<String>) = mpsc::channel();
    let (tb, rb): (mpsc::Sender<String>, mpsc::Receiver<String>) = mpsc::channel();

    let _frequency = monitor.cpu_frequency(tf.clone());
    let _battery = monitor.battery_level(tb.clone());

    let check = |name: String, params: (&mpsc::Receiver<String>, Conditions, String)| {
        let channel = params.0;
        let signal = params.1;
        let conditional = params.2;

        let data = channel.recv().expect("Could not receive the data");
        let event = Conditional {
            data: data.clone(),
            condition: (signal, conditional.clone()),
        };

        if event.check() {
            println!("{}", name);
            println!("SIM");
            println!("{}", data);
            println!("----------")
        } else {
            println!("{}", name);
            println!("NAO");
            println!("{}", data);
            println!("----------")
        }
    };

    loop {
        check(
            "cpu_frequency".to_string(),
            (rf.borrow(), Conditions::Great, "770000".to_string()),
        );
        check(
            "battery".to_string(),
            (rb.borrow(), Conditions::Less, "100".to_string()),
        );
    }
}

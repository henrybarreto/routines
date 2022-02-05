use routines::conditional::Conditional;
use routines::condition::Condition;
use routines::monitor::Monitor;
use routines::event::Event;

use std::borrow::Borrow;
use std::sync::mpsc;

fn main() {
    let monitor: Monitor = Monitor::new(1);

    let events: Vec<Event> = Event::load().expect("Could not get the event configuration file");

    let mut channels: Vec<mpsc::Receiver<String>> = Vec::<mpsc::Receiver<String>>::new();
    for index in 0..events.len() {
        let (tx, rx): (mpsc::Sender<String>, mpsc::Receiver<String>) = mpsc::channel();
        monitor.get_monitor_from_str(&events[index].clone().name, tx.clone());
        channels.push(rx)
    }

    let check = |name: String, params: (&mpsc::Receiver<String>, Condition, String)| {
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
            println!("TRUE");
            println!("{}", data);
            println!("----------")
        } else {
            println!("{}", name);
            println!("FALSE");
            println!("{}", data);
            println!("----------")
        }
    };

    loop {
        for index in 0..events.len() {
            check(
                events[index].clone().name,
                (
                    channels[index].borrow(),
                    events[index].clone().condition,
                    events[index].clone().value,
                ),
            );
        }
    }
}

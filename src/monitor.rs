use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::sync::mpsc;
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

#[derive(Debug)]
pub struct Monitor {
    pub interval: u64,
}
impl Monitor {
    const CPU_GOVERNOR_FILE_PATH: &'static str =
        "/sys/devices/system/cpu/cpu0/cpufreq/scaling_governor";
    const CPU_FREQUENCY_FILE_PATH: &'static str =
        "/sys/devices/system/cpu/cpufreq/policy0/scaling_cur_freq";
    const BATTERY_LEVEL_FILE_PATH: &'static str = "/sys/class/power_supply/BAT0/capacity";

    pub fn new(interval: u64) -> Self {
        Monitor { interval }
    }

    fn monitor<F>(path: &'static str, interval: u64, closure: F) -> JoinHandle<()>
    where
        F: Fn(Vec<u8>),
        F: Send + 'static,
    {
        thread::spawn(move || loop {
            let mut file =
                File::open(Path::new(path)).expect(&format!("Could not open the file: {}", path));
            let mut data = Vec::new();
            let _ = file
                .read_to_end(&mut data)
                .expect("Could read the file to the end");

            closure(data);

            thread::sleep(Duration::from_secs(interval));
        })
    }

    fn common(data: Vec<u8>, transport: mpsc::Sender<String>) {
        let mut string = String::from_utf8_lossy(&data).to_string();
        string = string.replace("\n", "");
        transport
            .send(string)
            .expect("Could not send the data through the channel");
    }

    pub fn cpu_governor(&self, transport: mpsc::Sender<String>) -> JoinHandle<()> {
        Self::monitor(
            Self::CPU_GOVERNOR_FILE_PATH,
            self.interval,
            move |data| Self::common(data, transport.clone()),
        )
    }
    pub fn cpu_frequency(&self, transport: mpsc::Sender<String>) -> JoinHandle<()> {
        Self::monitor(
            Self::CPU_FREQUENCY_FILE_PATH,
            self.interval,
            move |data| Self::common(data, transport.clone()),
        )
    }
    pub fn battery_level(&self, transport: mpsc::Sender<String>) -> JoinHandle<()> {
        Self::monitor(
            Self::BATTERY_LEVEL_FILE_PATH,
            self.interval,
            move |data| Self::common(data, transport.clone()),
        )
    }
}

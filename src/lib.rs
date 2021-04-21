use log::{info, warn};
use signal_device::SignalDevice;
use std::collections::HashMap;

pub struct SignalScanner {
    thread_text: String,
    devices: HashMap<String, SignalDevice>,
}

impl SignalScanner {
    pub fn new(thread_text: String) -> SignalScanner {
        info!("Creating signal scanner thread: {}", thread_text);

        SignalScanner {
            thread_text,
            devices: HashMap::new(),
        }
    }

    pub fn get_thread_text(&self) -> &String {
        &self.thread_text
    }
    pub fn register_device(&mut self, device_name: String, device: SignalDevice) {
        info!("Registering device.");

        self.devices.insert(device_name, device);
    }

    pub fn get_device(&self, device_name: &str) -> Option<&SignalDevice> {
        self.devices.get(device_name)
    }
    pub fn get_device_mut(&mut self, device_name: &str) -> Option<&mut SignalDevice> {
        self.devices.get_mut(device_name)
    }
}

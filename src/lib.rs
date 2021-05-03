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

    // Reaches into the registered devices and returns the value of the signal
    // based on what it was last time the scanner updated
    pub fn get_device_signal_status(
        &mut self,
        device_name: &String,
        signal_name: &String,
    ) -> Result<bool, String> {
        Ok(self
            .get_device_mut(device_name)
            .expect(format!("Unable to find device: {}", device_name).as_str())
            .get_signal(signal_name)
            .expect(format!("Unable to find signal: {}", signal_name).as_str())
            .get_signal_status()
            .clone())
    }

    // For each of the signals contained within each of the devices, execute a direct
    // query of its register to get a current value of its status
    pub fn refresh_signals(&mut self) -> Result<(), String> {
        for device in self.devices.iter_mut() {
            device.1.refresh_signals();
        }

        Ok(())
    }
}

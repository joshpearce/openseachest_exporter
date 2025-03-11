
use regex::Regex;


pub const PRE_FAIL_WARRANTY: u8 = 0x01;
pub const ONLINE_COLLECTION: u8 = 0x02;
pub const DECREASE_MEANS_DEGRADE: u8 = 0x04;
pub const ERROR_RATE: u8 = 0x08;
pub const EVENT_COUNT: u8 = 0x10;
pub const SAVED_ACROSS_POWER_CYCLES: u8 = 0x20;

#[derive(Debug, Clone)]
pub struct SmartAttribute {
    pub currently_failing: Option<bool>,
    pub previously_failed: Option<bool>,
    pub id: u8,
    pub name: String,
    pub status_flags: u8,
    pub normalized_value: u8,
    pub worst_normalized_value: u8,
    pub threshold_normalized_value: u8,
    pub warranty: bool,
    pub online_collection: bool,
    pub decrease_means_degrade: bool,
    pub error_rate: bool,
    pub event_count: bool,
    pub saved_on_power_cycle: bool,
    pub vendor: String,
    pub handle: String,
    pub model_number: String,
    pub serial_number: String,
    pub firmware_revision: String,
    pub raw_value: u64,
}

impl SmartAttribute {
    pub fn get_metric_name(&self) -> String {
        let re = Regex::new(r"[^a-z]").unwrap();
        let name = self.name.to_lowercase();
        let simple_name = re.replace_all(&name, "_");
        format!("seagate_{}", simple_name)
    }
}

#[derive(Debug, Clone)]
pub struct SmartDeviceInfo {
    pub vendor: String,
    pub handle: String,
    pub model_number: String,
    pub serial_number: String,
    pub firmware_revision: String,
}
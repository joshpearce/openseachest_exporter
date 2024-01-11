use crate::smart_data::{
    SmartAttribute, SmartDeviceInfo, DECREASE_MEANS_DEGRADE, ERROR_RATE, EVENT_COUNT,
    ONLINE_COLLECTION, PRE_FAIL_WARRANTY, SAVED_ACROSS_POWER_CYCLES,
};
use regex::Regex;
use std::collections::HashMap;
use std::error::Error;

pub fn parse_smart_scan(buffer: Vec<u8>) -> Result<Vec<SmartDeviceInfo>, Box<dyn Error>> {
    let header_regex = Regex::new(
        r"^(?<vendor>Vendor)\s+(?<handle>Handle)\s+(?<model>Model Number)\s+(?<serial>Serial Number)\s+(?<firmware>FwRev)"
    ).unwrap();

    let mut header_cols: HashMap<&str, (usize, usize)> = HashMap::new();
    let mut results: Vec<SmartDeviceInfo> = Vec::new();

    for line in String::from_utf8_lossy(&buffer).lines() {
        if header_cols.len() == 0 {
            let mut locs = header_regex.capture_locations();
            let captures = header_regex.captures_read(&mut locs, line);
            match captures {
                None => {
                    if line.contains("WARNING") {
                        return Err(line.into());
                    }
                }
                Some(_) => {
                    header_cols.insert("vendor", locs.get(1).unwrap());
                    header_cols.insert("handle", locs.get(2).unwrap());
                    header_cols.insert("model", locs.get(3).unwrap());
                    header_cols.insert("serial", locs.get(4).unwrap());
                    header_cols.insert("firmware", locs.get(5).unwrap());
                }
            }
        } else {
            if line.len() >= header_cols["firmware"].0 {
                results.push(SmartDeviceInfo {
                    vendor: line[header_cols["vendor"].0..header_cols["handle"].0 - 1]
                        .trim()
                        .to_string(),
                    handle: line[header_cols["handle"].0..header_cols["model"].0 - 1]
                        .trim()
                        .to_string(),
                    model_number: line[header_cols["model"].0..header_cols["serial"].0 - 1]
                        .trim()
                        .to_string(),
                    serial_number: line[header_cols["serial"].0..header_cols["firmware"].0 - 1]
                        .trim()
                        .to_string(),
                    firmware_revision: line[header_cols["firmware"].0..].trim().to_string(),
                });
            }
        }
    }
    if results.len() == 0 {
        Err("No device results found".into())
    } else {
        Result::Ok(results)
    }
}

pub fn parse_smart_attributes(buffer: Vec<u8>, device: &SmartDeviceInfo) -> Result<Vec<SmartAttribute>, Box<dyn Error>> {
    let attrs_regex = Regex::new(
        r"^(?<alerts>[^\d]+)(?<id>\d+)\s+(?<name>[^0]+)(?<status>[0-9A-F]+)h\s+(?<current>[0-9A-F]+)h\s+(?<worst>[0-9A-F]+)h\s+(?<thresh>[0-9A-F]+)h\s+(?<raw>[0-9A-F]+)h"
    ).unwrap();

    let mut results: Vec<SmartAttribute> = Vec::new();
    let mut start_parsing = false;

    for line in String::from_utf8_lossy(&buffer).lines() {
        if !start_parsing {
            start_parsing = line.contains("Attribute Name:");
        } else {
            let captures = attrs_regex.captures(line);
            match captures {
                None => {}
                Some(caps) => {
                    let status_mask: u8 =
                        u8::from_str_radix(caps["status"].to_string().trim(), 16)?;
                    let current: u8 = u8::from_str_radix(caps["current"].to_string().trim(), 16)?;
                    let worst: u8 = u8::from_str_radix(caps["worst"].to_string().trim(), 16)?;
                    let thresh: u8 = u8::from_str_radix(caps["thresh"].to_string().trim(), 16)?;

                    let (currently_failing, previously_failed): (Option<bool>, Option<bool>) =
                        match status_mask & PRE_FAIL_WARRANTY {
                            PRE_FAIL_WARRANTY => match thresh {
                                0 => (Some(false), Some(false)),
                                _ => (Some(current.lt(&thresh)), Some(worst.lt(&thresh))),
                            },
                            _ => (None, None),
                        };

                    results.push(SmartAttribute {
                        currently_failing: currently_failing,
                        previously_failed: previously_failed,
                        id: u8::from_str_radix(caps["id"].to_string().trim(), 10)?,
                        name: caps["name"].trim().to_string(),
                        status_flags: status_mask,
                        normalized_value: current,
                        worst_normalized_value: worst,
                        threshold_normalized_value: thresh,
                        decrease_means_degrade: status_mask & DECREASE_MEANS_DEGRADE > 0,
                        event_count: status_mask & EVENT_COUNT > 0,
                        error_rate: status_mask & ERROR_RATE > 0,
                        online_collection: status_mask & ONLINE_COLLECTION > 0,
                        saved_on_power_cycle: status_mask & SAVED_ACROSS_POWER_CYCLES > 0,
                        warranty: status_mask & PRE_FAIL_WARRANTY > 0,
                        handle: device.handle.clone(),
                        serial_number: device.serial_number.clone(),
                        firmware_revision: device.firmware_revision.clone(),
                        model_number: device.model_number.clone(),
                        vendor: device.vendor.clone(),
                        raw_value: u64::from_str_radix(caps["raw"].to_string().trim(), 16)?,
                    });
                }
            }
        }
    }

    Result::Ok(results)
}


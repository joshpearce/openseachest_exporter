
mod common;
use openseachest_exporter::smart_parsers::{parse_smart_scan};

#[test]
fn it_adds_two() {
    assert_eq!(4, 4);
}

#[test]
fn parse_scan() {
    let buf = common::read_text_resource("resources/test/openSeaChest_SMART.scan.txt");
    let info = parse_smart_scan(buf).unwrap();
    assert_eq!(info.first().unwrap().vendor, "ATA");
}


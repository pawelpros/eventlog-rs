use std::fs;
extern crate eventlog_rs;
use base64::{engine::general_purpose::STANDARD, Engine};
use std::convert::TryFrom;
use eventlog_rs::rtmr::Rtmr;

fn main() {
    env_logger::builder()
        .filter(None, log::LevelFilter::Info)
        .init();

    from_file("/home/pproskur/workspace/coco/eventlog-rs/cmd/tdxmeasure/CCEL_data_ovmf".to_string());
    from_file("/home/pproskur/workspace/coco/eventlog-rs/cmd/tdxmeasure/CCEL_data_grub".to_string());
    from_file("/home/pproskur/workspace/coco/eventlog-rs/cmd/tdxmeasure/CCEL_data".to_string());
    from_file("/home/pproskur/workspace/coco/eventlog-rs/cmd/tdxmeasure/ccel_test2.bin".to_string());

    from_base64("/home/pproskur/workspace/coco/eventlog-rs/cmd/tdxmeasure/gke_ccel.b64".to_string());
    from_base64("/home/pproskur/workspace/coco/eventlog-rs/cmd/tdxmeasure/gcp_ccel.b64".to_string());
    from_base64("/home/pproskur/workspace/coco/eventlog-rs/cmd/tdxmeasure/gke_ccel_14052025.b64".to_string());
}

fn from_base64(path: String) {
    println!("{}", path);
    let data = fs::read(path).unwrap();

    let evidence = STANDARD.decode(&data).unwrap();

    let event_log = eventlog_rs::Eventlog::try_from(evidence).unwrap();
    let rtmrs = Rtmr::try_from(event_log.clone()).unwrap();

    let json = serde_json::to_string_pretty(&event_log).unwrap();
    println!("{}", json);
    println!("{}", rtmrs);
}

fn from_file(path: String) {
    println!("{}", path);
    let data = fs::read(path).unwrap();

    let event_log = eventlog_rs::Eventlog::try_from(data).unwrap();
    let rtmrs = Rtmr::try_from(event_log.clone()).unwrap();
    let json = serde_json::to_string_pretty(&event_log).unwrap();
    println!("{}", json);
    println!("{}", rtmrs);
}

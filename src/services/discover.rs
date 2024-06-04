use futures_util::StreamExt;
use mdns::{Record, RecordKind};
use std::future::Future;
use std::pin::Pin;
use std::time::Duration;

// _http._tcp.local
const SERVICE_NAME: &str = "_airplay._tcp.local";

#[derive(Debug)]
pub struct DeviceInfo {
    pub box_model: String,
}

pub trait Device {
    fn get_info(&self, vec: Vec<String>) -> Option<DeviceInfo>;
}

pub trait Discover {
    type DeviceType: Device;
    async fn discover_device(
        &self,
        duration: Duration,
    ) -> Pin<Box<dyn Future<Output = Vec<Self::DeviceType>>>>;
}

pub struct FreeboxDevice;

impl Device for FreeboxDevice {
    fn get_info(&self, _: Vec<String>) -> Option<DeviceInfo> {
        Some(DeviceInfo {
            box_model: "Freebox Delta".to_string(),
        })
    }
}

pub struct FreeboxDiscover;

impl Discover for FreeboxDiscover {
    type DeviceType = FreeboxDevice;

    async fn discover_device(
        &self,
        duration: Duration,
    ) -> Pin<Box<dyn Future<Output = Vec<Self::DeviceType>>>> {
        Box::pin(async move {
            let stream = mdns::discover::all(SERVICE_NAME, duration)
                .unwrap()
                .listen();
            let _a = stream
                .all(|x| async {
                    if let Ok(response) = x {
                        let _addr = response.records().filter_map(to_ip_addr).next();
                        println!("abc");
                        true
                    } else {
                        false
                    }
                })
                .await; // keep the stream alive
            vec![FreeboxDevice]
        })
    }
}

fn to_ip_addr(record: &Record) -> Option<()> {
    match &record.kind {
        RecordKind::A(_addr) => (),
        RecordKind::AAAA(_addr) => (),
        RecordKind::TXT(x) => {
            println!("TXT {:?}", x);
        }
        RecordKind::SRV {
            port,
            target,
            weight,
            priority,
        } => {
            println!("SRV {:?}", target);
            println!("SRV {:?}", weight);
            println!("SRV {:?}", port);
            println!("SRV {:?}", priority);
        }
        RecordKind::NS(name) => println!("NS {:?}", name),
        RecordKind::MX {
            preference,
            exchange,
        } => {
            println!("MX {:?}", preference);
            println!("MX {:?}", exchange);
        }
        RecordKind::CNAME(name) => println!("CNAME {:?}", name),
        RecordKind::PTR(name) => println!("PTR {:?}", name),
        _ => (),
    };
    Some(())
}

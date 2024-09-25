use crate::{idl::Idl, util::idl_type_to_rust_type};
use askama::Template;
use heck::{ToSnekCase, ToUpperCamelCase};
use sha2::{Digest, Sha256};

#[allow(dead_code)]
#[derive(Debug)]
pub struct EventData {
    pub struct_name: String,
    pub module_name: String,
    pub discriminator: String,
    pub args: Vec<ArgumentData>,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct ArgumentData {
    pub name: String,
    pub rust_type: String,
}

#[derive(Template)]
#[template(path = "events_struct.askama", escape = "none", ext = ".askama")]
pub struct EventsStructTemplate<'a> {
    pub event: &'a EventData,
}

pub fn process_events(idl: &Idl) -> Vec<EventData> {
    let mut events_data = Vec::new();

    for event in &idl.events {
        let module_name = event.name.to_snek_case();
        let struct_name = event.name.to_upper_camel_case();
        let discriminator = compute_event_discriminator(&event.name);

        let mut args = Vec::new();
        for field in &event.fields {
            let rust_type = idl_type_to_rust_type(&field.type_);
            args.push(ArgumentData {
                name: field.name.to_snek_case(),
                rust_type,
            });
        }

        events_data.push(EventData {
            struct_name,
            module_name,
            discriminator,
            args,
        });
    }

    events_data
}

fn compute_event_discriminator(event_name: &str) -> String {
    let mut hasher = Sha256::new();
    let discriminator_input = format!("event:{}", event_name);
    hasher.update(discriminator_input.as_bytes());
    let hash = hasher.finalize();
    let discriminator_bytes = &hash[..8];
    format!("0xe445a52e51cb9a1d{}", hex::encode(discriminator_bytes))
}

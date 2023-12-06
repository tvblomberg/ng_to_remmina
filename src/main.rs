use std::{
    fs::{self},
    path::Path,
};

use clap::Parser;
use clap_derive::Parser;
use file::file_writer::FileWriter;
use ng::{
    models::{Connections, RDPConnection},
    ng_mapper::map_rdp_connections,
};
use remmina::config_writer::{ConfigWriter, ConfigWriterTrait};
use serde_xml_rs::from_str;

mod file;
mod ng;
mod remmina;

#[derive(Parser, Default, Debug)]
#[clap(
    author = "Thomas V. Blomberg",
    version,
    about = "Simple program to convert mRemoteNG RDP connections to Remmina"
)]
struct AppConfig {
    #[clap(short, long)]
    // Location of the mRemoteNG XMl Config File
    config: String,
    #[clap(short, long)]
    // Location of where the Remmina config files should be created
    destination: String,
}

fn main() -> Result<(), String> {
    let (ng_file_string, destination_string) = parse_app_args()?;
    let ng_file = Path::new(&ng_file_string);
    let destination = Path::new(&destination_string);
    let xml_content = read_xml_content(ng_file)?;
    let ng_connections = deserialize_connections(&xml_content)?;
    let rdp_connections = create_rdp_connections(&ng_connections)?;

    for connection in rdp_connections {
        let config_writer = ConfigWriter;
        let mut file_writer = FileWriter;

        match config_writer.write_config(&mut file_writer, destination, &connection) {
            Ok(_) => println!("Successfully processed {}", connection.name),
            Err(e) => eprintln!("Failed to process connection {}: {}", connection.name, e),
        };
    }

    Ok(())
}

fn create_rdp_connections(ng_connections: &Connections) -> Result<Vec<RDPConnection>, String> {
    match map_rdp_connections(ng_connections) {
        Ok(connections) => Ok(connections),
        Err(s) => Err(format!("Failed to create rdp connections: {}", s)),
    }
}

fn parse_app_args() -> Result<(String, String), String> {
    let args = AppConfig::parse();

    let ng_file_string = args.config;
    let destination_string = args.destination;

    Ok((ng_file_string.to_string(), destination_string.to_string()))
}

fn deserialize_connections(xml_content: &str) -> Result<Connections, String> {
    match from_str(xml_content) {
        Ok(connections) => Ok(connections),
        Err(e) => Err(format!(
            "Failed to deserialize NG XML to Connections: {}",
            e
        )),
    }
}

fn read_xml_content(file_location: &Path) -> Result<String, String> {
    match fs::read_to_string(file_location) {
        Ok(contents) => Ok(contents),
        Err(e) => Err(format!("Error reading NG XML file: {}", e)),
    }
}

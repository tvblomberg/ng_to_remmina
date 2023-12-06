use crate::{file::file_writer::FileWriterTrait, ng::models::RDPConnection};
use std::path::Path;

use super::{name_formatter::format_name, template::template};

pub trait ConfigWriterTrait {
    fn write_config<T: FileWriterTrait>(
        &self,
        writer: &mut T,
        destination: &Path,
        connection: &RDPConnection,
    ) -> Result<(), String>;
}

pub struct ConfigWriter;

impl ConfigWriterTrait for ConfigWriter {
    fn write_config<T: FileWriterTrait>(
        &self,
        writer: &mut T,
        destination: &Path,
        connection: &RDPConnection,
    ) -> Result<(), String> {
        let file_name = format_name(connection);
        let file_path = Path::join(destination, file_name);

        let content = template(connection)?;

        writer.write_to_destination(&file_path, &content)
    }
}

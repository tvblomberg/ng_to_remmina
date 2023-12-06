#[cfg(test)]
mod config_test {
    use std::path::Path;

    use mockall::predicate::eq;

    use crate::{
        file::file_writer::MockFileWriterTrait,
        ng::models::RDPConnection,
        remmina::{
            config_writer::{ConfigWriter, ConfigWriterTrait},
            name_formatter::format_name,
            template::template,
        },
    };

    #[test]
    fn write_config_should_callfilewriter() {
        // Arrange
        let sut = ConfigWriter;
        let mut mock = MockFileWriterTrait::new();

        let expected_connection = RDPConnection {
            name: "somename".to_string(),
            description: None,
            hostname: "somehostname".to_string(),
            username: None,
            group: None,
            domain: "somedomain".to_string(),
        };

        let expected_file_name = format_name(&expected_connection);
        let expected_template_content = template(&expected_connection).unwrap_or_default();
        let destination = Path::new("/");
        let expected_destination = Path::join(destination, expected_file_name);

        mock.expect_write_to_destination()
            .with(eq(expected_destination), eq(expected_template_content))
            .times(1)
            .returning(|_, _| Ok(()));

        // Act
        let result = sut.write_config(&mut mock, destination, &expected_connection);

        // Assert
        assert!(result.is_ok());
    }

    #[test]
    fn write_config_error_should_returnerror() {
        // Arrange
        let sut = ConfigWriter;
        let mut mock = MockFileWriterTrait::new();

        let expected_connection = RDPConnection {
            name: "somename".to_string(),
            description: None,
            hostname: "somehostname".to_string(),
            username: None,
            group: None,
            domain: "somedomain".to_string(),
        };

        let destination = Path::new("/");

        mock.expect_write_to_destination()
            .times(1)
            .returning(|_, _| Err("ope".to_string()));

        // Act
        let result = sut.write_config(&mut mock, destination, &expected_connection);

        // Assert
        assert!(result.is_err());
    }
}

#[cfg(test)]
mod formatter_test {
    use crate::{ng::models::RDPConnection, remmina::name_formatter::format_name};

    #[test]
    fn format_name_nogroup_should_defaulttogroup() {
        // Arrange
        let expected_connection = RDPConnection {
            name: "somename".to_string(),
            description: None,
            hostname: "somehostname".to_string(),
            username: None,
            group: None,
            domain: "somedomain".to_string(),
        };

        let expected_file_name = "group_rdp_somename_somehostname.remmina";

        // Act
        let result = format_name(&expected_connection);

        // Assert
        assert_eq!(result, expected_file_name);
    }

    #[test]
    fn format_name_uppercase_should_setnametolowercase() {
        // Arrange
        let expected_connection = RDPConnection {
            name: "soMename".to_string(),
            description: None,
            hostname: "someHostname".to_string(),
            username: None,
            group: Some("someGroup".to_string()),
            domain: "somedomain".to_string(),
        };

        let expected_file_name = "somegroup_rdp_somename_somehostname.remmina";

        // Act
        let result = format_name(&expected_connection);

        // Assert
        assert_eq!(result, expected_file_name);
    }

    #[test]
    fn format_name_invalidchars_should_replacewithvalidchar() {
        // Arrange
        let invalid_chars = "\\%|/$?<>:*. \"";
        let expected_connection = RDPConnection {
            name: format!("{}soMename", invalid_chars),
            description: None,
            hostname: format!("{}someHostname", invalid_chars),
            username: None,
            group: Some(format!("{}someGroup", invalid_chars)),
            domain: "somedomain".to_string(),
        };
        let expected_chars = "-------------";

        let expected_file_name = format!(
            "{}somegroup_rdp_{}somename_{}somehostname.remmina",
            expected_chars, expected_chars, expected_chars
        );

        // Act
        let result = format_name(&expected_connection);

        // Assert
        assert_eq!(result, expected_file_name);
    }
}

#[cfg(test)]
mod tests {
    use crate::{ng::models::RDPConnection, remmina::template::template};

    #[test]
    fn template_withcontext_should_setrdpinformation() {
        // Arrange
        let expected_connection = RDPConnection {
            name: "some name".to_string(),
            description: Some("some description".to_string()),
            hostname: "some hostname".to_string(),
            username: Some("some username".to_string()),
            group: Some("some group".to_string()),
            domain: "some domain".to_string(),
        };

        // Act
        let result = template(&expected_connection);

        // Assert
        assert!(result.is_ok());
        let actual_val = result.unwrap();
        let formatted_name = format!("name={}", expected_connection.name);
        let formatted_username = format!("username={}", expected_connection.username.unwrap());
        let formatted_description = format!("labels={}", expected_connection.description.unwrap());
        let formatted_hostname = format!("server={}", expected_connection.hostname);
        let formatted_group = format!("group={}", expected_connection.group.unwrap());
        let formatted_domain = format!("domain={}", expected_connection.domain);

        assert!(actual_val.contains(&formatted_name));
        assert!(actual_val.contains(&formatted_username));
        assert!(actual_val.contains(&formatted_description));
        assert!(actual_val.contains(&formatted_hostname));
        assert!(actual_val.contains(&formatted_group));
        assert!(actual_val.contains(&formatted_domain));
    }

    #[test]
    fn template_withoptionals_should_setrdpinformation() {
        // Arrange
        let expected_connection = RDPConnection {
            name: "some name".to_string(),
            description: None,
            hostname: "some hostname".to_string(),
            username: None,
            group: None,
            domain: "some domain".to_string(),
        };

        // Act
        let result = template(&expected_connection);

        // Assert
        assert!(result.is_ok());
        let actual_val = result.unwrap();
        let formatted_hostname = format!("server={}", expected_connection.hostname);
        let formatted_domain = format!("domain={}", expected_connection.domain);

        assert!(actual_val.contains(&formatted_hostname));
        assert!(actual_val.contains(&formatted_domain));
    }
}

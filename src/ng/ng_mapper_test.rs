#[cfg(test)]
mod tests {
    use crate::ng::models::{Connections, Node, NodeType};
    use crate::ng::ng_mapper::map_rdp_connections;

    #[test]
    fn map_rdp_connections_noconnections_should_returnemptyvec() {
        // Arrange
        let expected_connections = Connections {
            connections: Vec::new(),
        };

        // Act
        let result = map_rdp_connections(&expected_connections);

        // Assert
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    #[test]
    fn map_rdp_connections_onlygroups_should_returnemptyvec() {
        // Arrange
        let expected_connections = Connections {
            connections: vec![Node {
                name: "test".to_string(),
                node_type: NodeType::Container,
                domain: "test".to_string(),
                hostname: "test".to_string(),
                protocol: "RDP".to_string(),
                description: None,
                username: None,
                nodes: None,
            }],
        };

        // Act
        let result = map_rdp_connections(&expected_connections);

        // Assert
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    #[test]
    fn map_rdp_connections_wrongprotocol_should_returnemptyvec() {
        // Arrange
        let expected_connections = Connections {
            connections: vec![Node {
                name: "test".to_string(),
                node_type: NodeType::Connection,
                domain: "test".to_string(),
                hostname: "test".to_string(),
                protocol: "BAD".to_string(),
                description: None,
                username: None,
                nodes: None,
            }],
        };

        // Act
        let result = map_rdp_connections(&expected_connections);

        // Assert
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    #[test]
    fn map_rdp_connections_onlyconnections_should_returnconnections() {
        // Arrange
        let expected_connection = Node {
            name: "test1".to_string(),
            node_type: NodeType::Connection,
            domain: "test2".to_string(),
            hostname: "test3".to_string(),
            protocol: "RDP".to_string(),
            description: Some("test4".to_string()),
            username: Some("test5".to_string()),
            nodes: None,
        };
        let expected_connections = Connections {
            connections: vec![expected_connection.clone()],
        };

        // Act
        let result = map_rdp_connections(&expected_connections);

        // Assert
        assert!(result.is_ok());
        let actual_vals = result.unwrap();
        assert!(actual_vals.len() == 1);
        let actual_val = actual_vals.first().unwrap();
        assert_eq!(actual_val.name, expected_connection.name);
        assert_eq!(actual_val.description, expected_connection.description);
        assert_eq!(actual_val.group, None);
        assert_eq!(actual_val.username, expected_connection.username);
        assert_eq!(actual_val.hostname, expected_connection.hostname);
        assert_eq!(actual_val.domain, expected_connection.domain);
    }

    #[test]
    fn map_rdp_connections_connectionswithgroup_should_returnconnections() {
        // Arrange
        let expected_connection = Node {
            name: "test1".to_string(),
            node_type: NodeType::Connection,
            domain: "test2".to_string(),
            hostname: "test3".to_string(),
            protocol: "RDP".to_string(),
            description: Some("test4".to_string()),
            username: Some("test5".to_string()),
            nodes: None,
        };

        let expected_group = Node {
            name: "group1".to_string(),
            node_type: NodeType::Container,
            domain: "group2".to_string(),
            hostname: "group3".to_string(),
            protocol: "RDP".to_string(),
            description: None,
            username: None,
            nodes: Some(vec![expected_connection.clone()]),
        };

        let expected_connections = Connections {
            connections: vec![expected_group.clone()],
        };

        // Act
        let result = map_rdp_connections(&expected_connections);

        // Assert
        assert!(result.is_ok());
        let actual_vals = result.unwrap();
        assert!(actual_vals.len() == 1);
        let actual_val = actual_vals.first().unwrap();
        assert_eq!(actual_val.name, expected_connection.name);
        assert_eq!(actual_val.description, expected_connection.description);
        assert_eq!(actual_val.group, Some(expected_group.name));
        assert_eq!(actual_val.username, expected_connection.username);
        assert_eq!(actual_val.hostname, expected_connection.hostname);
        assert_eq!(actual_val.domain, expected_connection.domain);
    }
}

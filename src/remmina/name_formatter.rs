use crate::ng::models::RDPConnection;

pub fn format_name(connection: &RDPConnection) -> String {
    let group = sanitize(match connection.group.clone() {
        Some(x) => x,
        None => "group".to_string(),
    });

    let protocol = "rdp";
    let name = sanitize(connection.name.clone());
    let hostname = sanitize(connection.hostname.clone());

    format!("{}_{}_{}_{}.remmina", group, protocol, name, hostname).to_lowercase()
}

fn sanitize(some_str: String) -> String {
    let invalid_chars: Vec<char> = "\\%|/$?<>:*. \"".chars().collect();
    let replacement_char = '-';

    let sanitized_str: String = some_str
        .chars()
        .map(|f| {
            if invalid_chars.contains(&f) {
                return replacement_char;
            }

            f
        })
        .collect();

    sanitized_str
}

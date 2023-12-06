use tera::{Context, Tera};

use crate::ng::models::RDPConnection;

pub fn template(connection: &RDPConnection) -> Result<String, String> {
    let mut tera = Tera::default();

    match tera.add_raw_template("template.remmina", get_template()) {
        Ok(t) => t,
        Err(e) => {
            eprint!("Remmina template error: {}", e);

            return Err("Error creating template for Remmina".to_string());
        }
    };

    let context = build_context(connection);

    match tera.render("template.remmina", &context) {
        Ok(result) => Ok(result),
        Err(e) => {
            eprintln!("Rendering error(s): {}", e);

            Err("Error rendering template for Remmina".to_string())
        }
    }
}

fn build_context(connection: &RDPConnection) -> Context {
    let username = connection.username.clone().unwrap_or_default();
    let description = connection.description.clone().unwrap_or_default();
    let hostname = connection.hostname.clone();
    let name = connection.name.clone();
    let group = connection.group.clone().unwrap_or_default();
    let domain = connection.domain.clone();

    let mut context = Context::new();

    context.insert("username", &username);
    context.insert("description", &description);
    context.insert("hostname", &hostname);
    context.insert("name", &name);
    context.insert("group", &group);
    context.insert("domain", &domain);

    context
}

fn get_template() -> &'static str {
    r#"
[remmina]
password=
gateway_username=
notes_text=
vc=
preferipv6=0
ssh_tunnel_loopback=0
serialname=
tls-seclevel=
websockets=0
printer_overrides=
name={{name}}
console=0
colordepth=99
security=
precommand=
disable_fastpath=0
postcommand=
left-handed=0
multitransport=0
group={{group}}
server={{hostname}}
ssh_tunnel_certfile=
glyph-cache=0
ssh_tunnel_enabled=0
disableclipboard=0
labels={{description}}
audio-output=
parallelpath=
monitorids=
cert_ignore=0
gateway_server=
serialpermissive=0
protocol=RDP
old-license=0
ssh_tunnel_password=
resolution_mode=2
pth=
loadbalanceinfo=
disableautoreconnect=0
clientbuild=
clientname=
resolution_width=0
drive=
relax-order-checks=0
base-cred-for-gw=0
gateway_domain=
network=none
rdp2tcp=
gateway_password=
serialdriver=
rdp_reconnect_attempts=
profile-lock=0
domain={{domain}}
smartcardname=
multimon=0
restricted-admin=0
exec=
enable-autostart=0
usb=
shareprinter=0
ssh_tunnel_passphrase=
disablepasswordstoring=0
username={{username}}
serialpath=
quality=0
span=0
shareparallel=0
parallelname=
viewmode=1
ssh_tunnel_auth=0
keymap=
ssh_tunnel_username=
execpath=
shareserial=0
execcommand=
resolution_height=0
useproxyenv=0
sharesmartcard=0
freerdp_log_filters=
microphone=
runasync=0
ssh_tunnel_privatekey=
gwtransp=http
ssh_tunnel_server=
ignore-tls-errors=1
window_maximize=1
dvc=
gateway_usage=0
timeout=
disable-smooth-scrolling=0
no-suppress=0
freerdp_log_level=INFO
sound=offremmina]
password=
gateway_username=
notes_text=
"#
}

use hbb_common::config;

const APP_NAME: &str = "AlloDesk";
const RENDEZVOUS_SERVER: &str = "remote.allo-geek.com";
// Public key of our hbbs (`data/id_ed25519.pub` on the server). Left empty,
// the client falls back to RustDesk's public key and our server rejects it.
const RS_PUB_KEY: &str = "DfLDM9nlq0XvVF1swLjQxl34q5XXrcAdvZE5yx5MaBY=";

// Runs before any config is loaded: APP_NAME names the config directory.
// Defaults rather than overwrites, so the technician can still point a build
// at another server from Settings > Network.
pub fn apply() {
    *config::APP_NAME.write().unwrap() = APP_NAME.to_owned();
    let mut defaults = config::DEFAULT_SETTINGS.write().unwrap();
    defaults.insert(
        "custom-rendezvous-server".to_owned(),
        RENDEZVOUS_SERVER.to_owned(),
    );
    if !RS_PUB_KEY.is_empty() {
        defaults.insert("key".to_owned(), RS_PUB_KEY.to_owned());
    }
}

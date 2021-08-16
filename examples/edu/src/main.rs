use fabric::core::config::ConfigBackend;

fn main() {
    ConfigBackend::from_file("./config_e2e.yaml");
}

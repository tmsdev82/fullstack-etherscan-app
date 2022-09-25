
fn main() {
    log4rs::init_file("logging_config.yml", Default::default()).unwrap();
    log::debug!("Backend running");
}

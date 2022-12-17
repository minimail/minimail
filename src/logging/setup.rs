pub fn setup_logging(path: &str) {
    // unwrapping here for now. If setting up logging fails, we have issues.
    log4rs::init_file(path, Default::default()).unwrap();
}

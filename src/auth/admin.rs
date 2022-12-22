pub fn get_admin_token() -> Option<String> {
    std::env::var("ADMIN_TOKEN").ok()
}

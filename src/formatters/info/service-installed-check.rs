pub fn format(result: &str) -> String {
    if result != "install ok installed" {
        return "NOT INSTALLED".to_string();
    }

    "INSTALLED".to_string()
}

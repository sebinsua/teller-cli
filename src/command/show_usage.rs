pub fn show_usage_command(usage: &str) -> i32 {
    info!("Calling the show usage command");
    print!("{}", usage);
    0
}

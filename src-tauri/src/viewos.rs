pub fn relative_command_path(command: String) -> Result<String, &'static str> {
    match std::env::current_exe().unwrap().parent() {
        #[cfg(windows)]
        Some(exe_dir) => Ok(format!("{}\\{}.exe", exe_dir.display(), command)),
        #[cfg(not(windows))]
        Some(exe_dir) => Ok(format!("{}/{}", exe_dir.display(), command)),
        None => Err("Could not evaluate executable dir"),
    }
}
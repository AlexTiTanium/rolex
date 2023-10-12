pub fn is_ansible_installed() -> bool {
    // For Unix-like systems
    if std::process::Command::new("sh")
        .arg("-c")
        .arg("which ansible")
        .status()
        .unwrap()
        .success()
    {
        return true;
    }

    // For Windows
    if cfg!(target_os = "windows") {
        if std::process::Command::new("where.exe")
            .arg("ansible")
            .status()
            .unwrap()
            .success()
        {
            return true;
        }
    }

    false
}

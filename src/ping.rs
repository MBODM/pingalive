use std::{
    fs::File,
    process::{Command, Stdio},
};

const PING_APP: &str = "ping.exe";

pub fn is_available() -> bool {
    // Using .output() here, cause .status() writes the Command output to the terminal.
    let output_result = Command::new(PING_APP).arg("/?").output();
    // If there is NOT a NotFound error, this means PowerShell is definitely available.
    // Only the NotFound error shall produce a false result here. No other error shall.
    // It is ok to ignore all other errors here and to just fail gracefully on execute.
    // The sole purpose here is to determine if the PowerShell app is available or not.
    // For the same reason there is no validation here if exit code was SUCCESS or not.
    match output_result {
        Ok(_) => true,
        Err(error) => !is_notfound_error(&error),
    }
}

pub fn foo() -> Result<(), std::io::Error> {
    let child = Command::new("ping.exe")
        .args(&["-n 10", "194.25.2.129"])
        .stdout(Stdio::piped())
        .spawn()?;

    let mut f = File::create("ping.log")?;
    std::io::copy(&mut child.stdout.unwrap(), &mut f)?;
    Ok(())
}

pub fn run() -> bool {
    let child_result = Command::new(PING_APP).arg("-t 194.25.2.129").spawn();
    let child = match child_result {
        Ok(r) => r,
        Err(_) => return false,
    };
    let output_result = child.wait_with_output();
    let output = match output_result {
        Ok(r) => r,
        Err(_) => return false,
    };
    if !output.status.success() {
        todo!()
    }
    // let stdout_text =
    //     String::from_utf8(output.stdout).map_err(|err| AppErr::from_string(err.to_string()))?;
    true
}

fn is_notfound_error(error: &std::io::Error) -> bool {
    match error.kind() {
        std::io::ErrorKind::NotFound => true,
        _ => false,
    }
}

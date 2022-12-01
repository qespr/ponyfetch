use std::{fs::File, io::Read};
use std::process::Command;
use std::sync::Mutex;

#[cfg(target_os = "linux")]
pub fn get_ipaddr() -> String {
    use std::ops::Add;

    let mut final_str: Mutex<String> = Mutex::new(String::new());

    let mut f = File::open("/proc/net/route").unwrap();
    let mut intr = String::new();

    f.read_to_string(&mut intr).unwrap();

    let lines: &Vec<&str> = &intr.lines().collect();
    let mut interface = String::new();

    lines.into_iter().for_each(|line| {
        if line.contains("00000000") {
            interface = line.split("\t").collect::<Vec<&str>>()[0].to_string();
        }
    });

    let output = Command::new("ifconfig")
        .arg(interface.clone())
        .output()
        .expect("Failed to execute ifconfig");

    let output = String::from_utf8(output.stdout).unwrap();

    let lines: &Vec<&str> = &output.lines().clone().collect();

    let mut next: bool = false;

    let process_ip = |line: &str| {
        let ip = line.split(" ").collect::<Vec<&str>>()[1].to_string();
        final_str.lock().unwrap().push_str(&ip);
    };

    lines.into_iter().for_each(|line| {
        if next {
            line.replace("\t", "")
                .split("  ")
                .collect::<Vec<&str>>()
                .into_iter()
                .for_each(|item| {
                    if item.contains("inet") {
                        process_ip(item);
                    }
                });

            next = false;
        }

        if line.contains(&interface) {
            next = !next;
        }
    });

    let x = final_str
        .lock()
        .unwrap()
        .to_string()
        .add(format!(" ({})", interface).as_str()); 
        
    x
}
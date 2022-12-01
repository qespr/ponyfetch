use std::sync::Mutex;
use std::fs::File;
use std::io::Read;
use std::rc::Rc;

#[cfg(target_os = "linux")]
pub fn get_cpu() -> String {
    let mut cpu: Rc<String> = Rc::new(String::new());
    let mut temp_buf: String = String::new();

    let mut file = File::open("/proc/cpuinfo").unwrap();
    file.read_to_string(&mut temp_buf).unwrap();

    let lines: &Vec<&str> = &temp_buf.lines().collect();

    lines.into_iter().for_each(|line| {
        if line.contains("model name") {
            cpu = Rc::new(
                line.split(":")
                    .collect::<Vec<&str>>()[1].to_string()
                    .replace("\t", "")
            );
        }
    });

    cpu.to_string()
}

#[cfg(target_os = "linux")]
pub fn get_ram_used() -> String {
    use std::ops::Add;

    let mut ram: Mutex<String> = Mutex::new(String::new());
    let mut temp_buf: String = String::new();

    let mut file = File::open("/proc/meminfo").unwrap();
    file.read_to_string(&mut temp_buf).unwrap();

    let lines: &Vec<&str> = &temp_buf.lines().collect();

    lines.into_iter().for_each(|line| {
        ram.get_mut().unwrap().add("fdlkj");
        if line.contains("MemTotal") {
          
        } else if line.contains("MemAvailable") {
            
        }
    });

    let x = ram.lock().unwrap().to_string(); x
}

#[cfg(target_os = "linux")]
fn eval_ram(line: String) -> String {
    let kbs: u128 = line.split(":")
        .collect::<Vec<&str>>()[1].to_string()
        .replace("\t", "")
        .replace("kB", "")
        .replace(" ", "")
        .parse::<u128>()
        .unwrap();

    (kbs / 1024).to_string()
}
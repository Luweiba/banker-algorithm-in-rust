#[allow(dead_code)]
mod process;
mod banker;
use banker::Banker;
use process::{Process, ResourceVector};
use std::io::{BufReader, BufRead};
use std::fs::File;

fn main() {
    let config_path = "./config.txt";
    let config_file = File::open(config_path).unwrap();
    let mut buffer_reader = BufReader::new(config_file);
    let mut buf = String::with_capacity(512);
    buffer_reader.read_line(&mut buf).unwrap();
    let resource_len = buf.trim().parse::<u32>().unwrap();
    buf.clear();
    buffer_reader.read_line(&mut buf).unwrap();
    let resource = buf.trim().split(" ").map(|s| s.trim().parse::<u32>().unwrap()).collect::<Vec<u32>>();
    let mut banker = Banker::new(ResourceVector::from_vec(resource));
    buf.clear();
    buffer_reader.read_line(&mut buf).unwrap();
    let process_cnt = buf.trim().parse::<u32>().unwrap();
    for _ in 0..process_cnt {
        buf.clear();
        buffer_reader.read_line(&mut buf).unwrap();
        let resource_max_need = buf.trim().split(" ").map(|s| s.trim().parse::<u32>().unwrap()).collect::<Vec<u32>>();
        let process = Process::new(ResourceVector::from_vec(resource_max_need));
        banker.add_one_process(process);
    }
    banker.statistic();
    loop {
        println!("Enter a request:(format like below)\n\t\t[PID] [Resource requested separate with a space]\nTips: To see Statistic informations, enter \"info\"");
        buf.clear();
        std::io::stdin().read_line(&mut buf).unwrap();
        if buf.contains("info") {
            banker.statistic();
            continue;
        }
        let request_vec = buf.trim().split(" ").map(|s| s.trim().parse::<u32>().unwrap()).collect::<Vec<u32>>();
        let pid = request_vec[0];
        if !Process::is_pid_vaild(pid) {
            println!("Pid {} is invalid!", pid);
            continue;
        }
        if request_vec.len() != (resource_len + 1) as usize {
            println!("Bad Request, expect length {}", resource_len);
            continue;
        }
        let request = ResourceVector::from_vec(request_vec.into_iter().skip(1).collect());
        banker.handle_a_request(request, pid);
    }
}

use crate::process::*;
#[derive(Debug)]
pub struct Banker {
    available_resource: ResourceVector,
    total_resource: ResourceVector,
    process_queue: Vec<Process>,

}
impl Banker {
    pub fn new(total_resource: ResourceVector) -> Self {
        Self {
            available_resource: total_resource.clone(),
            total_resource: total_resource,
            process_queue: vec![],
        }
    }

    pub fn add_one_process(&mut self, process: Process) {
        self.process_queue.push(process);
        self.process_queue.sort_by(|a, b| a.pid.cmp(&b.pid));
    }

    pub fn handle_a_request(&mut self, request: ResourceVector, pid: u32) {
        // 检测系统是否有足够的资源分配
        if request > self.available_resource {
            println!("Requests is invalid: The requests is surpass the Available resources of the System, the Available resources is {}", self.available_resource);
            return;
        }
        let mut process_handle_index = 0;
        let mut flag = true;
        for (i, process) in self.process_queue.iter().enumerate() {
            if process.pid == pid {
                process_handle_index = i;
                flag = false;
                break;
            }
        }
        if flag {
            println!("Pid is invalid");
            return;
        }
        // 检测请求是否满足要求
        if request > self.process_queue[process_handle_index].need {
            println!("Requests is invalid: The requests is surpass the need of the Process {}, the need is {}", pid, self.process_queue[process_handle_index].need);
            return;
        }
        // 预分配
        self.process_queue[process_handle_index].allocated += request.clone();
        self.process_queue[process_handle_index].need -= request.clone();
        self.available_resource -= request.clone();
        // 检验是否满足安全性
        if self.is_in_safe_state() {
            println!("Requests is Safe, Successfully Allocated");
            if self.process_queue[process_handle_index].need.inner_resource.iter().fold(0, |mut acc, x| {acc += *x; acc }) == 0 {
                self.process_queue[process_handle_index].state = ProcessState::Finished;
                self.available_resource += self.process_queue[process_handle_index].allocated.clone();
            }
        } else {
            println!("Requests is Unsafe");
            self.process_queue[process_handle_index].allocated -= request.clone();
            self.process_queue[process_handle_index].need += request.clone();
            self.available_resource += request.clone();
        }
    }
    pub fn is_in_safe_state(&self) -> bool {
        println!("Safety Testing . . .");
        let mut available_resource_clone = self.available_resource.clone();
        let mut process_queue_clone = self.process_queue.clone();
        let process_cnt = self.process_queue.len();
        let mut safe_queue = Vec::with_capacity(process_cnt);
        let mut unsafe_test_count = process_queue_clone.len() as i32;
        while let Some(process) = process_queue_clone.pop() {
            if let ProcessState::Running = process.state {
                if process.need < self.available_resource {
                    available_resource_clone += process.allocated.clone();
                    safe_queue.push(process.pid);
                    // 重置不安全检测标志
                    unsafe_test_count = process_queue_clone.len() as i32;
                } else {
                    process_queue_clone.push(process);
                    unsafe_test_count -= 1;
                }
            }
            // 不安全
            if unsafe_test_count < 0 {
                return false;
            }
        }
        // 安全
        println!("Safe Queue is list below:");
        for pid in safe_queue {
            print!("{} ", pid);
        }
        println!();
        true
    }
    pub fn statistic(&self) {
        for process in self.process_queue.iter() {
            println!("PID: {}", process.pid);
            println!("Allocated: {}", process.allocated);
            println!("Still_Need: {}", process.need);
            println!("Max_Need: {}", process.max_need);
            println!("State: {}", process.state);
            println!("______________________________________");
        }
        println!("Available: {}", self.available_resource);
        println!("Total: {}", self.total_resource);
    }
}
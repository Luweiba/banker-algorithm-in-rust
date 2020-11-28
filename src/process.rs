//! Process进程抽象

use std::fmt::Formatter;
use std::ops::{Sub, Add, AddAssign, SubAssign};
use std::cmp::Ordering;

static mut PID_ALLOC_COUNTER: u32 = 0;
#[derive(Debug, Clone)]
pub enum ProcessState {
    Running,
    Finished,
}
#[derive(Clone, Debug)]
pub struct Process {
    pub pid: u32,
    pub allocated: ResourceVector,
    pub max_need: ResourceVector,
    pub need: ResourceVector,
    pub state: ProcessState,
}

impl Process {
    pub fn new(max_need: ResourceVector) -> Self {
        let len = max_need.cnt;
        let pid;
        unsafe {
            pid = PID_ALLOC_COUNTER;
            PID_ALLOC_COUNTER += 1;
        }
        Self {
            pid,
            allocated: ResourceVector::new(len),
            max_need: max_need.clone(),
            need: max_need,
            state: ProcessState::Running,
        }
    }
    pub fn is_pid_vaild(pid: u32) -> bool {
        unsafe {
            if pid < PID_ALLOC_COUNTER {
                true
            } else {
                false
            }
        }
    }
}
/// 资源Resource抽象
#[derive(Debug, Clone, Eq, PartialEq, PartialOrd)]
pub struct ResourceVector {
    pub inner_resource: Vec<u32>,
    pub cnt: usize,
}

impl ResourceVector {
    pub fn new(cnt: usize) -> Self {
        Self {
            inner_resource: vec![0; cnt],
            cnt,
        }
    }

    pub fn from_vec(resource: Vec<u32>) -> Self {
        let len = resource.len();
        Self {
            inner_resource: resource,
            cnt: len,
        }
    }
}

impl std::fmt::Display for ResourceVector {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut buf = String::new();
        for &item in self.inner_resource.iter() {
            buf.push_str(&format!(" {}", item));
        }
        write!(f, "{}", buf)
    }
}
impl Sub for ResourceVector {
    type Output = ResourceVector;

    fn sub(self, rhs: Self) -> Self::Output {
        let len = self.cnt;
        let mut output = Vec::with_capacity(self.inner_resource.len());
        for i in 0..len {
            output.push(self.inner_resource[i] - rhs.inner_resource[i]);
        }
        Self::from_vec(output)
    }
}
impl Add for ResourceVector {
    type Output = ResourceVector;

    fn add(self, rhs: Self) -> Self::Output {
        let len = self.cnt;
        let mut output = Vec::with_capacity(self.inner_resource.len());
        for i in 0..len {
            output.push(self.inner_resource[i] + rhs.inner_resource[i]);
        }
        Self::from_vec(output)
    }
}
impl AddAssign for ResourceVector {
    fn add_assign(&mut self, rhs: Self) {
        for i in 0..self.cnt {
            self.inner_resource[i] += rhs.inner_resource[i];
        }
    }
}
impl SubAssign for ResourceVector {
    fn sub_assign(&mut self, rhs: Self) {
        for i in 0..self.cnt {
            self.inner_resource[i] -= rhs.inner_resource[i];
        }
    }
}
impl std::fmt::Display for ProcessState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ProcessState::Running => write!(f, "Running"),
            ProcessState::Finished => write!(f, "Finished"),
        }
    }
}
impl std::cmp::Ord for ResourceVector {
    fn cmp(&self, other: &Self) -> Ordering {
        let len = self.inner_resource.len();
        let order = Ordering::Less;
        for i in 0..len {
            if self.inner_resource[i] > other.inner_resource[i] {
                return order.reverse();
            }
        }
        order
    }
}
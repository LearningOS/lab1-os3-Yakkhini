//! Process management syscalls

use crate::config::{MAX_APP_NUM, MAX_SYSCALL_NUM};
use crate::task::{self, exit_current_and_run_next, suspend_current_and_run_next, TaskStatus};
use crate::timer::get_time_us;

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

pub struct TaskInfo {
    status: TaskStatus,
    syscall_times: [u32; MAX_SYSCALL_NUM],
    time: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    info!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    suspend_current_and_run_next();
    0
}

/// get time with second and microsecond
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    let us = get_time_us();
    unsafe {
        *ts = TimeVal {
            sec: us / 1_000_000,
            usec: us % 1_000_000,
        };
    }
    0
}

/// YOUR JOB: Finish sys_task_info to pass testcases
pub fn sys_task_info(ti: *mut TaskInfo) -> isize {
    let syscall_times_vector = task::get_syscall_times();
    let mut syscall_times = [0; MAX_SYSCALL_NUM];

    let mut i = 0;
    for item in syscall_times_vector.iter() {
        syscall_times[i] = *item;
        i = i + 1;
    }
    unsafe {
        *ti = TaskInfo {
            status: TaskStatus::Running,
            syscall_times,
            time: task::get_current_task_time(),
        }
    };
    0
}

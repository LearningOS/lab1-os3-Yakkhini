//! Types related to task management

use super::TaskContext;

use alloc::vec::Vec;

#[derive(Copy, Clone)]
/// task control block structure
pub struct TaskControlBlock {
    pub task_status: TaskStatus,
    pub task_cx: TaskContext,
    // LAB1: Add whatever you need about the Task.
    /// Use start_time to record start time of task, aim to calculate time.
    pub start_time: usize,

    /// syscall_times
    pub syscall_times: [u32; 500],
}

#[derive(Copy, Clone, PartialEq)]
/// task status: UnInit, Ready, Running, Exited
pub enum TaskStatus {
    UnInit,
    Ready,
    Running,
    Exited,
}

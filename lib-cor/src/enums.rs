use std::fmt::Debug;

// Определение общего состояния процесса
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Status {
    None,
    Running,
    Finishing,
    Failing,
}

use std::fmt::Debug;

// Определение общего состояния процесса
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CorStatus {
    None,
    Running,
    Finishing,
    Failing,
}

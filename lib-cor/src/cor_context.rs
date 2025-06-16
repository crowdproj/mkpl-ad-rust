use crate::CorError;
use crate::Status;

// Трейт контекста
pub trait CorContext<E, F> {
    fn push_error(&mut self, err: CorError<E, F>);
    fn set_status(&mut self, status: Status);
    fn fail(&mut self, err: CorError<E, F>) {
        self.push_error(err);
        self.set_status(Status::Failing);
    }
}

use crate::config::FnmConfig;

pub trait Command: Sized {
    type Error;
    fn apply(self, config: FnmConfig) -> Result<(), Self::Error>;
    fn handle_error(err: Self::Error);

    fn call(self, config: FnmConfig) {
        match self.apply(config) {
            Ok(()) => (),
            Err(err) => Self::handle_error(err),
        }
    }
}

pub trait ResultExt<T> {
    fn unwrap_or_log(self) -> T;
}

impl<T, E: std::fmt::Debug> ResultExt<T> for Result<T, E> {
    fn unwrap_or_log(self) -> T {
        match self {
            Ok(value) => value,
            Err(e) => {
                eprintln!("Error: {:?}", &e);
                panic!("Terminating due to error: {:?}.", &e);
            }
        }
    }
}

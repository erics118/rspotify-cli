use anyhow::Result;

pub trait ResultOkPrintErr<T> {
    fn ok_or_print_err(self);
}

impl<T> ResultOkPrintErr<T> for Result<T> {
    fn ok_or_print_err(self) {
        match self {
            Ok(_) => (),
            Err(e) => eprintln!("{e}"),
        }
    }
}

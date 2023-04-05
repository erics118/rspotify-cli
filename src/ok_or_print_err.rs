use anyhow::Result;

/// Print error Err
pub trait OkOrPrintErr<T> {
    fn ok_or_print_err(self);
}

impl<T> OkOrPrintErr<T> for Result<T> {
    fn ok_or_print_err(self) {
        match self {
            Ok(_) => (),
            Err(e) => eprintln!("{e}"),
        }
    }
}

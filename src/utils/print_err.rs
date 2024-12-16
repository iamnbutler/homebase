use std::panic::Location;

pub trait PrintErr<E> {
    type Ok;

    fn print_err(self) -> Option<Self::Ok>;
}

impl<T, E> PrintErr<E> for Result<T, E>
where
    E: std::fmt::Debug,
{
    type Ok = T;

    #[track_caller]
    fn print_err(self) -> Option<T> {
        self.map_err(|error| {
            let caller = *Location::caller();
            let file = caller.file();
            let line = caller.line();

            println!("Error at {}:{}: {:?}", file, line, error);
        })
        .ok()
    }
}

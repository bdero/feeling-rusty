extern crate sdl2;

fn main() {
    let (code, reason) = feeling_rusty();

    if code != 0 {
        println!(
            "An unrecoverable error was encountered: {:?}",
            reason.unwrap()
        );
    }
    std::process::exit(code);
}

macro_rules! unrecoverable {
    ($error:expr) => {
        return (
            1,
            Some(String::from($error))
        )
    };
}

macro_rules! expected_exit {
    () => (return (0, None));
}

macro_rules! essential_unwrap {
    ($x:expr) => {
        match $x {
            Ok(value) => value,
            Err(error) => {
                unrecoverable!(error);
            }
        };
    };
}

fn feeling_rusty() -> (i32, Option<String>) {
    let mut sdl_context = essential_unwrap!(sdl2::init());

    expected_exit!();
}

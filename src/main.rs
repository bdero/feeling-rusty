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

macro_rules! essential {
    ($x:expr, $error:expr) => {
        match $x {
            Ok(value) => value,
            Err(_) => {
                unrecoverable!(String::from($error));
            }
        };
    };
}

fn feeling_rusty() -> (i32, Option<String>) {
    let sdl_context = essential!(sdl2::init(), "Couldn't initialize SDL context.");
    let video_system = essential!(sdl_context.video(), "Couldn't initialize SDL subsystem.");

    let window = essential!(
        video_system.window("Feeling Rusty", 800, 600)
            .position_centered()
            .opengl()
            .build(),
        "Couldn't initialize SDL window."
    );

    let mut renderer = essential!(window.renderer().build(), "Couldn't initialize SDL renderer.");

    expected_exit!();
}

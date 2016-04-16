extern crate sdl2;

macro_rules! str {
    ($x: expr) => (String::from($x));
}

fn main() {
    let (code, reason) = feeling_rusty();

    if code != 0 {
        println!(
            "An unrecoverable error was encountered{:?}",
            match reason {
                Some(message) => format!(": {}", message),
                None => str!("!")
            }
        );
    }
    std::process::exit(code);
}

macro_rules! unrecoverable {
    ($error:expr) => (return (1, Some(str!($error))));
}

macro_rules! expected_exit {
    () => (return (0, None));
}

macro_rules! essential {
    ($x:expr, $error:expr) => {
        match $x {
            Ok(value) => value,
            Err(_) => unrecoverable!(str!($error))
        };
    };
}

fn feeling_rusty() -> (i32, Option<String>) {
    let sdl_context = essential!(sdl2::init(), "Couldn't initialize SDL context.");
    let video_system = essential!(sdl_context.video(), "Couldn't initialize SDL subsystem.");

    let gl_attributes = video_system.gl_attr();
    gl_attributes.set_context_major_version(3);
    gl_attributes.set_context_minor_version(2);
    gl_attributes.set_double_buffer(true);

    let window = essential!(
        video_system.window("Feeling Rusty", 800, 600)
            .position_centered()
            .opengl()
            .build(),
        "Couldn't initialize SDL window."
    );

    let mut gl_context = essential!(window.gl_create_context(), "Unable to initialize GL context.");

    expected_exit!();
}

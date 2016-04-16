extern crate glium;


struct Monitor {
    id: glium::glutin::MonitorId,
    name: String,
}

impl Monitor {
    fn new(id: glium::glutin::MonitorId) -> Monitor {
        let dimensions = id.get_dimensions();
        let name = id.get_name().unwrap();
        Monitor {
            id: id,
            name: format!("{} ({}x{})", name, dimensions.0, dimensions.1),
        }
    }
}

fn main() {
    use glium::DisplayBuild;

    println!("Detected Monitors:");
    let monitors: Vec<Monitor> = glium::glutin::get_available_monitors().map(|mon| {
        Monitor::new(mon)
    }).collect();
    for (i, mon) in monitors.iter().enumerate() {
        println!("{}: {}", i, mon.name);
    }


    let display = glium::glutin::WindowBuilder::new()
        .with_title(String::from("Glium test"))
        .with_dimensions(800, 600)
        .build_glium()
        .unwrap();
}

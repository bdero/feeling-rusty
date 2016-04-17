extern crate glium;

struct Monitor {
    name: String,
    id: glium::glutin::MonitorId,
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

    fn get_total_pixels(&self) -> u32 {
        let dimensions = self.id.get_dimensions();
        dimensions.0*dimensions.1
    }
}

fn main() {
    use glium::{DisplayBuild, Surface};

    println!("Detected Monitors:");
    let monitors: Vec<Monitor> = glium::glutin::get_available_monitors().map(|mon| {
        Monitor::new(mon)
    }).collect();
    for (i, mon) in monitors.iter().enumerate() {
        println!("{}: {}", i, mon.name);
    }
    let largest_monitor = monitors.iter().max_by_key(|x| x.get_total_pixels()).unwrap();
    println!("Largest monitor: {}", largest_monitor.id.get_name().unwrap());

    let display = glium::glutin::WindowBuilder::new()
        .with_title(String::from("Glium test"))
        .with_dimensions(800, 600)
        .build_glium()
        .unwrap();

    loop {
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        target.finish().unwrap();

        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                _ => (),
            }
        }
    }
}

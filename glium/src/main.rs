extern crate glium;

use glium::glutin::MonitorId;

struct Monitor {
    total_pixels: u32,
    name: String,
    id: glium::glutin::MonitorId,
}

impl Monitor {
    fn new(id: glium::glutin::MonitorId) -> Monitor {
        let dimensions = id.get_dimensions();
        let name = id.get_name().unwrap();
        Monitor {
            total_pixels: dimensions.0*dimensions.1,
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
    use glium::DisplayBuild;

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
}

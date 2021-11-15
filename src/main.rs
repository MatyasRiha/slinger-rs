pub mod graphics;
extern crate sdl2;
extern crate gl;
pub mod ecs;


fn main() {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    
    let _window = video_subsystem
        .window("Slinger", 900, 700)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let mut world = ecs::component::world::new();
    world.new_entity(Some(ecs::component::health(-10)), Some(ecs::component::name("Adam")));
    println!("{:?}", world);

    //Testing colors
    let black = graphics::color::black();
    let white = graphics::color::white();
    let red = graphics::color::red();
    let blue = graphics::color::blue();
    let green = graphics::color::green();
    println!("{:?}", black);
    println!("{:?}", white);
    println!("{:?}", red);
    println!("{:?}", blue);
    println!("{:?}", green);

    //let _gl_context = window.gl_create_context().unwrap();
    //let _gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    let mut event_pump = sdl.event_pump().unwrap();
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                _ => {}
            }
        }
    }
    println!("Sling finished");
}




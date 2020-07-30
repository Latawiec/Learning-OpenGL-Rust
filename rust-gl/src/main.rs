#[macro_use] extern crate failure;
extern crate sdl2;
extern crate gl;
extern crate vec_2_10_10_10;
extern crate nalgebra;

pub mod render_gl;
pub mod resources;
mod triangle;

#[macro_use] extern crate render_gl_derive;

fn main() {
    if let Err(e) = run() {
        println!("{}", e);
    }
}

fn run() -> Result<(), failure::Error> {
    let _sdl = sdl2::init().unwrap();
    let video_subsystem = _sdl.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();

    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 5);
    
    let window = video_subsystem
        .window("Game", 900, 700)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    let gl = gl::Gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);
    
    let mut event_pump = _sdl.event_pump().unwrap();

    use std::path::Path;
    let res = resources::Resources::from_relative_exe_path(Path::new("assets")).unwrap();

    let mut viewport = render_gl::Viewport::for_window(900, 700);
    viewport.set_used(&gl);

    let triangle = triangle::Triangle::new(&res, &gl)?;

    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => break 'main,
                sdl2::event::Event::Window {
                    win_event: sdl2::event::WindowEvent::Resized(w, h),
                    ..
                } => {
                    viewport.update_size(w, h);
                    viewport.set_used(&gl);
                }
                _ => {},
            }
        }

        unsafe {
            gl.Clear(gl::COLOR_BUFFER_BIT);
        }
        
        triangle.render(&gl);
        window.gl_swap_window();
    }

    Ok(())
}

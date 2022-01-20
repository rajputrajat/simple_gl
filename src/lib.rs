mod error;
mod shader_helper;
mod shape;
mod shape_manager;

pub use crate::{
    error::{CustomError, CustomResult},
    shader_helper::{build_shader_program, ProgramId},
    shape::Shape,
    shape_manager::ShapesManager,
};
pub use gleam::gl::Gl;
use gleam::gl::GlFns;
pub use glfw::WindowEvent;
use glfw::{self, Context, SwapInterval, Window, WindowMode};
use log::debug;
use std::{sync::mpsc::Receiver, time::Duration};

pub struct SimpleGl {
    glfw_: glfw::Glfw,
    window: Window,
    events: Receiver<(f64, WindowEvent)>,
    shapes_manager: ShapesManager,
}

impl SimpleGl {
    pub fn init(mut shapes_manager: ShapesManager) -> Result<Self, CustomError> {
        let mut glfw_ = glfw::init(glfw::FAIL_ON_ERRORS)?;
        let (mut window, events) = glfw_
            .create_window(800, 600, "trying out simple shapes", WindowMode::Windowed)
            .ok_or_else(|| CustomError::Other("couldn't create window".to_owned()))?;
        let gl = unsafe { GlFns::load_with(|s| window.get_proc_address(s)) };
        window.make_current();
        window.set_key_polling(true);
        glfw_.set_swap_interval(SwapInterval::Sync(1));
        shapes_manager.init(gl)?;
        Ok(Self {
            glfw_,
            window,
            events,
            shapes_manager,
        })
    }

    pub fn main_loop(&mut self) -> CustomResult {
        let mut last_time: f64 = self.glfw_.get_time();
        while !self.window.should_close() {
            let curr_time = self.glfw_.get_time();
            let elapsed = Duration::from_secs_f64(curr_time - last_time);
            self.shapes_manager.update(elapsed)?;
            last_time = curr_time;
            self.glfw_.poll_events();
            for (_, event) in glfw::flush_messages(&self.events) {
                debug!("received event: '{event:?}'");
                self.shapes_manager.input(event)?;
            }
            self.shapes_manager.draw()?;
            self.window.swap_buffers();
        }
        Ok(())
    }
}

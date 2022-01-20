use crate::error::CustomResult;
use gleam::gl::Gl;
use glfw::WindowEvent;
use std::{rc::Rc, time::Duration};

pub trait Shape {
    fn init(&mut self, gl: Rc<dyn Gl>) -> CustomResult;
    fn draw(&self) -> CustomResult;
    fn update(&mut self, elapsed: &Duration) -> CustomResult;
    fn input(&mut self, event: &WindowEvent) -> CustomResult;
}

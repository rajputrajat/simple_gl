use crate::error::CustomResult;
use crate::shape::Shape;
use gleam::gl::Gl;
use glfw::WindowEvent;
use std::{rc::Rc, time::Duration};

pub struct ShapesManager {
    pub shapes: Vec<Box<dyn Shape>>,
}

impl ShapesManager {
    pub fn new() -> Self {
        Self { shapes: vec![] }
    }
    pub fn add_shape(&mut self, shape: Box<dyn Shape>) {
        self.shapes.push(shape);
    }
    pub fn init(&mut self, gl: Rc<dyn Gl>) -> CustomResult {
        for shape in &mut self.shapes {
            shape.init(gl.clone())?;
        }
        Ok(())
    }
    pub fn update(&mut self, elapsed: Duration) -> CustomResult {
        for shape in &mut self.shapes {
            shape.update(&elapsed)?;
        }
        Ok(())
    }
    pub fn input(&mut self, event: WindowEvent) -> CustomResult {
        for shape in &mut self.shapes {
            shape.input(&event)?;
        }
        Ok(())
    }
    pub fn draw(&self) -> CustomResult {
        for shape in &self.shapes {
            shape.draw()?;
        }
        Ok(())
    }
}

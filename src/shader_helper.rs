use crate::error::CustomError;
use gleam::gl::{self, GLenum, GLint, GLuint, Gl};
use log::trace;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    rc::Rc,
};

struct ShaderHelper {
    gl: Rc<dyn Gl>,
}

impl ShaderHelper {
    fn parse_shader(&self, shader_path: &str) -> Result<(String, String), CustomError> {
        let shader_file = File::open(shader_path)?;
        let shader_reader = BufReader::new(shader_file);
        let mut vertex_source = String::new();
        let mut fragment_source = String::new();
        let mut invalid_source = String::new();
        let mut current = &mut invalid_source;
        for l in shader_reader.lines() {
            let l = l?;
            if l.contains("#shader vertex") {
                current = &mut vertex_source;
            } else if l.contains("#shader fragment") {
                current = &mut fragment_source;
            } else {
                current.push_str(&l);
                current.push('\n');
            }
        }
        assert!(invalid_source.is_empty());
        trace!("v_src: {vertex_source}, f_src: {fragment_source}");
        Ok((vertex_source, fragment_source))
    }

    fn compile_shader(&self, shader_type: GLenum, source: &str) -> Result<ShaderId, CustomError> {
        let shader_id = self.gl.create_shader(shader_type);
        let source: [&[u8]; 1] = [source.as_bytes()];
        self.gl.shader_source(shader_id, &source);
        self.gl.compile_shader(shader_id);
        let mut comp_status: [GLint; 1] = [0];
        unsafe {
            self.gl
                .get_shader_iv(shader_id, gl::COMPILE_STATUS, &mut comp_status)
        };
        if comp_status[0] == gl::FALSE as i32 {
            Err(CustomError::Other(self.gl.get_shader_info_log(shader_id)))
        } else {
            Ok(ShaderId(shader_id))
        }
    }

    fn link(&self, shaders: &[ShaderId]) -> Result<ProgramId, CustomError> {
        let program_id = self.gl.create_program();
        for shader_id in shaders {
            self.gl.attach_shader(program_id, shader_id.0);
        }
        self.gl.link_program(program_id);
        self.gl.validate_program(program_id);
        for shader_id in shaders {
            self.gl.delete_shader(shader_id.0);
        }
        let mut comp_status: [GLint; 1] = [0];
        unsafe {
            self.gl
                .get_program_iv(program_id, gl::LINK_STATUS, &mut comp_status)
        };
        if comp_status[0] == gl::FALSE as i32 {
            Err(CustomError::Other(self.gl.get_program_info_log(program_id)))
        } else {
            Ok(ProgramId(program_id))
        }
    }
}

pub fn build_shader_program(
    gl: Rc<dyn Gl>,
    shader_src_path: &str,
) -> Result<ProgramId, CustomError> {
    let shader_helper = ShaderHelper { gl: Rc::clone(&gl) };
    let (v_src, f_src) = shader_helper.parse_shader(shader_src_path)?;
    let v_shader_id = shader_helper.compile_shader(gl::VERTEX_SHADER, &v_src)?;
    let f_shader_id = shader_helper.compile_shader(gl::FRAGMENT_SHADER, &f_src)?;
    let program_id = shader_helper.link(&[v_shader_id, f_shader_id])?;
    Ok(program_id)
}

struct ShaderId(GLuint);
pub struct ProgramId(pub GLuint);

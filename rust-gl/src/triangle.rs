use gl;
use failure;
use crate::render_gl::{self, data, buffer, shader};
use crate::resources::Resources;


#[derive(VertexAttribPointers)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
struct Vertex {
    #[location = 0]
    pos: data::f32_f32_f32,
    #[location = 1]
    clr: data::u2_u10_u10_u10_rev_float,
}

pub struct Triangle {
    program: shader::Program,
    _vbo: buffer::ArrayBuffer,
    vao: buffer::VertexArray
}

impl Triangle {
    pub fn new(res: &Resources, gl: &gl::Gl) -> Result<Triangle, failure::Error> {
        
        // set up program
        let vert_shader = shader::Shader::from_resource(
            &gl,
            &res,
            "shaders/triangle.vert"
        ).unwrap();
        let frag_shader = shader::Shader::from_resource(
            &gl,
            &res,
            "shaders/triangle.frag"
        ).unwrap();
        let program = shader::Program::from_shaders(
            &gl,
            &[vert_shader, frag_shader]
        ).unwrap();

        // set up vertex buffer object
        let vertices: Vec<Vertex> = vec![
            Vertex {
                pos: (-0.5, -0.5, 0.0).into(),
                clr: (1.0, 0.0, 0.0, 1.0).into()
            }, // bottom right
            Vertex { 
                pos: ( 0.5, -0.5, 0.0).into(), 
                clr: (0.0, 1.0, 0.0, 1.0).into()
            }, // bottom left
            Vertex { 
                pos: ( 0.0,  0.5, 0.0).into(), 
                clr: (0.0, 0.0, 1.0, 1.0).into()
            }, // top
        ];
    
        let mut vbo = buffer::ArrayBuffer::new(&gl);
        vbo.bind();
        vbo.static_draw_data(&vertices);
        vbo.unbind();
    
        let vao = buffer::VertexArray::new(&gl);
        vao.bind();
        vbo.bind();
        Vertex::vertex_attrib_pointers(&gl);
        vbo.unbind();
        vao.unbind();

        Ok(Triangle {
            program,
            _vbo: vbo,
            vao,
        })
    }

    pub fn set_uniform<T: shader::UniformType>(&self, name: &str, value: &T) {
        self.program.set_uniform(name, value)
    }

    pub fn render(&self, gl: &gl::Gl) {
        self.program.set_used();
        self.vao.bind();

        unsafe {
            gl.DrawArrays(
                gl::TRIANGLES,
                0,
                3
            );
        }
    }
}
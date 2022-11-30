use notan::prelude::*;

//language=glsl
const VERTEX_SHADER: ShaderSource = notan::vertex_shader! {
  r#"
    #version 450
    layout(location = 0) in vec3 a_position;
    layout(location = 1) in vec2 a_texcoord;
    layout(location = 0) out vec2 v_texcoord;

    void main() {
        v_texcoord = a_texcoord;
        gl_Position = vec4(a_position, 1.0);
    }
    "#
};

//language=glsl
const FRAGMENT_SHADER: ShaderSource = notan::fragment_shader! {
  r#"
    #version 450
    precision mediump float;

    layout(location = 0) out vec4 outColor;
    layout(location = 0) in vec2 v_texcoord;

    layout(binding = 0) uniform sampler2D src;

    void main() {
        outColor = texture(src, v_texcoord);
    }
    "#
};

#[derive(Clone)]
pub struct RenderTextureCopier {
    pipeline: Pipeline,
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    uniform_buffer: Buffer,
}

impl RenderTextureCopier {
    pub fn new(gfx: &mut Graphics) -> Self {
        let vertex_info = VertexInfo::new()
            .attr(0, VertexFormat::Float32x3)
            .attr(1, VertexFormat::Float32x2);

        let pipeline = gfx
            .create_pipeline()
            .from(&VERTEX_SHADER, &FRAGMENT_SHADER)
            .with_color_blend(BlendMode::NONE)
            .with_vertex_info(&vertex_info)
            .with_texture_location(0, "src")
            .build()
            .unwrap();

        #[rustfmt::skip]
        let vertices = [
            //pos               //coords
            1.0,  1.0, 0.0,     1.0, 1.0,
            1.0, -1.0, 0.0,     1.0, 0.0,
            -1.0, -1.0, 0.0,    0.0, 0.0,
            -1.0, 1.0, 0.0,    0.0, 1.0
        ];

        #[rustfmt::skip]
        let indices = [
            0, 1, 3,
            1, 2, 3,
        ];

        let uniforms = [0.0]; // TODO: initialize with something better?

        let vertex_buffer = gfx
            .create_vertex_buffer()
            .with_info(&vertex_info)
            .with_data(&vertices)
            .build()
            .unwrap();

        let index_buffer = gfx
            .create_index_buffer()
            .with_data(&indices)
            .build()
            .unwrap();

        let uniform_buffer = gfx
            .create_uniform_buffer(0, "Locals")
            .with_data(&uniforms)
            .build()
            .unwrap();

        Self {
            pipeline,
            vertex_buffer,
            index_buffer,
            uniform_buffer,
        }
    }

    pub fn copy(&self, device: &mut Device, src: &Texture, dst: &RenderTexture) {
        let mut renderer = device.create_renderer();

        renderer.begin(Some(&ClearOptions::color(Color::TRANSPARENT)));

        renderer.set_pipeline(&self.pipeline);
        renderer.bind_texture_slot(0, 0, src);
        renderer.bind_buffers(&[
            &self.vertex_buffer,
            &self.index_buffer,
            &self.uniform_buffer,
        ]);
        renderer.draw(0, 6);
        renderer.end();

        device.render_to(dst, renderer.commands());
    }
}

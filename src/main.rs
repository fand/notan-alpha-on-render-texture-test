use notan::draw::*;
use notan::prelude::*;
use render_pipeline::RenderPipeline;
use workaround::Workaround;

mod render_pipeline;
mod workaround;

const W: f32 = 800.0;
const H: f32 = 600.0;
const TEX_W: f32 = 1200.0;
const TEX_H: f32 = 800.0;

#[derive(AppState)]
struct State {
    textures: Vec<Texture>,
    rt0: RenderTexture,
    rt1: RenderTexture,
    rt2: RenderTexture,
    workaround: Workaround,
    pipeline: RenderPipeline,
}

#[notan_main]
fn main() -> Result<(), String> {
    notan::init_with(init)
        .add_config(DrawConfig)
        .draw(draw)
        .build()
}

fn init(gfx: &mut Graphics) -> State {
    let textures = vec![
        texture(gfx, include_bytes!("ferris.png")),
        texture(gfx, include_bytes!("ferris_transparent.png")),
    ];

    let rt0 = gfx
        .create_render_texture(TEX_W as i32, TEX_H as i32)
        .with_format(TextureFormat::Rgba32)
        .build()
        .unwrap();
    let rt1 = gfx
        .create_render_texture(TEX_W as i32, TEX_H as i32)
        .with_format(TextureFormat::Rgba32)
        .build()
        .unwrap();
    let rt2 = gfx
        .create_render_texture(TEX_W as i32, TEX_H as i32)
        .build()
        .unwrap();

    let workaround = Workaround::new(gfx);

    let pipeline = RenderPipeline::new(gfx, Color::GRAY);

    State {
        textures,
        rt0,
        rt1,
        rt2,
        workaround,
        pipeline,
    }
}

fn texture(gfx: &mut Graphics, img: &[u8]) -> Texture {
    gfx.create_texture()
        .from_image(img)
        // .with_premultiplied_alpha()
        .build()
        .unwrap()
}

fn draw(app: &mut App, gfx: &mut Graphics, state: &mut State) {
    // let mut draw = gfx.create_draw();
    // draw.clear(Color::GRAY);
    state.pipeline.start_rendering();

    let width = W / 2.0;
    let scale = width / state.textures[0].width();

    // Draw normal PNG
    {
        state
            .pipeline
            .render_texture(&state.textures[0], (0.0, 0.0, W / 2.0, H / 2.0));
    }

    // Draw transparent PNG
    {
        state
            .pipeline
            .render_texture(&state.textures[1], (W / 2.0, 0.0, W / 2.0, H / 2.0));
    }

    // Draw transparent PNG with RenderTexture
    {
        state
            .workaround
            .draw(&mut gfx.device, &state.textures[1], &state.rt0, vec![1.0]);

        state
            .pipeline
            .render_texture(&state.rt2, (0.0, H / 2.0, W / 2.0, H / 2.0));
    }

    // Draw transparent PNG with RenderTexture twice
    {
        state
            .workaround
            .draw(&mut gfx.device, &state.textures[1], &state.rt1, vec![1.0]);

        state
            .workaround
            .draw(&mut gfx.device, &state.rt1, &state.rt2, vec![1.0]);

        state
            .pipeline
            .render_texture(&state.rt2, (W / 2.0, H / 2.0, W / 2.0, H / 2.0));
    }

    state.pipeline.blit(gfx);

    // gfx.render(&draw);
}

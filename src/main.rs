use notan::draw::*;
use notan::prelude::*;
use render_texture_copier::RenderTextureCopier;
use render_texture_drawer::RenderTextureDrawer;

mod render_texture_copier;
mod render_texture_drawer;

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
    rt_copier: RenderTextureCopier,
    rt_drawer: RenderTextureDrawer,
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

    let workaround = RenderTextureCopier::new(gfx);

    let pipeline = RenderTextureDrawer::new(gfx, Color::GRAY);

    State {
        textures,
        rt0,
        rt1,
        rt2,
        rt_copier: workaround,
        rt_drawer: pipeline,
    }
}

fn texture(gfx: &mut Graphics, img: &[u8]) -> Texture {
    gfx.create_texture().from_image(img).build().unwrap()
}

fn draw(app: &mut App, gfx: &mut Graphics, state: &mut State) {
    state.rt_drawer.start_rendering();

    // Draw normal PNG
    {
        state
            .rt_drawer
            .draw(&state.textures[0], (0.0, 0.0, W / 2.0, H / 2.0));
    }

    // Draw transparent PNG
    {
        state
            .rt_drawer
            .draw(&state.textures[1], (W / 2.0, 0.0, W / 2.0, H / 2.0));
    }

    // Draw transparent PNG with RenderTexture
    {
        state
            .rt_copier
            .copy(&mut gfx.device, &state.textures[1], &state.rt0);

        state
            .rt_drawer
            .draw(&state.rt2, (0.0, H / 2.0, W / 2.0, H / 2.0));
    }

    // Draw transparent PNG with RenderTexture twice
    {
        state
            .rt_copier
            .copy(&mut gfx.device, &state.textures[1], &state.rt1);

        state
            .rt_copier
            .copy(&mut gfx.device, &state.rt1, &state.rt2);

        state
            .rt_drawer
            .draw(&state.rt2, (W / 2.0, H / 2.0, W / 2.0, H / 2.0));
    }

    state.rt_drawer.render(gfx);
}

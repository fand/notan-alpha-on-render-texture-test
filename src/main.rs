use notan::draw::*;
use notan::prelude::*;

const W: f32 = 800.0;
const H: f32 = 600.0;
const TEX_W: f32 = 1200.0;
const TEX_H: f32 = 800.0;

#[derive(AppState)]
struct State {
    textures: Vec<Texture>,
    rt1: RenderTexture,
    rt2: RenderTexture,
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

    let rt1 = gfx
        .create_render_texture(TEX_W as i32, TEX_H as i32)
        .build()
        .unwrap();
    let rt2 = gfx
        .create_render_texture(TEX_W as i32, TEX_H as i32)
        .build()
        .unwrap();

    State { textures, rt1, rt2 }
}

fn texture(gfx: &mut Graphics, img: &[u8]) -> Texture {
    gfx.create_texture()
        .from_image(img)
        .with_premultiplied_alpha()
        .build()
        .unwrap()
}

fn draw(app: &mut App, gfx: &mut Graphics, state: &mut State) {
    let mut draw = gfx.create_draw();
    draw.clear(Color::GRAY);

    let width = W / 2.0;
    let scale = width / state.textures[0].width();

    draw.set_blend_mode(Some(BlendMode::OVER));

    // Draw normal PNG
    draw.image(&state.textures[0])
        .translate(0.0, 0.0)
        .scale(scale, scale);

    // Draw transparent PNG
    draw.image(&state.textures[1])
        .translate(width, 0.0)
        .scale(scale, scale);

    // Draw transparent PNG with RenderTexture
    {
        let mut d = state.rt1.create_draw();
        d.clear(Color::TRANSPARENT);
        d.image(&state.textures[1]).blend_mode(BlendMode::OVER);
        gfx.render_to(&state.rt1, &d);

        draw.image(&state.rt1)
            .translate(0.0, H / 2.0)
            .scale(scale, scale);
    }

    // Draw transparent PNG with RenderTexture twice
    {
        let mut d2 = state.rt2.create_draw();
        d2.clear(Color::TRANSPARENT);
        d2.image(&state.rt1).blend_mode(BlendMode::OVER);
        gfx.render_to(&state.rt2, &d2);

        draw.image(&state.rt2)
            .translate(width, H / 2.0)
            .scale(scale, scale);
    }

    gfx.render(&draw);
}

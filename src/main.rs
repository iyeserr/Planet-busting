use std::any::Any;

use macroquad::prelude::*;

struct PlayerInput {
    vector: Vec2,
    launches: Vec<Vec2>,
}
impl PlayerInput {
    fn new() -> Self {
        Self {
            vector: Vec2::ZERO,
            launches: Vec::new(),
        }
    }
}

struct StellarBody {
    collider: CircularPhysicsBody,
    texture: Texture2D,
}

impl StellarBody {
    fn new(position: Vec2, velocity: Vec2, radius: f32, texture: Texture2D) -> Self {
        Self {
            collider: CircularPhysicsBody {
                position,
                velocity,
                radius,
            },
            texture: texture,
        }
    }
}

struct CircularPhysicsBody {
    position: Vec2,
    velocity: Vec2,
    radius: f32,
}

struct Bullet {
    collider: CircularPhysicsBody,
    texture: Texture2D,
}

struct GameApp {
    res_textures: ResTextures,
    player_input: PlayerInput,
    player: StellarBody,
    planets: Vec<StellarBody>,
    bullets: Vec<Bullet>,
}

struct ResTextures {
    player: Texture2D,
    stellar_bodies: Vec<Texture2D>,
    bullet: Texture2D,
}

impl ResTextures {
    async fn load_all() -> Self {
        let player = load_texture("moon.png")
            .await
            .expect("Couldn't load player image.");
        let stellar_bodies = load_texture("earth.png")
            .await
            .expect("Couldn't load stellar body.");
        let bullet = load_texture("moon.png")
            .await
            .expect("Couldn't load bullet texture.");

        build_textures_atlas();

        Self {
            player: player,
            stellar_bodies: vec![stellar_bodies],
            bullet: bullet,
        }
    }
}

impl GameApp {
    fn new(res_textures: ResTextures) -> Self {
        let player_texture = res_textures.player.clone();

        Self {
            res_textures: res_textures,
            player_input: PlayerInput::new(),
            player: StellarBody::new(Vec2::ZERO, Vec2::ZERO, 10., player_texture),
            planets: Vec::new(),
            bullets: Vec::new(),
        }
    }

    fn get_input(&mut self) {
        self.player_input.vector = Vec2::ZERO;
        if is_key_down(KeyCode::W) {
            self.player_input.vector.y -= 1.;
        }
        if is_key_down(KeyCode::S) {
            self.player_input.vector.y += 1.;
        }
        if is_key_down(KeyCode::A) {
            self.player_input.vector.x -= 1.;
        }
        if is_key_down(KeyCode::D) {
            self.player_input.vector.x += 1.;
        }
        self.player_input.vector = self.player_input.vector.normalize_or_zero();

        if is_mouse_button_down(MouseButton::Left) {
            let mouse_pos = mouse_position();
            let mouse_pos = Vec2 {
                x: mouse_pos.0,
                y: mouse_pos.1,
            };
            self.player_input.launches.push(mouse_pos);
        }
    }

    fn update(&mut self, delta: f32) {
        // Update player
        self.player.collider.velocity = self.player_input.vector;
        self.player.collider.position += self.player.collider.velocity * 300. * delta;
    }

    fn render(&mut self) {
        let color = WHITE;

        draw_texture_ex(
            &self.player.texture,
            self.player.collider.position.x,
            self.player.collider.position.y,
            color,
            DrawTextureParams {
                dest_size: Some(Vec2 { x: 64., y: 64. }),
                source: None,
                rotation: 0.,
                flip_x: false,
                flip_y: false,
                pivot: None,
            },
        );
    }
}

#[macroquad::main("Lunar Light Launcher")]
async fn main() {
    set_pc_assets_folder("assets");

    let res_textures = ResTextures::load_all().await;
    let mut game_app = GameApp::new(res_textures);

    // Main loop
    loop {
        let delta = get_frame_time();

        //// Handle User Input
        game_app.get_input();

        //// Game Logic
        game_app.update(delta);

        //// Render
        game_app.render();
        next_frame().await
    }
}

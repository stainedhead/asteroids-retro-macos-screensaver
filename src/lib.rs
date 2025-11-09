pub mod renderer;
pub mod game;
pub mod macos;

use winit::window::Window;

pub struct AsteroidsScreensaver<'a> {
    pub renderer: renderer::Renderer<'a>,
    pub game_state: game::GameState,
}

impl<'a> AsteroidsScreensaver<'a> {
    pub async fn new(window: &'a Window) -> Self {
        let renderer = renderer::Renderer::new(window).await;
        let game_state = game::GameState::new();

        Self {
            renderer,
            game_state,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.game_state.update(delta_time);
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        self.renderer.render(&self.game_state)
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.renderer.resize(new_size);
        self.game_state.resize(new_size.width as f32, new_size.height as f32);
    }
}

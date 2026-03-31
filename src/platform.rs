use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{self, Color};
use ggez::event::{self, EventHandler};
use ggez::input::keyboard::KeyInput;

pub struct Platform {
}

impl Platform{
    pub fn process_input(self, k: &mut [u8; 16]) {
        // k.copy_from_slice(&self.keys); 
   }
}

impl EventHandler for Platform {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);
        canvas.finish(ctx)
    }

    fn key_down_event(&mut self, ctx: &mut Context, input: KeyInput, repeat: bool) -> GameResult {
        println!(
            "Key pressed: physical key {:?}, logical key {:?}, modifier {:?}, repeat: {}",
            input.event.physical_key, input.event.logical_key, input.mods, repeat
        );
        Ok(())
    }

    fn key_up_event(&mut self, ctx: &mut Context, input: KeyInput) -> GameResult {
        Ok(())
    }
    
}


mod chip8;
mod platform;
use crate::chip8::Chip8;
use crate::platform::Platform;
use ggez::{conf, GameResult, event};

fn main() -> GameResult {
    let mut cpu: Chip8;
    let mut platform: Platform = Platform {};

        let cb = ggez::ContextBuilder::new("input_test", "ggez").window_mode(
        conf::WindowMode::default()
            .fullscreen_type(conf::FullscreenType::Windowed)
            .resizable(true),
    );
    let (ctx, event_loop) = cb.build()?;

    event::run(ctx, event_loop, platform)
}

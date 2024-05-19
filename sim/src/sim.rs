

use ggez::mint::Point2;
use ggez::{Context, GameResult};
use ggez::graphics::{self, Color, Mesh};
use ggez::event::EventHandler;



pub struct Sim {
    circle: graphics::Mesh,
}

impl Sim {
    pub fn new(_ctx: &mut Context) -> Sim {
        // Load/create resources such as images here.

        let cirlce=  Mesh::new_circle(
            _ctx, 
            graphics::DrawMode::Stroke(graphics::StrokeOptions::DEFAULT), 
            Point2{x:100.0,y:200.0}, 
            200.0, 
            1.0, 
            Color::BLUE
        );

        Sim {
        circle: cirlce.expect("could not make cirlce"),  
        }
    }
}

impl EventHandler for Sim {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // Update code here...
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);
        // Draw code here...
        canvas.draw(&self.circle, graphics::DrawParam::default());
        canvas.finish(ctx)
    }
}

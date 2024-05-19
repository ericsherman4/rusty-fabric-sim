use ggez::mint::Point2;
use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{self, Color, Mesh};
use ggez::event::{self, EventHandler};

fn main() {
    // Make a Context.
    let (mut ctx, event_loop) = ContextBuilder::new("my_game", "Cool Game Author")
        .build()
        .expect("aieee, could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let my_game = MyGame::new(&mut ctx);

    // Run!
    event::run(ctx, event_loop, my_game);
}

struct MyGame {
    circle: graphics::Mesh,
}

impl MyGame {
    pub fn new(_ctx: &mut Context) -> MyGame {
        // Load/create resources such as images here.

        let cirlce=  Mesh::new_circle(
            _ctx, 
            graphics::DrawMode::Stroke(graphics::StrokeOptions::DEFAULT), 
            Point2{x:100.0,y:200.0}, 
            200.0, 
            1.0, 
            Color::BLUE
        );

        MyGame {
          circle: cirlce.expect("could not make cirlce"),  
        }
    }
}

impl EventHandler for MyGame {
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
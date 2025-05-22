use handler::AppHandler;

use seagull_lib::app::App;

mod handler;
mod pixelate;

fn main() {
    env_logger::init();

    let mut app = App::new(|gl, size| Ok(Box::new(AppHandler::new(gl, size)?)));
    app.run().unwrap();
}

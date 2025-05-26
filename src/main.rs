use handler::AppHandler;

use seagull_lib::app::App;

mod handler;
mod voronoi;
mod pixelate;

fn main() {
    env_logger::init();

    let mut app = App::new(|gl, window, size| Ok(Box::new(AppHandler::new(gl, window, size)?)));
    app.run().unwrap();
}

mod lib;
use lib::{Route, WebServer};

fn main() -> std::io::Result<()> {
    let index = Route::new("/", "GET");
    let test = Route::new("/test", "GET");

    let routes = vec![index, test];

    WebServer::new().base_dir("./html").bind("127.0.0.1:3000").register(routes).listen();

    Ok(())
}

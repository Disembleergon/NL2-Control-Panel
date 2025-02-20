use std::{process::exit, thread, time::Duration};

use enigo::{Direction, Enigo, Keyboard, Settings};
use keymap::ActionMap;
use local_ip_address::local_ip;
use rouille::{Request, Response};
use serde::Deserialize;

mod keymap;
mod ui;

#[derive(Deserialize)]
#[allow(non_snake_case)]
struct PostRequestBody {
    action: String,
    connectionTest: bool,
}

const PORT: u16 = 3000;

fn post_handler(req: &Request, action_map: &ActionMap) -> Response {
    let body: PostRequestBody = rouille::try_or_404!(rouille::input::json_input(req));
    if body.connectionTest {
        println!("Client connected!");
        return Response::empty_204();
    }

    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    let key = match action_map.get_key_by_action(body.action.as_str()) {
        Ok(k) => k,
        Err(_) => {
            exit(1);
        }
    };

    let _ = enigo.key(key, Direction::Press);
    thread::sleep(Duration::from_millis(300));
    let _ = enigo.key(key, Direction::Release);

    Response::empty_204()
}

fn main() {
    ui::intro();

    // read in the config of actions.json
    let action_map = match ActionMap::new("actions.json") {
        Ok(m) => m,
        Err(_) => {
            exit(1);
        }
    };

    let ip_address = local_ip().unwrap();
    println!("Server listening on http://{ip_address}:{PORT}/");

    rouille::start_server(format!("0.0.0.0:{PORT}"), move |request| {
        {
            let response = rouille::match_assets(request, "static");
            if response.is_success() {
                return response;
            }
        }

        rouille::router!(request,

            (GET) (/) => {
                rouille::Response::redirect_303("/index.html")
            },

            (GET) (/panel) => {
                rouille::Response::redirect_303("/panel.html")
            },

            (POST) (/) => {
                post_handler(request, &action_map)
            },

            _ => {
                rouille::Response::redirect_301("/index.html")
            }
        )
    });
}

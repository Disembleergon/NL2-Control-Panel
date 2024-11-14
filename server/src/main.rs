use local_ip_address::local_ip;
use rouille::{Request, Response};
use serde::Deserialize;

#[derive(Deserialize)]
#[allow(non_snake_case)]
struct PostRequestBody {
    action: String,
    connectionTest: bool,
}

const PORT: u16 = 3000;

fn post_handler(req: &Request) -> Response {
    let body: PostRequestBody = rouille::try_or_404!(rouille::input::json_input(req));
    if body.connectionTest {
        println!("Client connected!");
        return Response::empty_204();
    }

    // TODO key press

    Response::empty_204()
}

fn main() {
    // TODO: pretty banner with short explanation
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
                post_handler(request)
            },

            _ => {
                rouille::Response::redirect_301("/index.html")
            }
        )
    });
}

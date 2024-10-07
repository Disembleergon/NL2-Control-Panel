const PORT: u16 = 3000;

fn main() {
    // TODO: pretty banner with short explanation
    // TODO: show external ip address
    println!("Server listening on localhost:{PORT}");

    rouille::start_server(format!("127.0.0.1:{PORT}"), move |request| {
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
                // TODO: press keys according to specified action
                rouille::Response::text("key tap")
            },

            _ => {
                rouille::Response::redirect_301("/index.html")
            }
        )
    });
}

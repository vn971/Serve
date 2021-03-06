extern crate iron;
extern crate staticfile;
extern crate rustc_serialize;
extern crate docopt;

use iron::Iron;
use staticfile::Static;
use docopt::Docopt;

const DEFAULT_PORT: u16 = 8080;
const USAGE: &'static str = "
Serve files in the current directory via HTTP.

Usage:
    serve [<port>]
    serve (-h | --help)

Options:
    -h, --help  Show this screen
";

#[derive(RustcDecodable)]
struct Args {
    arg_port: Option<u16>,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|dopt| dopt.decode())
                            .unwrap_or_else(|e| e.exit());

    let port = args.arg_port.unwrap_or(DEFAULT_PORT);

    let addr = format!("localhost:{}", port);

    match Iron::new(Static::new(".")).http(&*addr) {
        Ok(_) => {
            println!("Listening on port {}", port);
        },
        Err(_) => {
            println!("Could not start server on port {}. \
                      Make sure that you have the required permissions \
                      and that the port is not already in use.", port);
        },
    }
}

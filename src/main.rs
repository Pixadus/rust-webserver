#[macro_use]
extern crate nickel;

use nickel::{Nickel, StaticFilesHandler};

fn main() {
    let mut server = Nickel::new();

    server.utilize(StaticFilesHandler::new("html/"));

    server.listen("0.0.0.0:8080");
}

use graphviz::enums::{Engine, OutputFormat, OutputLocation};
use graphviz::GVC;
use std::fs;
use std::io::stdout;
use std::io::Write;
use std::path::Path;

fn main() {
    let mut gvc = GVC::new(Engine::Dot, OutputFormat::PNG);
    stdout().write(
        &gvc.render(&fs::read_to_string("graph.dot").unwrap())
            .unwrap(),
    ).unwrap();
}

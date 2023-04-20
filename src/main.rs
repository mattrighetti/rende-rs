use std::collections::HashMap;
use usvg::{
    fontdb,
    TreeParsing,
    TreeTextToPath
};

fn new(s: &String, args: Vec<String>) -> Result<String, liquid::Error> {
    let args: HashMap<String, String> = args
        .chunks_exact(2)
        .map(|chunk| (chunk[0].clone(), chunk[1].clone()))
        .collect();

    let mut globals = liquid::model::Object::new();
    for (k,v) in args {
        let _ = globals
            .insert(
                k.into(),
                liquid::model::to_value(&v)?
            );
    }

    let template = liquid::ParserBuilder::with_stdlib()
        .build()?
        .parse(s)?;

    template.render(&globals)
}

fn from_path(path: String, args: Vec<String>) -> Result<String, liquid::Error> {
    let svg_string = std::fs::read_to_string(path).unwrap();
    new(&svg_string, args)
}

fn main() -> Result<(), liquid::Error> {
    let mut args: Vec<String> = std::env::args().collect();
    if args.len() < 3 || args.len() & 1 == 0 {
        println!("Usage:\n\tminimal <in-svg> <out-png> (<liquid_key> <liquid_value>)*");
        return Ok(());
    }

    let _ = args.remove(0);
    let input_path = args.remove(0);
    let output_path = args.remove(0);

    let svg = from_path(input_path, args)?;
    let mut tree = usvg::Tree::from_data(
        &svg.as_bytes(),
        &usvg::Options::default()
        )
        .unwrap();

    let mut fontdb = fontdb::Database::new();
    fontdb.load_system_fonts();
    tree.convert_text(&fontdb);

    let mut pixmap = tiny_skia::Pixmap::new(1920, 1080).unwrap();
    resvg::render(
        &tree,
        resvg::FitTo::Original,
        tiny_skia::Transform::default(),
        pixmap.as_mut(),
    )
    .unwrap();

    pixmap.save_png(output_path).unwrap();
    
    Ok(())
}
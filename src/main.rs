use usvg::{fontdb, TreeParsing, TreeTextToPath};

fn main() {
    let mut args: Vec<String> = std::env::args().collect();
    if args.len() < 3 || args.len() & 1 == 0 {
        println!("Usage:\n\tminimal <in-svg> <out-png> (<liquid_key> <liquid_value>)*");
        return;
    }

    let _ = args.remove(0);
    let input_path = args.remove(0);
    let output_path = args.remove(0);

    let mut opt = usvg::Options::default();
    // Get file's absolute directory.
    opt.resources_dir = std::fs::canonicalize(&input_path)
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()));

    let mut fontdb = fontdb::Database::new();
    fontdb.load_system_fonts();

    let svg_data = std::fs::read_to_string(&input_path).unwrap();
    
    let template = liquid::ParserBuilder::with_stdlib()
        .build().unwrap()
        .parse(&svg_data).unwrap();

    let mut globals = liquid::model::Object::new();

    for pair in args.chunks(2).collect::<Vec<&[String]>>() {
        println!("Injecting: {} -> {}", pair.get(1).unwrap(), pair.get(0).unwrap());
        let _ = globals
            .insert(
                pair.get(0).unwrap().into(),
                liquid::model::to_value(pair.get(1).unwrap()).unwrap()
            );
    }

    let output = template.render(&globals).unwrap();
    let mut tree = usvg::Tree::from_data(&output.as_bytes(), &opt).unwrap();
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
}
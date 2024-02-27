pub fn load_model(path: &str) {
    let object_path = format!("{}{}", "object/", path);

    let (models, _materials) = tobj::load_obj(object_path, &tobj::LoadOptions::default())
        .expect("Unexpected object parsing error");

    // Just going to use the first model, for loop is for later
    let model = models.get(0).unwrap();
    let mesh = &model.mesh;
    println!("Parsing {}", model.name);

    for pos in &mesh.positions {}
}

use glium::{glutin::surface::WindowSurface, IndexBuffer, VertexBuffer};

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    position: [f32; 3],
    tex_coords: [f32; 2],
}
implement_vertex!(Vertex, position, tex_coords);

#[derive(Copy, Clone, Debug)]
pub struct Normal {
    surface_normal: [f32; 3],
}
implement_vertex!(Normal, surface_normal);

pub fn load_model(
    path: &str,
    display: &glium::Display<WindowSurface>,
) -> (VertexBuffer<Vertex>, VertexBuffer<Normal>, IndexBuffer<u32>) {
    let object_path = format!("{}{}", "object/", path);

    let (models, _materials) = tobj::load_obj(object_path, &tobj::LoadOptions::default())
        .expect("Unexpected object parsing error");

    // Just going to use the first model, for loop is for later
    let model = models.get(0).unwrap();
    let mesh = &model.mesh;

    if 2 * &mesh.indices.len() != &mesh.normal_indices.len() + &mesh.texcoord_indices.len() {
        panic!("Unequal indices");
    }

    let mut vertices: Vec<Vertex> = vec![];
    let mut normals: Vec<Normal> = vec![];
    let mut indices: Vec<u32> = vec![];
    for i in 0..mesh.indices.len() {
        let vertex = batch3(&mesh.positions, mesh.indices[i] as usize);
        let normal = batch3(&mesh.normals, mesh.normal_indices[i] as usize);
        let uv = batch2(&mesh.texcoords, mesh.texcoord_indices[i] as usize);

        vertices.push(Vertex {
            position: vertex,
            tex_coords: uv,
        });
        normals.push(Normal {
            surface_normal: normal,
        });
        indices.push(i as u32);
    }

    let vertex_buffer = glium::VertexBuffer::new(display, &vertices);
    let normal_buffer = glium::VertexBuffer::new(display, &normals);
    let index_buffer = glium::IndexBuffer::new(
        display,
        glium::index::PrimitiveType::TrianglesList,
        &indices,
    );

    (
        vertex_buffer.unwrap(),
        normal_buffer.unwrap(),
        index_buffer.unwrap(),
    )
}

fn batch2(arr: &Vec<f32>, idx: usize) -> [f32; 2] {
    let tri = idx * 2;
    [arr[tri], arr[tri + 1]]
}

fn batch3(arr: &Vec<f32>, idx: usize) -> [f32; 3] {
    let tri = idx * 3;
    [arr[tri], arr[tri + 1], arr[tri + 2]]
}

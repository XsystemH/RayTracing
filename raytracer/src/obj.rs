use tobj;
use std::sync::Arc;
use crate::bvh::BvhNode;
use crate::color::Color;
use crate::hittable_list::HittableList;
use crate::material::Lambertian;
use crate::triangle::Triangle;
use crate::vec3::Point3;

pub fn read_obj(obj_filename: &str, scale: f64) -> HittableList {
    let mut object = HittableList::new();

    let filename = String::from(obj_filename);

    let obj = tobj::load_obj(
        format!("objects/{}", filename),
        &tobj::LoadOptions {
            single_index: false,
            triangulate: false,
            ignore_points: true,
            ignore_lines: true,
        });
    assert!(obj.is_ok());

    let (models, _materials) = obj.expect("Failed to load OBJ file");
    println!("# of models: {}", models.len());

    for (i, m) in models.iter().enumerate() {
        let mesh = &m.mesh;
        println!(
            "Number of Faces of model{}: {}",
            i,
            mesh.face_arities.len(),
        );
        println!("  model[{}] = {:?}", i, mesh.indices);
        let mut next_face = 0;
        for f in 0..mesh.face_arities.len() {
            let end = next_face + mesh.face_arities[f] as usize;
            let face_indices: Vec<_> = mesh.indices[next_face..end].iter().collect();
            println!("    face[{}] = {:?}", f, face_indices);
            next_face = end;

            let mat = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.8)));
            let mut p: [Point3;3] = [Color::white();3];
            let mut t = 0;
            let p0 = *face_indices[0] as usize;
            p[0] = Point3::new(
                mesh.positions[p0*3] as f64 * scale,
                mesh.positions[p0*3 + 1] as f64 * scale,
                mesh.positions[p0*3 + 2] as f64 * scale,
            );
            for v in face_indices {
                t += 1;
                p[1] = p[2];
                p[2] = Point3::new(
                    mesh.positions[*v as usize*3] as f64 * scale,
                    mesh.positions[*v as usize*3 + 1] as f64 * scale,
                    mesh.positions[*v as usize*3 + 2] as f64 * scale,
                );

                if t >= 3 {
                    object.add(Arc::new(Triangle::new(&p[0], &p[1], &p[2], mat.clone())));
                    println!("      Add Triangle:");
                    println!("        {:?}", p[0]);
                    println!("        {:?}", p[1]);
                    println!("        {:?}", p[2]);
                }
            }
        }
    }
    object = HittableList::new_from(Arc::new(BvhNode::from_list(&mut object)));
    object
}

use std::path::Path;

use crate::error::Error;
use emesh::Polygon;
use nalgebra::Point3;
use std::time::Instant;
use tracing::info;

pub fn run(
    input_file_path: impl AsRef<Path>,
    output_gltf_file_path: impl AsRef<Path>,
) -> Result<(), Error> {
    info!("Start run on {}", input_file_path.as_ref().display());
    let _now = Instant::now();

    let mut mesh = emesh::Mesh::new(vec![], vec![]);
    //mesh.add_vertex(Point3::default());
    //mesh.add_vertex(Point3::new(1.0, 0.0, 0.0));
    //mesh.add_vertex(Point3::new(1.0, 1.0, 1.0));
    //mesh.add_indexed_polygon(IndexedPolygon::new(vec![0, 1, 2])?);
    let polygon = Polygon::new(vec![
        Point3::default(),
        Point3::new(1.0, 0.0, 0.0),
        Point3::new(1.0, 1.0, 1.0),
    ])?;
    mesh.add_polygon(&polygon, None);
    // mesh.add_vertex(Point3::new(1.0, 2.0, 2.0));
    //mesh.add_vertex(Point3::new(1.0, 3.0, 0.0));
    //mesh.add_indexed_polygon(IndexedPolygon::new(vec![2, 3, 4])?);
    let polygon = Polygon::new(vec![
        Point3::new(1.0, 1.0, 1.0),
        Point3::new(1.0, 2.0, 2.0),
        Point3::new(1.0, 3.0, 0.0),
    ])?;
    mesh.add_polygon(&polygon, None);

    let graphics_triangle_mesh = emesh_converter::mesh_to_graphics(mesh)?;

    egraphics::io::EgraphicsExporter::new(output_gltf_file_path).finish(graphics_triangle_mesh)?;

    /*let citygml_model = ecitygml::io::CitygmlReader::from_path(input_file_path)?
        .finish()?;
    info!("Read model in {}ms", now.elapsed().as_millis());

    let envelope = egml::geometry::Envelope::new(
        corner_min.map_or_else(|| DirectPosition::MIN, |c| c.into()),
        corner_max.map_or_else(|| DirectPosition::MAX, |c| c.into()),
    )?;
    let citygml_model =
        ecitygml::transform::filter::filter_by_bounding_box(citygml_model, &envelope)?;

    let transformed_citygml_model = match translation_offset {
        Some(v) => ecitygml::transform::offset::offset_citygml_model(citygml_model, &v)?,
        _ => citygml_model,
    };

    let triangle_mesh = ecitygml_converter::citymodel_to_mesh(transformed_citygml_model);
    if triangle_mesh.is_empty() {
        info!("is empty");
        exit(1);
    }

    egraphics::io::EgraphicsExporter::new(output_gltf_file_path)
        .with_derive_obj_file(true)
        .with_create_parent_directories(true)
        .finish(triangle_mesh)?;*/

    Ok(())
}

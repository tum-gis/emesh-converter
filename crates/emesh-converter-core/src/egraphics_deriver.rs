use crate::error::Error;
use egraphics::{Triangle, TriangleMesh};

use nalgebra::Point3;
use rayon::iter::*;

pub fn mesh_to_graphics(mesh: emesh::Mesh) -> Result<TriangleMesh, Error> {
    let triangles: Vec<Triangle> = mesh
        .get_polygons()
        .into_par_iter()
        .map(polygon_to_triangle)
        .collect();

    let triangle_mesh = TriangleMesh::new(triangles)?;
    Ok(triangle_mesh)
}

fn polygon_to_triangle(polygon: emesh::Polygon) -> Triangle {
    let points: Vec<Point3<f32>> = polygon
        .vertices()
        .iter()
        .map(|p| Point3::new(p.x as f32, p.y as f32, p.z as f32))
        .collect();

    Triangle::from(points)
}

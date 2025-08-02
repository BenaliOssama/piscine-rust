pub use areas_volumes::*;

mod areas_volumes;


pub fn area_fit(
    (x, y): (usize, usize),
    kind: areas_volumes::GeometricalShapes,
    times: usize,
    (a, b): (usize, usize),
) -> bool {
    // code here
    let rectangle_area = areas_volumes::rectangle_area(x, y) as f64;
    
    let shape_area = match kind {
        GeometricalShapes::Square => areas_volumes::square_area(a) as f64 ,
        GeometricalShapes::Circle => areas_volumes::circle_area(a) as f64,
        GeometricalShapes::Rectangle => areas_volumes::rectangle_area(a, b) as f64,
        GeometricalShapes::Triangle => areas_volumes::triangle_area(a, b) as f64,
    };
    rectangle_area >= shape_area * times as f64
    // done here
}

pub fn volume_fit(
    (x, y, z): (usize, usize, usize),
    kind: areas_volumes::GeometricalVolumes,
    times: usize,
    (a, b, c): (usize, usize, usize),
) -> bool {
    
    // code here
    let box_volume = parallelepiped_volume(x, y, z) as f64;

    let shape_volume = match kind {
        GeometricalVolumes::Cube => cube_volume(a) as f64,
        GeometricalVolumes::Sphere => sphere_volume(a),
        GeometricalVolumes::Cone => cone_volume(a, b),
        GeometricalVolumes::TriangularPyramid => {
            let base = triangle_area(a, b);
            triangular_pyramid_volume(base, c)
        }
        GeometricalVolumes::Parallelepiped => parallelepiped_volume(a, b, c) as f64,
    };

    box_volume >= shape_volume * times as f64
}

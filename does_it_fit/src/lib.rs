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
    rectangle_area > shape_area * times as f64
    // done here
}

pub fn volume_fit(
    (x, y, z): (usize, usize, usize),
    kind: areas_volumes::GeometricalVolumes,
    times: usize,
    (a, b, c): (usize, usize, usize),
) -> bool {
    todo!()
}

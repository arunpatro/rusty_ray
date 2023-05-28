use nalgebra::{Vector2, Vector3};

fn lerp(a0: f64, a1: f64, w: f64) -> f64 {
    assert!(w >= 0.0);
    assert!(w <= 1.0);
    // TODO implement linear and cubic interpolation
    a0 + w * (a1 - a0)
}

// Computes the dot product of the distance and gradient vectors.
fn dot_grid_gradient(ix: i32, iy: i32, x: f64, y: f64, grid: &[[Vector2<f64>; 4]; 4]) -> f64 {
    // TODO: Compute the distance vector
    let distance = Vector2::new(x - ix as f64, y - iy as f64);

    // TODO: Compute and return the dot-product
    distance.dot(&grid[ix as usize][iy as usize])
}

// Compute Perlin noise at coordinates x, y
fn perlin(x: f64, y: f64, grid: &[[Vector2<f64>; 4]; 4]) -> f64 {
    // TODO: Determine grid cell coordinates x0, y0
    let x0 = x.floor() as i32;
    let x1 = x0 + 1;
    let y0 = y.floor() as i32;
    let y1 = y0 + 1;

    // Determine interpolation weights
    let sx = x - x0 as f64;
    let sy = y - y0 as f64;

    // Interpolate between grid point gradients
    let n0 = dot_grid_gradient(x0, y0, x, y, grid);
    let n1 = dot_grid_gradient(x1, y0, x, y, grid);

    let ix0 = lerp(n0, n1, sx);

    let n0 = dot_grid_gradient(x0, y1, x, y, grid);
    let n1 = dot_grid_gradient(x1, y1, x, y, grid);

    let ix1 = lerp(n0, n1, sx);

    lerp(ix0, ix1, sy)
}

pub fn procedural_texture(tu: f64, tv: f64) -> Vector3<f64> {
    let grid_size = 20;
    assert!(tu >= 0.0);
    assert!(tv >= 0.0);

    assert!(tu <= 1.0);
    assert!(tv <= 1.0);

    // TODO: uncomment these lines once you implement the perlin noise
    // let color = (perlin(tu * grid_size, tv * grid_size, grid) + 1.0) / 2.0;
    // Vector4::new(0.0, color, 0.0, 0.0)

    // Example for checkerboard texture
    let color =
        (tu * grid_size as f64).floor() as i32 + (tv * grid_size as f64).floor() as i32 % 2 == 0;
    Vector3::new(0.0, color as i32 as f64, 0.0)
}

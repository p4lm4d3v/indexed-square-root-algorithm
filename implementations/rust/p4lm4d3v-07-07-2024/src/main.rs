fn main() {
    for x in 0..20 {
        println!("idxsqrt({}) = {}", x, idxsqrt_simplified(x));
    }
}

fn idxsqrt_simplified(z: usize) -> f64 {
    if z == 0 || z == 1 {
        return z as f64;
    }
    if has_natural_root(z) || z < 5 {
        return (z as f64).sqrt();
    }

    let x: usize = lower_sqrt(z); // the closest lower number that has a natural root
    let y: usize = higher_sqrt(z); // the closest higher number that has a natural root

    let sqrt_x: f64 = (x as f64).sqrt();
    let sqrt_y: usize = (y as f64).sqrt() as usize;

    let idx: f64 = (z - x - 1) as f64;
    let n: f64 = (2 * sqrt_y - 3) as f64;
    let dotpart: f64 = idx / n;

    sqrt_x + dotpart
}

fn idxsqrt(z: usize) -> f64 {
    if z == 0 || z == 1 {
        return z as f64;
    }
    if has_natural_root(z) || z < 5 {
        return (z as f64).sqrt();
    }

    let x: usize = lower_sqrt(z); // the closest lower number that has a natural root
    let y: usize = higher_sqrt(z); // the closest higher number that has a natural root

    let range: Vec<usize> = ((x + 1)..(y - 1)).collect();
    let idx: usize = z - range[0];
    let n: usize = range.len();

    let sqrt_x: f64 = (x as f64).sqrt();
    let dotpart = idx as f64 / n as f64;

    sqrt_x + dotpart
}

fn lower_sqrt(mut z: usize) -> usize {
    while !has_natural_root(z) {
        z -= 1;
    }
    z
}

fn higher_sqrt(mut z: usize) -> usize {
    while !has_natural_root(z) {
        z += 1;
    }
    z
}

fn has_natural_root(n: usize) -> bool {
    let sqrt: f64 = (n as f64).sqrt().floor();
    sqrt * sqrt == n as f64
}

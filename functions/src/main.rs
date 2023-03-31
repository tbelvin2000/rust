
const DIM:usize = 20;
fn main() {
    let voltage = 1.;
    let max_diff = 1e-12;
    let mut diff = 1.;
    let mut phi = [[1000.0; DIM]; DIM];
    let mut phi_diff = phi;
    let mut i = 0;
    while diff > max_diff {
        for i in 0..DIM {
            for j in 0..DIM {
                if i == 0 { // Top boundary (y = 0)-> phi = V
                    phi[i][j] = voltage;
                } else if j == 0 || // Left side boundary (x = 0) -> phi = 0
                    j == DIM - 1 || // Right side boundary (x = L) -> phi = 0
                    i == DIM - 1 || // Bottom boundary (y = L) -> phi = 0
                    (i >= DIM/2 && j >= DIM/2) {   // Bottom right corner boundary -> phi = 0
                    phi[i][j] = 0.;
                } else {
                    phi[i][j] = 1./6. * (phi[i + 1][j] + phi[i - 1][j] + phi[i][j + 1] + phi[i][j - 1])
                }
            }
        }
        i += 1;
        diff = max(phi, phi_diff);
        phi_diff = phi;
    }
    println!("Completed after {i} loops with difference {diff}");
    println!("Phi(x = L/4, y = L/4) = {:0.4} mV", 1000.*phi[DIM/4][DIM/4]);
}

fn max(arr1: [[f64; DIM]; DIM], arr2: [[f64; DIM]; DIM]) -> f64 {
    let mut max_diff: f64 = 0.;
    for i in 0..DIM {
        for j in 0..DIM {
            let diff = (arr1[i][j] - arr2[i][j]).abs();
            if diff > max_diff {max_diff = diff}
        }
    }
    max_diff
}

// fn relaxation(a: u32, phi: &[i32], diff: f64) -> array {

// }
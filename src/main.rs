use ndarray::{Array, ArrayBase, ArrayD, Ix, IxDyn};
use ndarray::Dim;
use num_traits::identities::Zero;
use rand::{thread_rng, Rng};
use rand::distributions::{Distribution, Uniform};

#[derive(Debug, Clone)]
struct Node {
    state: bool,
}

//impl Zero for Node {
//    fn zero() -> usize {
//        0
//    }
//}

struct IsingModel {
//    nodes: ArrayBase<i16, D>,
}

fn main() {
    let lattice_size: usize = 10;
    let dim: usize = 2;
    let J: f64 = 1.0;
    let T: f64 = 1.0;

    let mut vec: Vec<usize> = Vec::with_capacity(dim);
    for i in 0..dim {
        vec.push(lattice_size);
    }

    println!("{:?}", vec);

    let ix_dyn = IxDyn(&vec);

    let mut grid = ArrayD::<i8>::zeros(ix_dyn);

    // Initialization
    let mut rng = thread_rng();
    for i in 0..lattice_size {
        for j in 0..lattice_size {
            let p = rng.gen_bool(1.0 / 2.0);
            if p {
                grid[[i, j]] = 1;
            } else {
                grid[[i, j]] = -1;
            }
        }
    }

    let num_iters = 100;
    let side = Uniform::new(0, lattice_size);

    // Iteration
    for iter in 0..num_iters {
        let (x, y) = (rng.sample(side), rng.sample(side));
        println!("{:?}", calc_energy(J, &grid));
//        println!("({}, {})", x, y);
    }

    println!("{:?}", grid);
}

fn calc_energy(J: f64, grid: &ArrayD::<i8>) -> f64 {
    let mut energy: f64 = 0.0;

    let adjacent: Vec<Vec<i8>> = vec![
        [0, -1, 0, 1].to_vec(),
        [1, 0, -1, 0].to_vec(),
    ];

    let shape = grid.shape();
    println!("{:?}", shape);
    
    for y in 0..shape[0] {
        for x in 0..shape[1] {
            for k in 0..4 {
                let dy: i8 = y as i8 + adjacent[0][k];
                let dx: i8 = x as i8 + adjacent[1][k];
                
                if 0 <= dx && dx < shape[1] as i8 && 0 <= dy && dy < shape[0] as i8 {
                    energy += -J * (grid[[dy as usize, dx as usize]] as f64) * (grid[[y, x]] as f64);
                }
            }
        }
    }

    energy
}
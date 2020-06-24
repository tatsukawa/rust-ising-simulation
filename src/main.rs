use ndarray::{ArrayBase, ArrayD, Ix, IxDyn};
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

    let mut A = ArrayD::<i8>::zeros(ix_dyn);

    // Initialization
    let mut rng = thread_rng();
    for i in 0..lattice_size {
        for j in 0..lattice_size {
            let p = rng.gen_bool(1.0 / 2.0);
            if p {
                A[[i, j]] = 1;
            } else {
                A[[i, j]] = -1;
            }
        }
    }

    let num_iters = 100;
    let side = Uniform::new(0, lattice_size);

    // Iteration
    for iter in 0..num_iters {
        let (x, y) = (rng.sample(side), rng.sample(side));


        println!("({}, {})", x, y);
    }

    println!("{:?}", A);
}

fn calc_energy() -> f64 {
    0.0
}
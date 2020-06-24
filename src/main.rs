use ndarray::{ArrayBase, ArrayD, Ix, IxDyn};
use ndarray::Dim;
use num_traits::identities::Zero;
use rand::{thread_rng, Rng};

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

    let mut vec: Vec<usize> = Vec::with_capacity(dim);
    for i in 0..dim {
        vec.push(lattice_size);
    }

    println!("{:?}", vec);
    
    let ix_dyn = IxDyn(&vec);

    let mut a = ArrayD::<i8>::zeros(ix_dyn);

    let mut rng = thread_rng();
    for i in 0..lattice_size {
        for j in 0..lattice_size {
            let p = rng.gen_bool(1.0 / 2.0);
            if p {
                a[[i, j]] = 1;
            } else {
                a[[i, j]] = -1;
            }
        }
    }

    let num_iters = 100;

    for iter in 0..num_iters {

    }

    println!("{:?}", a);
}

use std::fs::File;
use std::io::{Write, BufWriter};
use std::fmt::format;
use std::cell::Cell;
use ndarray::{Array, ArrayBase, ArrayD, Ix, IxDyn, Dim, LinalgScalar, Array2};
use rand::{thread_rng, Rng};
use rand::rngs::StdRng;
use rand::distributions::{Distribution, Uniform};


#[derive(Debug, Clone)]
pub struct IsingModel<A: LinalgScalar> {
    grid: ArrayD::<A>,
    rng: rand::rngs::StdRng
}

impl<A: LinalgScalar> IsingModel<A> {
    fn new(shape: Vec<usize>, seed_value: u8) -> Self {
        let ix_dyn = IxDyn(&shape);

        Self {
            grid: ArrayD::<A>::zeros(ix_dyn), 
            rng: rand::SeedableRng::from_seed([seed_value; 32])
        }
    }

    fn init<F>(&mut self, f: F) 
    where
        F: Fn(bool) -> A,
    {
        // Note: only support 2 dimentional grid.
        for i in 0..self.grid.shape()[0] {
            for j in 0..self.grid.shape()[1] {
                let p = self.rng.gen_bool(1.0 / 2.0);
                self.grid[[i, j]] = f(p);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::IsingModel;

    #[test]
    fn test_ising_model() {
        let dim: usize = 2;
        let lattice_size: usize = 100;
        let seed_value: u8 = 0;
        let mut shape: Vec<usize> = Vec::with_capacity(dim);
        for _i in 0..dim { shape.push(lattice_size); }

        let mut model = IsingModel::<i8>::new(shape, seed_value);

        let f = |val| {
            if val {
                return 1;
            } else {
                return -1;
            }
        };

        model.init(f);
    }
}

fn main() {
    let lattice_size: usize = 10;
    let dim: usize = 2;
    let J: f64 = 1.0;
    let beta: f64 = 0.5;
    let seed_value: u8 = 0;

    let mut vec: Vec<usize> = Vec::with_capacity(dim);
    for i in 0..dim {
        vec.push(lattice_size);
    }

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

    let num_iters = 40000;
    let side = Uniform::new(0, lattice_size);

    // File
    let mut file = BufWriter::new(File::create("./states.csv").unwrap());

    // Iteration
    for iter in 0..num_iters {
        let (x, y) = (rng.sample(side), rng.sample(side));

        let mut proposal_next_grid = grid.to_owned();
        proposal_next_grid[[y, x]] *= -1;

        let local_energy = calc_local_energy(J, &grid, (y, x));
        let flipped_local_energy = calc_local_energy(J, &proposal_next_grid, (y, x));

        let diff_energy = local_energy - flipped_local_energy;

        if flipped_local_energy < 0.0 {
            grid = proposal_next_grid;
        } else {
            let p: f64 = rng.gen::<f64>();
            if p < (-1.0 * beta * flipped_local_energy).exp() {
                grid = proposal_next_grid;
            }
        }

        // File output
        for y in 0..lattice_size {
            for x in 0..lattice_size {
                let s: String = format!("{},", grid[[y, x]]);
                file.write(s.as_bytes()).unwrap();
            }
            file.write("\n".as_bytes()).unwrap();
        }
    }

    println!("{:?}", grid);
}


fn calc_local_energy(J: f64, grid: &ArrayD::<i8>, point: (usize, usize)) -> f64 {
    let mut local_energy: f64 = 0.0;

    let shape = grid.shape();

    let adjacent: Vec<Vec<i8>> = vec![
        [0, -1, 0, 1].to_vec(),
        [1, 0, -1, 0].to_vec(),
    ];

    let y = point.0;
    let x = point.1;

    for i in 0..4 {
        let dy: i8 = y as i8 + adjacent[0][i];
        let dx: i8 = x as i8 + adjacent[1][i];
        
        if 0 <= dx && dx < shape[1] as i8 && 0 <= dy && dy < shape[0] as i8 {
            local_energy += -J * (grid[[dy as usize, dx as usize]] as f64) * (grid[[y, x]] as f64);
        }
    }

    local_energy
}

fn calc_global_energy(J: f64, grid: &ArrayD::<i8>) -> f64 {
    let mut energy: f64 = 0.0;

    let shape = grid.shape();
    println!("{:?}", shape);
    
    for y in 0..shape[0] {
        for x in 0..shape[1] {
            energy += calc_local_energy(J, &grid, (y, x));
       }
    }

    energy
}
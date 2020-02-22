use std::fs::File;
use std::io::{BufRead, BufReader};

use rand::{thread_rng, Rng};


pub struct Graph {
	pub n: usize,
	pub graph: Vec<Vec<u64>>
}

impl Graph {
	pub fn new(size: usize) -> Graph {
		Graph {
			n: size,
			graph: vec![vec![0; size]; size]
		}
	}

	pub fn contract(&mut self) {
		let mut rng = thread_rng();
		let i1: usize = rng.gen_range(0, &self.n);

		let mut nonzero_indices = Vec::new();

		for (j, adjacency) in self.graph[i1].iter().enumerate() {
			if *adjacency >= 1 {
				nonzero_indices.push(j);
			}
		}
//		println!("Step {}:\t number: {}\n{:?}\n", 200 - self.n, i1, nonzero_indices);
		let i2: usize = nonzero_indices[rng.gen_range(0, nonzero_indices.len())];

		for i in 0..self.n {
			self.graph[i2][i] += self.graph[i1][i];
			self.graph[i][i2] += self.graph[i][i1];
		}

		self.graph[i2][i2] = 0;

		for i in 0..self.n {
//			if i != i2 {
//				self.graph[i][i2] = self.graph[i2][i];
			self.graph[i].remove(i1);

		}
		self.graph.remove(i1);
		self.n -= 1;
	}
}

pub fn load_graph_from_file(fname: &str) -> Graph {
    let file = File::open(fname).unwrap();
    let f = BufReader::new(file).lines();
	let mut res = Vec::new();

	for line in f {
		let mut m = Vec::new();

		let l = & line.unwrap();
		let nums = l.split_whitespace();
		for n in nums.into_iter() {
			m.push(u64::from_str_radix(n, 10).unwrap() - 1);
		}

		res.push(m)
	}

	let mut graph = Graph::new(res.len());
	for (i, indices) in res.iter().enumerate() {
		for ind in indices {
			let j = *ind as usize;
			if i != j {
				graph.graph[i][j] = 1;
			}
		}
	}

	graph
}
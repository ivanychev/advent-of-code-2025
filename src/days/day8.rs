use crate::args::Args;
use crate::utils::input::read_input_lines;
use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
struct Vector {
    index: usize,
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Debug)]
struct VectorPair {
    v1: Vector,
    v2: Vector,
}

impl VectorPair {
    fn distance(&self) -> f64 {
        self.v1.distance(&self.v2)
    }
}

impl Vector {
    fn distance(&self, other: &Vector) -> f64 {
        let squared = (self.x - other.x) * (self.x - other.x)
            + (self.y - other.y) * (self.y - other.y)
            + (self.z - other.z) * (self.z - other.z);
        (squared as f64).sqrt()
    }
}

// struct Cluster {
//     points: Vec<Vector>,
// }

// impl Cluster {
//     fn distance(&self, other: &Cluster) -> f64 {
//         iproduct!(self.points.iter(), other.points.iter())
//             .map(|(v1, v2)| {
//                 v1.distance(v2)
//             })
//             .reduce(f64::min)
//             .unwrap()
//     }
// }

impl Vector {
    fn from_string_and_index(s: &str, index: usize) -> Self {
        let coords: Vec<i64> = s
            .trim()
            .split(',')
            .map(|part| part.parse::<i64>().unwrap())
            .collect();
        Vector {
            index,
            x: coords[0],
            y: coords[1],
            z: coords[2],
        }
    }
}

// impl From<&str> for Cluster {
//     fn from(s: &str) -> Self {
//         Cluster { points: vec![Vector::from(s)] }
//     }
// }

pub fn main(args: &Args) {
    let lines = read_input_lines(args.day as u32, args.input_tag.as_deref());
    let vectors: Vec<_> = lines
        .into_iter()
        .enumerate()
        .map(|(usize, line)| Vector::from_string_and_index(line.as_str(), usize))
        .collect();

    let num_pairs_to_merge = args
        .input_tag
        .as_deref()
        .map(|_tag| 10usize)
        .unwrap_or(vectors.len());

    let pairs_to_merge: Vec<_> = vectors
        .iter()
        .combinations(2)
        .map(|vectors| VectorPair {
            v1: *vectors[0],
            v2: *vectors[1],
        })
        .k_smallest_by(num_pairs_to_merge, |p1, p2| {
            let d1 = p1.distance();
            let d2 = p2.distance();
            f64::partial_cmp(&d1, &d2).unwrap()
        })
        .collect();

    let mut cluster_to_vectors: Vec<Vec<usize>> = vectors
        .iter()
        .enumerate()
        .map(|(i, _vec)| vec![i])
        .collect();
    let mut vector_to_cluster: Vec<usize> = vectors.iter().enumerate().map(|(i, _vec)| i).collect();

    for pair in pairs_to_merge {
        let cluster1 = vector_to_cluster[pair.v1.index];
        let cluster2 = vector_to_cluster[pair.v2.index];
        if cluster1 == cluster2 {
            continue;
        }

        let min_cluster = usize::min(cluster1, cluster2);
        let max_cluster = usize::max(cluster1, cluster2);

        {
            let slices = cluster_to_vectors.split_at_mut(max_cluster);
            let min_cluster_size_before = slices.0[min_cluster].len();
            slices.0[min_cluster].append(&mut slices.1[0]);
            for &vec_index in &slices.0[min_cluster][min_cluster_size_before..] {
                vector_to_cluster[vec_index] = min_cluster;
            }
        }
    }

    let cluster_sizes = cluster_to_vectors
        .into_iter()
        .filter(|vecs| !vecs.is_empty())
        .map(|vecs| vecs.len())
        .k_largest_by_key(3, |&size| size)
        .collect::<Vec<usize>>();

    println!(
        "{:?}",
        cluster_sizes[0] * cluster_sizes[1] * cluster_sizes[2]
    );
}

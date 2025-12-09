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

type BreakMergeFn<'a> =
    Box<dyn Fn(usize, Option<&Vec<usize>>, &'a VectorPair) -> (bool, &'a VectorPair) + 'a>;

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

pub fn main(args: &Args) {
    let lines = read_input_lines(args.day as u32, args.input_tag.as_deref());
    let vectors: Vec<_> = lines
        .into_iter()
        .enumerate()
        .map(|(usize, line)| Vector::from_string_and_index(line.as_str(), usize))
        .collect();

    let pairs_to_merge: Vec<_> = vectors
        .iter()
        .combinations(2)
        .map(|vectors| VectorPair {
            v1: *vectors[0],
            v2: *vectors[1],
        })
        .sorted_by(|p1, p2| {
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

    let break_merge_when: BreakMergeFn = match &args.part {
        1 => {
            let num_pairs_to_merge = args
                .input_tag
                .as_deref()
                .map(|_tag| 10usize)
                .unwrap_or(vectors.len());
            println!("Part 1: Number of pairs to merge: {}", num_pairs_to_merge);
            Box::new(move |index, _, vp| (index + 1 == num_pairs_to_merge, vp))
        }
        2 => Box::new(|_, cluster, vp| {
            (
                cluster.map(|c| c.len() == vectors.len()).unwrap_or(false),
                vp,
            )
        }),
        _ => panic!("Unsupported part for day {}", args.day),
    };

    let mut last_vector_pair: Option<&VectorPair> = None;
    for (idx, pair) in pairs_to_merge.iter().enumerate() {
        let cluster1 = vector_to_cluster[pair.v1.index];
        let cluster2 = vector_to_cluster[pair.v2.index];
        let break_cond = if cluster1 != cluster2 {
            let min_cluster = usize::min(cluster1, cluster2);
            let max_cluster = usize::max(cluster1, cluster2);

            let slices = cluster_to_vectors.split_at_mut(max_cluster);
            let min_cluster_size_before = slices.0[min_cluster].len();
            slices.0[min_cluster].append(&mut slices.1[0]);
            for &vec_index in &slices.0[min_cluster][min_cluster_size_before..] {
                vector_to_cluster[vec_index] = min_cluster;
            }

            break_merge_when(idx, Some(&slices.0[min_cluster]), pair)
        } else {
            break_merge_when(idx, None, pair)
        };

        if break_cond.0 {
            last_vector_pair = Some(break_cond.1);
            break;
        }
    }

    match &args.part {
        1 => {
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
        2 => {
            let v1 = last_vector_pair.unwrap().v1;
            let v2 = last_vector_pair.unwrap().v2;
            println!("{}", v1.x * v2.x)
        }
        _ => panic!("Unsupported part for day {}", args.day),
    }
}

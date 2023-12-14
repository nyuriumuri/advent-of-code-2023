use std::collections::{HashMap, HashSet};

use itertools::Itertools;

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<u32> {
    let grid: HashMap<Vertex, char> = input
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| (Vertex(x as i32, y as i32), c))
                .collect_vec()
        })
        .filter(|(_, c)| c != &'.')
        .collect();

    let edges: Vec<(Vertex, Vertex)> = grid
        .iter()
        .flat_map(|(v, c)| {
            let x = v.0;
            let y = v.1;
            let n = match c {
                'L' => vec![(x + 1, y), (x, y - 1)],
                'F' => vec![(x, y + 1), (x + 1, y)],
                '7' => vec![(x, y + 1), (x - 1, y)],
                '-' => vec![(x - 1, y), (x + 1, y)],
                '|' => vec![(x, y + 1), (x, y - 1)],
                'J' => vec![(x - 1, y), (x, y - 1)],
                _ => vec![],
            };
            n.into_iter()
                .map(|co| (v.clone(), Vertex(co.0, co.1)))
                .collect_vec()
        })
        .collect();

    let mut distances: HashMap<&Vertex, u32> = HashMap::new();

    let starting = grid.iter().find(|&(_, v)| *v == 'S').unwrap().0;
    let mut unvisited = HashSet::new();
    let mut visited = HashSet::new();
    for vertex in grid.keys() {
        distances.insert(vertex, 0);
    }

    unvisited.insert(starting);
    distances.insert(starting, 0);
    let get_neighbors = |cur: &Vertex| {
        edges
            .iter()
            .filter(|(v1, v2)| v1 == cur || v2 == cur)
            .map(|(v1, v2)| match (v1, v2) {
                (a, b) | (b, a) if b == cur => a,
                _ => unimplemented!(),
            })
            .collect_vec()
    };

    while !unvisited.is_empty() {
        // get unvisited element with least distance
        let cur = *unvisited
            .iter()
            .sorted_by(|&a, &b| {
                let d_a = distances.get(a).unwrap_or(&0);
                let d_b = distances.get(b).unwrap_or(&0);
                Ord::cmp(d_b, d_a)
                // distances.get(&a).u nwrap().cmp(distances.get(&b).unwrap())
            })
            .next()
            .unwrap();

        visited.insert(cur);
        unvisited.remove(cur);
        let neighbors: Vec<&Vertex> = get_neighbors(cur)
            .into_iter()
            .filter(|n| !visited.contains(n))
            .collect();
        let cur_d = distances.get(cur).unwrap().clone();
        for neighbor in neighbors {
            if let Some(d) = distances.get_mut(neighbor) {
                *d = u32::max(*d, cur_d + 1);
                unvisited.insert(neighbor);
            }
        }
    }

    Some(distances.into_iter().filter(|(v, d)| visited.contains(v)).map(|(v, d)| d).max().unwrap() / 2 + 1)
}

pub fn part_two(input: &str) -> Option<i32> {
    None
}
#[derive(Eq, PartialEq, PartialOrd, Clone, Hash, Debug)]
struct Vertex(i32, i32);
struct Graph {
    grid: HashMap<(i32, i32), char>,
    edges: Vec<((i32, i32), (i32, i32))>,
    starting: (i32, i32),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

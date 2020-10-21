mod heap;
mod parse;

use std::time::{Duration, Instant};

/// Graphe sous forme d'une matrice d'adjacence.
#[derive(Debug)]
pub struct Graph {
    // matrix: Vec<Vec<usize>>,
    matrix: Vec<Vec<usize>>,
}

/// Les stats d'un graphe
#[derive(Debug)]
pub struct Stats {
    pub nodes: usize,
    pub edges: usize,
    pub distance: Option<usize>,
    pub duration: Duration,
}

impl Graph {
    pub fn new(size: usize) -> Graph {
        Graph {
            matrix: vec![vec![0; 0]; size],
            // matrix: vec![vec![0; size]; size],
        }
    }
    pub fn add(&mut self, (begin, end): (usize, usize)) {
        let l = self.len();
        if begin >= l || end >= l {
            return;
        }
        self.matrix[begin].push(end)
        // self.matrix[begin][end] = 1
    }
    pub fn new_iter<I: std::iter::Iterator<Item = (usize, usize)>>(iter: I, size: usize) -> Graph {
        let mut g = Graph::new(size);
        iter.for_each(|arc| g.add(arc));
        g
    }
    pub fn new_tab(f: &str, size: usize) -> Result<Graph, String> {
        use std::io::prelude::*;
        Ok(Graph::new_iter(
            std::io::BufReader::new(
                std::fs::File::open(f).map_err(|err| format!("Fail to open {:?} {}", f, err))?,
            )
            .lines()
            .take_while(|r| r.is_ok())
            .filter_map(|r| r.ok())
            .map(|s| {
                s[..match s.find('#') {
                    Some(l) => l,
                    None => s.len(),
                }]
                    .trim()
                    .to_string()
            })
            .enumerate()
            .filter(|(_, l)| l.len() > 0)
            .filter_map(|(num, l)| match parse::tab(&l) {
                Ok(arc) => Some(arc),
                Err(e) => {
                    eprintln!("line {}, parse fail: {:?}", num, e);
                    None
                }
            }),
            size,
        ))
    }
    pub fn new_csv(f: &str, size: usize) -> Result<Graph, String> {
        use std::io::prelude::*;
        Ok(Graph::new_iter(
            std::io::BufReader::new(
                std::fs::File::open(f).map_err(|err| format!("Fail to open {:?} {}", f, err))?,
            )
            .lines()
            .take_while(|r| r.is_ok())
            .filter_map(|r| r.ok())
            .enumerate()
            .skip(1)
            .filter(|(_, l)| l.len() > 0)
            .filter_map(|(num, l)| match parse::csv(&l) {
                Ok(arc) => Some(arc),
                Err(e) => {
                    eprintln!("line {}, parse fail: {:?}", num, e);
                    None
                }
            }),
            size,
        ))
    }

    pub fn stats(&self) -> Stats {
        let before = Instant::now();
        Stats {
            nodes: self.len(),
            edges: self
                .matrix
                .iter()
                .map(|children| children.into_iter())
                .flatten()
                .count(),
            distance: self.distance_by_dijkstra(),
            duration: before.elapsed(),
        }
    }
    pub fn len(&self) -> usize {
        self.matrix.len()
    }
    fn distance_by_dijkstra(&self) -> Option<usize> {
        (0..self.len())
            .inspect(|origin| println!("origin={:?}", origin))
            .map(|origin| self.dijkstra(origin))
            .map(|v| v.into_iter())
            .flatten()
            .filter_map(|opt| opt)
            .max()
    }
    pub fn dijkstra(&self, origin: usize) -> Vec<Option<usize>> {
        let mut dist: Vec<Option<usize>> = vec![None; self.len()];
        let mut node_todo = heap::Heap::new();
        let mut min_theoretical: usize = 0;
        dist[origin] = Some(0);
        node_todo.push(origin);

        loop {
            match node_todo.next(min_theoretical, |s| dist[s]) {
                None => break,
                Some(parent) => {
                    min_theoretical = dist[parent].unwrap_or(0);
                    let minimum: usize = min_theoretical + 1;
                    self.children(parent).for_each(|child| {
                        if dist[child].is_none() {
                            node_todo.push(child);
                        }
                        dist[child] = Some(match dist[child] {
                            Some(old) if old < minimum => old,
                            _ => minimum,
                        });
                    })
                }
            }
        }

        dist
    }
    // Retourne tout les enfants, si ce n'est pas possible, on retourne un itérateur vide.
    pub fn children(&self, parent: usize) -> impl Iterator<Item = usize> + '_ {
        if parent >= self.len() {
            &[]
        } else {
            &self.matrix[parent][..]
        }
        .iter()
        .copied()
        // .enumerate()
        // .filter_map(|(i, &a)| match a > 0 {
        //     true => Some(i),
        //     false => None,
        // })
    }
}
#[test]
fn graph_dijkstra() {
    // From: https://fr.wikipedia.org/wiki/Matrice_d%27adjacence#Exemples

    let mut g = Graph::new(8);
    g.add((0, 1));
    g.add((0, 4));
    g.add((1, 6));
    g.add((3, 6));
    g.add((3, 2));
    g.add((5, 0));
    g.add((5, 1));
    g.add((5, 2));
    g.add((6, 7)); // moddifié par rapport à Wikipédia

    assert_eq!(
        vec!(
            Some(1),
            Some(1),
            Some(1),
            None,
            Some(2),
            Some(0), // 5 (origin)
            Some(2),
            Some(3),
        ),
        g.dijkstra(5)
    );

    assert_eq!(Some(3), g.distance_by_dijkstra());
}
#[test]
fn graph_children() {
    let mut g = Graph::new(8);
    g.add((0, 1));
    g.add((0, 4));
    g.add((1, 6));
    g.add((3, 6));
    g.add((3, 2));
    g.add((5, 0));
    g.add((5, 1));
    g.add((5, 2));
    g.add((7, 6));

    assert_eq!(vec![0; 0], g.children(2).collect::<Vec<usize>>());
    assert_eq!(vec![0, 1, 2], g.children(5).collect::<Vec<usize>>());
}

impl std::fmt::Display for Graph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        for i in self.matrix.iter() {
            for j in i.iter() {
                if *j == 0 {
                    f.write_str(". ")?
                } else {
                    f.write_str("1 ")?
                }
            }
            f.write_str("\r\n")?
        }
        Ok(())
    }
}

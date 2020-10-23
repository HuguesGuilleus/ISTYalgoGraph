mod heap;
mod parse;

use std::time::{Duration, Instant};

/// Un graphe, il contient une liste où chaque sommet a ses enfants.
/// ```
/// // From: https://fr.wikipedia.org/wiki/Matrice_d%27adjacence#Exemples
/// let mut g = graph::Graph::new(8);
/// g.add((0, 1));
/// g.add((0, 4));
/// g.add((1, 6));
/// g.add((3, 6));
/// g.add((3, 2));
/// g.add((5, 0));
/// g.add((5, 1));
/// g.add((5, 2));
/// g.add((7, 6));
///
/// println!("{}", g);
/// println!("{:?}", g.stats());
/// ```
#[derive(Debug)]
pub struct Graph {
    // matrix: Vec<Vec<usize>>,
    matrix: Vec<Vec<usize>>,
}

/// Les statistiques d'un graphe. Généré par `graph.stats()`.
#[derive(Debug)]
pub struct Stats {
    pub nodes: usize,
    pub edges: usize,
    pub max_degree: usize,
    pub average_degree: usize,
    pub degree_distrib: Vec<usize>,
    pub distance: Option<usize>,
    pub duration: Duration,
}

impl Graph {
    /// Créé un nouveau graphe vide. Pour ajouter des sommets utiliser les méthodes `add` ou `push`.
    pub fn new(size: usize) -> Graph {
        Graph {
            matrix: vec![vec![0; 0]; size],
            // matrix: vec![vec![0; size]; size],
        }
    }
    /// Ajoute un nouvel arc si `begin` et `end` sont inférieur à `self.len()`.
    pub fn add(&mut self, (begin, end): (usize, usize)) {
        let l = self.len();
        if begin >= l || end >= l {
            return;
        }
        self.matrix[begin].push(end)
        // self.matrix[begin][end] = 1
    }
    /// Ajoute un nouvel arc. On agrandit la liste des nœuds si besoin.
    pub fn push(&mut self, (begin, end): (usize, usize)) {
        use std::cmp::max;
        self.matrix
            .resize_with(max(self.len(), max(begin, end)) + 1, || vec![]);
        self.matrix[begin].push(end);
    }
    /// Charge un graphe à partir d'un itérateur d'arc. Si `size` n'est pas défini, le graphe sera
    /// agrandi pour contenir tout les sommets, sinon les sommets trop grands seront ignorés.
    pub fn new_iter<I: std::iter::Iterator<Item = (usize, usize)>>(
        iter: I,
        size: Option<usize>,
    ) -> Graph {
        let mut g = Graph::new(size.unwrap_or(0));
        let f: fn(&mut Graph, (usize, usize)) = match size {
            Some(..) => Graph::add,
            None => Graph::push,
        };
        iter.for_each(|arc| f(&mut g, arc));
        g
    }
    /// Charge un graphe à partir du fichier `f` en CSV ou TAB suivant le sont préfix.
    /// voir les méthodes `load_csv` et `load_tab` pour plus de détails.
    pub fn load(f: &str, size: Option<usize>) -> Result<Graph, String> {
        match f.strip_suffix(".csv") {
            Some(..) => return Graph::load_csv(f, size),
            _ => {}
        }

        match f.strip_suffix(".txt") {
            Some(..) => return Graph::load_tab(f, size),
            _ => {}
        }

        Err(format!("Unknow extension of the file {:?}", f))
    }

    /// Charge un graphe à partir du fichier pointé par `f`; le fichier peut contenir des lignes
    /// vides, des commentaires précédés d'un croisillon `'#'`. Les arcs sont constitué du sommet
    /// de départ des espaces et le sommet d'arrivée. Si `size` n'est pas défini, le graphe sera
    /// agrandi pour contenir tout les sommets, sinon les sommets trop grands seront ignorés.
    pub fn load_tab(f: &str, size: Option<usize>) -> Result<Graph, String> {
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
    /// Charge un graphe à partir du fichier pointé par `f`; la première ligne du fichier est ignorée,
    /// les autres lignes doivent contenir le sommet de départ, une virgule et le sommet d'arrivé.
    /// Si `size` n'est pas défini, le graphe sera agrandi pour contenir tout les sommets, sinon les
    /// sommets trop grands seront ignorés.
    pub fn load_csv(f: &str, size: Option<usize>) -> Result<Graph, String> {
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

    /// Génère les statistiques du graphe comme demandé par l'énnoncé.
    pub fn stats(&self) -> Stats {
        let before = Instant::now();
        let max_degree = self.matrix.iter().map(|n| n.len()).max().unwrap_or(0);
        let mut degree_distrib: Vec<usize> = vec![0; max_degree + 1];
        self.matrix
            .iter()
            .for_each(|n| degree_distrib[n.len()] += 1);

        Stats {
            nodes: self.len(),
            edges: self.edges(),
            max_degree: max_degree,
            average_degree: self.matrix.iter().map(|n| n.len()).sum::<usize>() / self.len(),
            degree_distrib: degree_distrib,
            distance: self.distance_by_dijkstra(),
            duration: before.elapsed(),
        }
    }
    /// Nombre total de sommets. Complexité constante.
    pub fn len(&self) -> usize {
        self.matrix.len()
    }
    /// Nombre total d'arrêtes. S'exécute en temps linéaire par rapport au nombre de sommets.
    pub fn edges(&self) -> usize {
        self.matrix.iter().map(|children| children.len()).sum()
    }
    /// Calcul la distance en prenant tous les sommets comme origine pour l'algorithme de Disktra.
    fn distance_by_dijkstra(&self) -> Option<usize> {
        let mut last_print = Instant::now();
        let m = (0..self.len())
            .inspect(|origin| {
                use std::io::prelude::*;
                if last_print.elapsed().subsec_millis() > 100 {
                    fn print_3digit(n: usize) {
                        if n == 0 {
                            return;
                        }
                        print_3digit(n / 1000);
                        print!("{:03} ", n % 1000);
                    }
                    print!("\x1b[K origin=");
                    print_3digit(*origin);
                    print!("\x1b[1G");
                    std::io::stdout().flush().unwrap();
                    last_print = Instant::now();
                }
            })
            .map(|origin| self.dijkstra(origin))
            .map(|v| v.into_iter())
            .flatten()
            .filter_map(|opt| opt)
            .max();

        print!("\x1b[K");
        m
    }
    /// Lance l'algorithme de Disktra sur le graphe.
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
            let mut node = i.clone();
            node.sort();
            let mut k = 0;
            for j in 0..self.len() {
                if node.len() > k && j == node[k] {
                    f.write_str("1 ")?;
                    k += 1;
                } else {
                    f.write_str(". ")?;
                }
            }
            f.write_str("\n")?
        }
        Ok(())
    }
}
#[test]
fn graph_display() {
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
    g.add((7, 6));

    assert_eq!(
        vec![
            ". 1 . . 1 . . .",
            ". . . . . . 1 .",
            ". . . . . . . .",
            ". . 1 . . . 1 .",
            ". . . . . . . .",
            "1 1 1 . . . . .",
            ". . . . . . . .",
            ". . . . . . 1 . \n",
        ]
        .join(" \n"),
        format!("{}", g)
    );
}

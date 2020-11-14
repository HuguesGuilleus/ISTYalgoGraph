mod parse;
mod printer;
mod stack2;
use rand;
use stack2::Stack2;

use std::fs::File;
use std::time::{Duration, Instant};

/// Un graphe, il contient la liste où chaque sommet a la liste de tous ses sommets voisins.
/// ```
/// // From: https://fr.wikipedia.org/wiki/Matrice_d%27adjacence#Exemples
/// let mut g = graph::Graph::new(Some(8));
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
    adjacency_list: Vec<Vec<usize>>,
}

/// Les statistiques d'un graphe. Généré par `graph.stats()`.
#[derive(Debug)]
pub struct Stats {
    /// Nombre de sommets
    pub nodes: usize,
    /// Nombre d'arrêtes
    pub edges: usize,
    /// Degrée moyen
    pub degree_average: f64,
    /// Fréquence d'apparition d'un degré. Longeur = degree_max+1
    pub degree_distrib: Vec<usize>,
    /// Degré maximale
    pub degree_max: usize,
    /// Distance = plus long plus court chemin.
    pub distance: Option<usize>,
    /// Temps pour obtenir ces statistiques.
    pub duration: Duration,
}

impl Graph {
    /// Génération de graphe avec le modèle d'Edgar Gilbert.
    pub fn gen_gilbert(size: usize) -> Graph {
        use rand::prelude::*;
        let mut g = Graph::new(Some(size));
        let mut r = rand::thread_rng();

        for i in 0..size {
            for j in 0..size {
                if r.gen::<bool>() {
                    g.adjacency_list[i].push(j);
                }
            }
        }

        g
    }
    /// Génère un graphe de Barabàsi-Albert. Utilise une complexité temporelle et mémoire linéiare.
    pub fn gen_barabasi_albert(size: usize) -> Graph {
        use rand::prelude::*;

        let mut g = Graph::new(Some(size));
        g.add((0, 1));
        g.add((1, 0));
        g.add((0, 2));
        g.add((2, 0));
        g.add((2, 1));
        g.add((1, 2));

        let mut p: Vec<usize> = Vec::with_capacity(size * 4);
        p.extend_from_slice(&[0, 0, 1, 1, 2, 2]);

        let mut r = rand::thread_rng();
        for i in 3..size {
            for _ in 0..2 {
                let j = p[r.gen::<usize>() % p.len()];
                p.push(i);
                p.push(j);
                g.add((j, i));
            }
        }

        g
    }
    /// Crée un nouveau graphe vide. Pour ajouter des sommets utiliser les méthodes `add` ou `push`.
    pub fn new(size: Option<usize>) -> Graph {
        Graph {
            adjacency_list: vec![vec![0; 0]; size.unwrap_or(0)],
        }
    }
    /// Ajoute un nouvel arc si `begin` et `end` sont inférieur à `self.len()`.
    pub fn add(&mut self, (begin, end): (usize, usize)) {
        let l = self.len();
        if begin >= l || end >= l {
            return;
        }
        self.adjacency_list[begin].push(end)
    }
    /// Ajoute un nouvel arc. On agrandit la liste des nœuds si besoin.
    pub fn push(&mut self, (begin, end): (usize, usize)) {
        use std::cmp::max;
        self.adjacency_list
            .resize_with(max(self.len(), max(begin, end)) + 1, || vec![]);
        self.adjacency_list[begin].push(end);
    }
    /// Charge un graphe à partir d'un itérateur d'arc. Si `size` n'est pas défini, le graphe sera
    /// agrandi pour contenir tout les sommets, sinon les sommets trop grands seront ignorés.
    pub fn new_iter<I: std::iter::Iterator<Item = (usize, usize)>>(
        iter: I,
        size: Option<usize>,
    ) -> Graph {
        let mut g = Graph::new(size);
        let f: fn(&mut Graph, (usize, usize)) = match size {
            Some(..) => Graph::add,
            None => Graph::push,
        };
        iter.for_each(|arc| f(&mut g, arc));
        g
    }
    /// Charge un graphe à partir du fichier `f` en CSV ou TAB suivant le sont préfixe.
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
    /// vides, des commentaires précédés d'un croisillon `'#'`. Les arêtes sont constitués de deux
    /// sommets séparér par des espaces ou des tabulations. Si `size` n'est pas défini, le graphe
    /// sera agrandi pour contenir tout les sommets, sinon les sommets trop grands seront ignorés.
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
    /// Enregistre le graphe dans le fichier `name`; le format est déterminé par les extentions qui
    /// penvent être ".txt" ou bien ".csv".
    pub fn save(&self, name: &str) -> Result<(), String> {
        use std::io::Write;
        let e = |err| format!("Fail to write into {:?} {}", name, err);
        let mut file = File::create(name).map_err(e)?;

        let writer = if name.strip_suffix(".csv").is_some() {
            write!(file, "id1,id2\n").map_err(e)?;
            parse::save_csv
        } else if name.strip_suffix(".txt").is_some() {
            write!(file, "# FromNodeId	ToNodeId\n").map_err(e)?;
            parse::save_txt
        } else {
            return Err(format!("Unknow extension of the file {:?}", name));
        };

        self.edge_list()
            .map(|couple| writer(&mut file, couple).map_err(e))
            .filter(|r| r.is_err())
            .next()
            .unwrap_or(Ok(()))
    }

    /// Génère les statistiques du graphe comme demandé par l’énoncé.
    pub fn stats(&self) -> Stats {
        let before = Instant::now();

        let edges = self.edges();
        let degree_max = self
            .adjacency_list
            .iter()
            .map(|n| n.len())
            .max()
            .unwrap_or(0);
        let mut degree_distrib: Vec<usize> = vec![0; degree_max + 1];
        self.adjacency_list
            .iter()
            .for_each(|n| degree_distrib[n.len()] += 1);

        Stats {
            nodes: self.len(),
            edges: edges,
            degree_average: (edges as f64) / (self.len() as f64),
            degree_distrib: degree_distrib,
            degree_max: degree_max,
            distance: self.distance_by_bfs(),
            duration: before.elapsed(),
        }
    }
    /// Nombre total de sommets. Complexité constante.
    pub fn len(&self) -> usize {
        self.adjacency_list.len()
    }
    /// Nombre total d’arêtes. Complexité: O(S).
    pub fn edges(&self) -> usize {
        self.adjacency_list
            .iter()
            .map(|children| children.len())
            .sum()
    }
    /// Calcule la distance en prenant tous les sommets comme origine et leur applique un parcours
    /// en largeur. Complexité: O(S*(S+A))
    fn distance_by_bfs(&self) -> Option<usize> {
        let mut p = printer::Printer::new();
        (0..self.len())
            .inspect(|origin| p.print(*origin))
            .map(|origin| self.bfs(origin))
            .map(|v| v.into_iter())
            .flatten()
            .filter_map(|opt| opt)
            .max()
    }
    /// Applique l’algorithme de parcours en largeur (*Breadth-first search* en anglais) sur le
    /// sommet `origin`. Complexité: O(A+S).
    pub fn bfs(&self, origin: usize) -> Vec<Option<usize>> {
        let mut dist: Vec<Option<usize>> = vec![None; self.len()];
        let mut node_todo = Stack2::new();
        dist[origin] = Some(0);
        node_todo.push_back(origin);

        loop {
            match node_todo.pop_front() {
                None => break,
                Some(parent) => {
                    let minimum: usize = dist[parent].unwrap_or(0) + 1;
                    self.children(parent).for_each(|child| match dist[child] {
                        Some(..) => {}
                        None => {
                            dist[child] = Some(minimum);
                            node_todo.push_back(child);
                        }
                    })
                }
            }
        }

        dist
    }
    /// Retourne tout les enfants, si ce n'est pas possible, on retourne un itérateur vide.
    /// Complexité constante.
    pub fn children(&self, parent: usize) -> impl Iterator<Item = usize> + '_ {
        if parent >= self.len() {
            &[]
        } else {
            &self.adjacency_list[parent][..]
        }
        .iter()
        .copied()
    }
    /// Retourne un itérateur avec chaque arrêtes du graphe.
    pub fn edge_list(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        self.adjacency_list
            .iter()
            .enumerate()
            .map(|(p, parent): (usize, _)| parent.iter().map(move |child: &usize| (p, *child)))
            .flatten()
    }
}
#[test]
fn graph_bfs() {
    // From: https://fr.wikipedia.org/wiki/Matrice_d%27adjacence#Exemples

    let mut g = Graph::new(Some(8));
    g.add((0, 1));
    g.add((0, 4));
    g.add((1, 6));
    g.add((3, 6));
    g.add((3, 2));
    g.add((5, 0));
    g.add((5, 1));
    g.add((5, 2));
    g.add((6, 7)); // modifié par rapport à Wikipédia

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
        g.bfs(5)
    );

    assert_eq!(Some(3), g.distance_by_bfs());
}
#[test]
fn graph_children() {
    let mut g = Graph::new(Some(8));
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
        for (line, i) in self.adjacency_list.iter().enumerate() {
            let mut nodes = i.clone();
            nodes.sort();
            let mut nodes = nodes.iter().peekable();

            for j in 0..self.len() {
                let mut nb = 0;
                loop {
                    match nodes.peek() {
                        Some(n) if **n == j => {
                            nb += 1;
                            nodes.next();
                        }
                        _ => break,
                    }
                }
                match nb {
                    0 if j == line => f.write_str("* "),
                    0 => f.write_str(". "),
                    n => write!(f, "{} ", n),
                }?;
            }
            f.write_str("\n")?;
        }
        Ok(())
    }
}
#[test]
fn graph_display() {
    // From: https://fr.wikipedia.org/wiki/Matrice_d%27adjacence#Exemples
    let mut g = Graph::new(Some(8));
    g.add((0, 1));
    g.add((0, 4));
    g.add((1, 6));
    g.add((3, 6));
    g.add((3, 2));
    g.add((5, 0));
    g.add((5, 1));
    g.add((5, 2));
    g.add((7, 6));
    g.add((7, 6)); // Double

    assert_eq!(
        vec![
            "* 1 . . 1 . . .",
            ". * . . . . 1 .",
            ". . * . . . . .",
            ". . 1 * . . 1 .",
            ". . . . * . . .",
            "1 1 1 . . * . .",
            ". . . . . . * .",
            ". . . . . . 2 * \n",
        ]
        .join(" \n"),
        format!("{}", g)
    );
}

mod parse;
mod printer;

use rand;
use std::collections::VecDeque;
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
    pub distance: usize,
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
    pub fn add(&mut self, (a, b): (usize, usize)) {
        let l = self.len();
        if a >= l || b >= l {
            return;
        }
        self.adjacency_list[a].push(b);
        self.adjacency_list[b].push(a);
    }
    /// Ajoute un nouvel arc. On agrandit la liste des nœuds si besoin.
    pub fn push(&mut self, (a, b): (usize, usize)) {
        use std::cmp::max;

        let l = max(self.len(), max(a, b) + 1);
        self.adjacency_list.resize_with(l, || vec![]);

        self.adjacency_list[a].push(b);
        self.adjacency_list[b].push(a);
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
            .filter(|couple| couple.0 < couple.1)
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
            degree_average: ((edges * 2) as f64) / (self.len() as f64),
            degree_distrib: degree_distrib,
            degree_max: degree_max,
            distance: self.distance(),
            duration: before.elapsed(),
        }
    }
    /// Nombre total de sommets. Complexité constante.
    pub fn len(&self) -> usize {
        self.adjacency_list.len()
    }
    /// Nombre total d'arêtes. Complexité: O(S).
    pub fn edges(&self) -> usize {
        self.adjacency_list
            .iter()
            .map(|children| children.len())
            .sum::<usize>()
            / 2
    }
    /// Calcule la distance en précalculant la distance des sous-arbres, séléctionne les nœuds avec
    /// un sous-arbre ou à l'extrémité du graphe, et leur applique un parcours en largeur.
    /// Complexité: minimal O(S+A); maximal: O(S*(S+A))
    fn distance(&self) -> usize {
        use std::cmp::max;

        let mut p = printer::Printer::new();

        p.print("mark_tree", 0);
        let (whitelist, subtree, mut longest) = self.mark_tree();

        // Applique BFS sur chaque composante connexe.
        let mut dist = vec![0; self.len()];
        for n in 0..self.len() {
            if !whitelist[n] || dist[n] > 0 {
                continue;
            }
            p.print("first seen", n);
            self.bfs(n, &whitelist, &mut |n, d| {
                dist[n] = d;
            });
        }

        // Séléctionne les nœuds pouvant donner le diamètre.
        p.print("selecting", 0);
        let mut origins = whitelist.clone();
        for n in (0..self.len()).filter(|n| whitelist[*n]) {
            let dist_n = dist[n];
            let have_not_subtree = subtree[n] == 0;
            for c in self.children(n, &whitelist) {
                if dist_n < dist[c] {
                    if have_not_subtree {
                        origins[n] = false;
                    }
                } else if subtree[c] == 0 {
                    origins[c] = false;
                }
            }
        }

        // Classe les nœuds séléctionnés
        let selected: Vec<usize> = (0..self.len()).filter(|n| origins[*n]).collect();

        // Récupère les nœuds séléctionnés et mesure le diamètre.
        selected
            .into_iter()
            .inspect(|origin| p.print("diameter", *origin))
            .for_each(|origin| {
                let min = subtree[origin];
                self.bfs(origin, &whitelist, &mut |n, d| {
                    longest = max(longest, min + d + subtree[n]);
                });
            });

        longest
    }
    /// Applique l'algorithme de parcours en largeur (*Breadth-first search* en anglais) sur le
    /// sommet `origin`. Complexité: O(A+S). La closure `f` prend le nœud et sa distance minimal
    /// depuis l'origine. whitelist les sommets ignorées.
    pub fn bfs<F>(&self, origin: usize, whitelist: &'_ [bool], f: &mut F) -> Vec<Option<usize>>
    where
        F: FnMut(usize, usize),
    {
        let mut dist: Vec<Option<usize>> = vec![None; self.len()];
        let mut node_todo = VecDeque::with_capacity(self.len() / 2);
        dist[origin] = Some(0);
        node_todo.push_back(origin);

        loop {
            match node_todo.pop_front() {
                None => break,
                Some(parent) => {
                    let d = dist[parent].unwrap_or(0);
                    f(parent, d);
                    let minimum: usize = d + 1;
                    self.children(parent, &whitelist).for_each(|child| {
                        if dist[child].is_none() {
                            dist[child] = Some(minimum);
                            node_todo.push_back(child);
                        }
                    })
                }
            }
        }

        dist
    }
    /// Recherche tous les sous-arbres. Retourne un triplet:
    ///   - Tableau des nœuds appartenant à des sous-arbres (plus pris en compte)
    ///   - Tableau des poids des sous-arbres.
    ///   - Distance maximal trouvée.
    fn mark_tree(&self) -> (Vec<bool>, Vec<usize>, usize) {
        use std::cmp::max;

        let mut whitelist = vec![true; self.len()]; // Les nœuds appartenant à des sous-arbres.
        let mut weight = vec![0; self.len()]; // Plus longue branche dans le sous-arbre.
        let mut longest = 0; // La plus longue branche.

        for node in 0..self.len() {
            if !whitelist[node] {
                continue;
            }

            let mut parent = node;
            let mut deep = 0;
            loop {
                // Les deux voisins si ils existent.
                let (a, b): (Option<usize>, Option<usize>);
                {
                    let mut it = self.children(parent, &whitelist);
                    a = it.next();
                    b = it.next();
                }
                match (a, b) {
                    (Some(child), None) => {
                        whitelist[parent] = false;
                        let parent_deep = weight[parent];
                        longest = max(longest, deep + parent_deep);
                        deep = 1 + max(deep, parent_deep);
                        parent = child;
                    }
                    (None, None) => {
                        whitelist[parent] = false;
                        longest = max(longest, deep);
                        break;
                    }
                    _ => {
                        let parent_deep = weight[parent];
                        longest = max(longest, deep + parent_deep);
                        weight[parent] = max(deep, parent_deep);
                        break;
                    }
                }
            }
        }

        (whitelist, weight, longest)
    }
    /// Retourne tout les enfants, si ce n'est pas possible, on retourne un itérateur vide.
    /// Complexité constante.
    pub fn children<'a>(
        &'a self,
        parent: usize,
        whitelist: &'a [bool],
    ) -> impl Iterator<Item = usize> + 'a {
        if parent >= self.len() {
            &[]
        } else {
            &self.adjacency_list[parent][..]
        }
        .iter()
        .copied()
        .filter(move |n| whitelist[*n])
    }
    /// Retourne un itérateur avec chaque arrête du graphe.
    pub fn edge_list(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        self.adjacency_list
            .iter()
            .enumerate()
            .map(|(p, parent): (usize, _)| parent.iter().map(move |child: &usize| (p, *child)))
            .flatten()
    }
}
#[test]
fn graph_add() {
    let mut g = Graph::new(Some(2));
    g.add((0, 1));

    assert_eq!(2, g.adjacency_list.len());
    assert_eq!(vec![1], g.adjacency_list[0]);
    assert_eq!(vec![0], g.adjacency_list[1]);
}
#[test]
fn graph_push() {
    let mut g = Graph::new(None);
    g.push((0, 1));

    assert_eq!(2, g.adjacency_list.len());
    assert_eq!(vec![1], g.adjacency_list[0]);
    assert_eq!(vec![0], g.adjacency_list[1]);
}
#[test]
fn graph_distance() {
    // Source: https://fr.wikipedia.org/wiki/Matrice_d%27adjacence#Exemples mais non orienté
    let mut g = Graph::new(Some(8));
    g.add((0, 1));
    g.add((0, 4));
    g.add((1, 6));
    g.add((3, 6));
    g.add((3, 2));
    g.add((5, 0));
    g.add((5, 1));
    g.add((5, 2));
    g.add((6, 7));

    let dist = vec![
        Some(1),
        Some(1),
        Some(1),
        Some(2),
        Some(2),
        Some(0), // 5 (origin)
        Some(2),
        Some(3),
    ];

    assert_eq!(
        dist,
        g.bfs(5, &vec![true; 8], &mut |n, d| if dist[n] != Some(d) {
            panic!("Node: {} and distance: {} is wrong", n, d);
        })
    );

    assert_eq!(4, g.distance());
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
    let w = vec![true; g.len()];

    assert_eq!(vec![3, 5], g.children(2, &w).collect::<Vec<usize>>());
    assert_eq!(vec![0, 1, 2], g.children(5, &w).collect::<Vec<usize>>());
}
#[test]
fn test_mark_tree() {
    let mut g = Graph::new(Some(15));
    g.add((0, 1));
    g.add((0, 2));
    g.add((1, 2));
    g.add((0, 3));
    g.add((1, 4));
    g.add((4, 5));
    g.add((4, 6));
    g.add((6, 7));
    g.add((4, 8));
    g.add((8, 10));
    g.add((8, 9));
    g.add((9, 11));
    g.add((12, 13));
    // 14 est seul

    let (whitelist, weight, longest) = g.mark_tree();
    assert_eq!(
        vec![
            true, true, true, false, false, false, false, false, false, false, false, false, false,
            false, false,
        ],
        whitelist
    );
    assert_eq!(vec![1, 4, 0], weight[..3]);
    // vec![Weight::new(1, 1), Weight::new(4, 5), Weight::NULL],
    // weight[..3]
    assert_eq!(5, longest);
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
            "* 1 . . 1 1 . .",
            "1 * . . . 1 1 .",
            ". . * 1 . 1 . .",
            ". . 1 * . . 1 .",
            "1 . . . * . . .",
            "1 1 1 . . * . .",
            ". 1 . 1 . . * 2",
            ". . . . . . 2 * \n",
        ]
        .join(" \n"),
        format!("{}", g)
    );
}

fn main() -> Result<(), String> {
    // Nombre de nœud au maximum dans le graphe.
    // let size: Option<usize> = Some(20_000);
    let size: Option<usize> = None;

    // Fichier à charger.
    // let f = "../db/FacebookSites.csv";
    let f = "../db/GitHub.csv";
    // let f = "../db/RoadNetwork.txt";
    // let f = "../db/twitchDE.csv";
    // let f = "../db/Wikipedia1.csv";
    // let f = "../db/Wikipedia2.csv";

    println!("graph: {:#?}", f);
    let mut s = graph::Graph::load(f, size)?.stats();
    s.degree_distrib = vec![];
    println!("{:#?}", s);

    Ok(())
}

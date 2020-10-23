fn main() -> Result<(), String> {
    // Nombre de nœud au maximum dans le graphe.
    let size: Option<usize> = Some(20_000);
    // let size: Option<usize> = None;

    // Fichier à charger.
    // let f = "../x/db/FacebookSites.csv";
    let f = "../x/db/GitHub.csv";
    // let f = "../x/db/RoadNetwork.txt";
    // let f = "../x/db/twitchDE.csv";
    // let f = "../x/db/Wikipedia1.csv";
    // let f = "../x/db/Wikipedia2.csv";

    let mut s = graph::Graph::load(f, size)?.stats();
    s.degree_distrib = vec![];
    println!("{:#?}", s);

    Ok(())
}

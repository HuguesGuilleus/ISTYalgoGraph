fn main() -> Result<(), String> {
    let fs = vec![
        "../db/RoadNetwork.txt",
        "../db/FacebookSites.csv",
        "../db/GitHub.csv",
        "../db/twitchDE.csv",
        "../db/Wikipedia1.csv",
        "../db/Wikipedia2.csv",
    ];

    for f in fs {
        println!("===> {}", f);
        let s = graph::Graph::load(f, None)?.stats();
        println!("     {:?} /// {:?}", s.duration, s.distance);
    }

    Ok(())
}

#![feature(termination_trait_lib, process_exitcode_placeholder)]

fn main() -> finalreturn::R {
    let f = "../x/db/FacebookSites.csv";
    // let f = "../x/db/GitHub.csv";
    // let f = "../x/db/RoadNetwork.txt";
    // let f = "../x/db/twitchDE.csv";
    // let f = "../x/db/Wikipedia1.csv";()
    // let f = "../x/db/Wikipedia2.csv";()

    println!("Loading ...");
    println!("{:#?}", graph::Graph::load(f, Some(10_000))?.stats());

    Ok(())
}

mod finalreturn {
    pub type R = Result<(), FinalReturn>;

    pub struct FinalReturn {
        s: String,
    }
    impl std::convert::From<String> for FinalReturn {
        fn from(s: String) -> Self {
            FinalReturn { s: s }
        }
    }
    impl std::fmt::Debug for FinalReturn {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
            f.write_str(&self.s)
        }
    }
    impl std::process::Termination for FinalReturn {
        fn report(self) -> i32 {
            eprintln!("Error: {}", self.s);
            return std::process::ExitCode::FAILURE.report();
        }
    }
}

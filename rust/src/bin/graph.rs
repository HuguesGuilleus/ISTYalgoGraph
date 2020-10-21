#![feature(termination_trait_lib, process_exitcode_placeholder)]

fn main() -> finalreturn::R {
    println!("Load ...");

    let g = graph::Graph::new_csv("../x/db/GitHub.csv", 10_000)?;
    // let g = graph::Graph::new_tab("../x/db/RoadNetwork.txt", 1_965_206)?;
    // let g = graph::Graph::new_tab("../x/db/2RoadNetwork.txt", 470)?;

    println!("Gen stats ...");
    let s = g.stats();
    println!("g.stats(): {:?}", s);

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

use std::time::Instant;

pub struct Printer(Instant);

impl Printer {
    pub fn new() -> Printer {
        Printer(Instant::now())
    }
    // Print the name of the current operation op and the origni node.
    pub fn print(&mut self, op: &str, origin: usize) {
        if self.0.elapsed().subsec_millis() > 100 {
            print!("[ \x1b[1;32m{}\x1b[0m ] origin: ", op);
            print_3digit(origin);
            println!("\x1b[1F");
            self.0 = Instant::now();
        }
    }
}

impl Drop for Printer {
    fn drop(&mut self) {
        print!("\x1b[K");
    }
}

fn print_3digit(n: usize) {
    if n == 0 {
        return;
    }
    let next = n / 1000;
    print_3digit(next);
    if next == 0 {
        print!("{} ", n % 1000);
    } else {
        print!("{:03} ", n % 1000);
    }
}

use std::time::Instant;

pub struct Printer(Instant);

impl Printer {
    pub fn new() -> Printer {
        Printer(Instant::now())
    }
    pub fn print(&mut self, origin: usize) {
        if self.0.elapsed().subsec_millis() > 100 {
            print!("origin: ");
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

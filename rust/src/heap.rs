#[derive(Clone, Debug, std::cmp::PartialEq)]
pub struct Heap {
    list: Vec<usize>,
}

impl Heap {
    pub fn new() -> Heap {
        Heap { list: Vec::new() }
    }
    pub fn push(&mut self, v: usize) {
        self.list.push(v);
    }
    // Return the minimal evalued by the closure f.
    pub fn min<Min>(&mut self, f: Min) -> Option<usize>
    where
        Min: Fn(usize) -> Option<usize>,
    {
        let mut min_eval: Option<usize> = None;
        let mut min_value: Option<usize> = None;
        let mut min_index: Option<usize> = None;

        self.list
            .iter()
            .enumerate()
            .for_each(|(j, &v)| match (f(v), min_eval) {
                (Some(e), Some(me)) if e < me => {
                    min_eval = Some(e);
                    min_value = Some(v);
                    min_index = Some(j)
                }
                (Some(e), None) => {
                    min_eval = Some(e);
                    min_value = Some(v);
                    min_index = Some(j)
                }
                _ => {}
            });

        if let Some(i) = min_index {
            let last = self.list.pop();
            if i < self.list.len() && last.is_some() {
                self.list[i] = last.unwrap();
            }
        }

        min_value
    }
}

#[test]
fn heap_min() {
    let mut h = Heap::new();
    let f = |u| Some(5 - u);

    assert_eq!(h.min(f), None);

    h.push(1);
    assert_eq!(h.min(f), Some(1));

    h.push(1);
    h.push(2);
    h.push(3);
    h.push(3);

    assert_eq!(h.min(f), Some(3));

    let mut hh = Heap::new();
    hh.push(1);
    hh.push(2);
    hh.push(3);
    assert_eq!(h, hh);
}

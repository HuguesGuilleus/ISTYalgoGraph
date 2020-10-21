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
    /// min_minimum est la valeur minimal que l'on peut atteindre.
    pub fn next<Min>(&mut self, min_theoretical: usize, f: Min) -> Option<usize>
    where
        Min: Fn(usize) -> Option<usize>,
    {
        let mut min_eval: Option<usize> = None;
        let mut min_value: Option<usize> = None;
        let mut min_index: Option<usize> = None;

        for i in (0..self.list.len()).rev() {
            let v = self.list[i];
            match (f(v), min_eval) {
                (Some(current), _) if current == min_theoretical => {
                    self.list.swap_remove(i);
                    // self.list.pop();
                    return Some(v);
                }
                (Some(current), Some(min)) if current < min => {
                    min_eval = Some(current);
                    min_value = Some(v);
                    min_index = Some(i);
                }
                (Some(current), None) => {
                    min_eval = Some(current);
                    min_value = Some(v);
                    min_index = Some(i);
                }
                _ => {}
            }
        }

        if let Some(i) = min_index {
            self.list.swap_remove(i);
        }

        min_value
    }
}

#[test]
fn heap_next() {
    let mut h = Heap::new();
    let f = |u| Some(5 - u);

    assert_eq!(h.next(0, f), None);

    h.push(1);
    assert_eq!(h.next(0, f), Some(1));

    h.push(1);
    h.push(2);
    h.push(3);
    h.push(3);

    assert_eq!(h.next(2, f), Some(3));

    let mut hh = Heap::new();
    hh.push(1);
    hh.push(2);
    hh.push(3);
    assert_eq!(h, hh);
}

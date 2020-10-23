pub struct Stack2 {
    current: Vec<usize>,
    future: Vec<usize>,
}

impl Stack2 {
    pub fn new() -> Stack2 {
        Stack2 {
            current: vec![],
            future: vec![],
        }
    }
    pub fn push_back(&mut self, v: usize) {
        self.future.push(v);
    }
    pub fn pop_front(&mut self) -> Option<usize> {
        if self.current.len() == 0 {
            std::mem::swap(&mut self.future, &mut self.current);
        }
        self.current.pop()
    }
}

#[test]
fn stack() {
    let mut s = Stack2::new();

    s.push_back(1);
    s.push_back(2);
    s.push_back(3);

    assert_eq!(s.pop_front(), Some(3));
    assert_eq!(s.pop_front(), Some(2));
    assert_eq!(s.pop_front(), Some(1));

    s.push_back(4);
    s.push_back(5);
    s.push_back(6);

    assert_eq!(s.pop_front(), Some(6));
    assert_eq!(s.pop_front(), Some(5));
    assert_eq!(s.pop_front(), Some(4));

    assert_eq!(s.pop_front(), None);
}

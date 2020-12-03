#[derive(Debug)]
pub struct Stack {
    present: Vec<(usize, usize)>,
    future: Vec<(usize, usize)>,
    reeval: Vec<usize>,
    reeval_mode: bool,
}

#[derive(Debug, PartialEq)]
pub enum StackNext {
    None,
    Some((usize, usize)),
    Reeval(usize),
}

impl Stack {
    pub fn with_capacity(len: usize) -> Self {
        Self {
            present: Vec::with_capacity(len),
            future: Vec::with_capacity(len),
            reeval: Vec::with_capacity(len),
            reeval_mode: false,
        }
    }
    pub fn pop(&mut self) -> StackNext {
        if self.reeval_mode {
            match self.reeval.pop() {
                Some(n) => StackNext::Reeval(n),
                None => {
                    self.reeval_mode = false;
                    self.pop()
                }
            }
        } else {
            match self.present.pop() {
                Some(n) => StackNext::Some(n),
                None if self.reeval.len() + self.future.len() > 0 => {
                    self.reeval_mode = true;
                    std::mem::swap(&mut self.present, &mut self.future);
                    self.pop()
                }
                None => StackNext::None,
            }
        }
    }
    pub fn push_future(&mut self, n: (usize, usize)) {
        self.future.push(n);
    }
    pub fn push_reeval(&mut self, n: usize) {
        self.reeval.push(n);
    }
}

#[test]
fn queue() {
    // let mut q = Stack::with_capacity(10);
    // q.push_future(1);
    // assert_eq!(StackNext::Some(1), q.pop());
    // assert_eq!(StackNext::None, q.pop());
    // assert_eq!(false, q.reeval_mode);
	//
    // q.push_future(1);
    // q.push_future(2);
    // q.push_reeval(4);
    // q.push_reeval(5);
    // q.push_future(3);
    // assert_eq!(vec![1, 2, 3], q.future);
    // assert_eq!(vec![4, 5], q.reeval);
    // assert_eq!(false, q.reeval_mode);
	//
    // assert_eq!(StackNext::Reeval(5), q.pop());
    // assert_eq!(StackNext::Reeval(4), q.pop());
	//
    // assert_eq!(StackNext::Some(3), q.pop());
    // assert_eq!(StackNext::Some(2), q.pop());
    // assert_eq!(StackNext::Some(1), q.pop());
    // q.push_future(6);
    // q.push_future(7);
    // assert_eq!(StackNext::Some(7), q.pop());
    // assert_eq!(StackNext::Some(6), q.pop());
    // assert_eq!(StackNext::None, q.pop());
    // assert_eq!(StackNext::None, q.pop());
}

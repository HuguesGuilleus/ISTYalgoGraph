use std::cmp::max;

/// Représente la longueur de la plus grande branche et la longueur du chemin le plus profond.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Weight {
    pub deep: usize,
}

impl Weight {
    pub const NULL: Weight = Weight { deep: 0 };
    #[cfg(test)]
    pub fn new(deep: usize, _branch: usize) -> Self {
        Self { deep: deep }
    }
    /// Returne `true` si la branche existe.
    pub fn nobranch(&self) -> bool {
        self.deep == 0
    }
}

impl std::ops::AddAssign for Weight {
    fn add_assign(&mut self, other: Self) {
        self.deep = max(self.deep, other.deep);
    }
}
#[test]
fn weight_add_assign() {
    let mut w = Weight { deep: 5 };

    w.deep += 1;
    assert_eq!(Weight { deep: 6 }, w);

    w += Weight { deep: 3 };
    assert_eq!(Weight { deep: 6 }, w);
}

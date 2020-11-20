use std::cmp::max;

/// Représente la longueur de la plus grande branche et la longueur du chemin le plus profond.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Weight {
    pub deep: usize,
    pub branch: usize,
}

impl Weight {
    pub const NULL: Weight = Weight { deep: 0, branch: 0 };
    #[cfg(test)]
    pub fn new(deep: usize, branch: usize) -> Self {
        Self {
            deep: deep,
            branch: branch,
        }
    }
    pub fn max(&self) -> usize {
        max(self.deep, self.branch)
    }
    /// Quand on avance le long d'une branche.
    pub fn walk(&mut self) {
        self.deep += 1;
    }
    /// Returne `true` si la branche existe.
    pub fn nobranch(&self) -> bool {
        self.deep == 0
    }
}

impl std::ops::AddAssign for Weight {
    fn add_assign(&mut self, other: Self) {
        self.branch = max(max(self.branch, other.branch), self.deep + other.deep);
        self.deep = max(self.deep, other.deep);
    }
}
#[test]
fn weight_add_assign() {
    let mut w = Weight {
        deep: 5,
        branch: 14,
    };

    w.walk();
    assert_eq!(
        Weight {
            deep: 6,
            branch: 14,
        },
        w
    );

    w += Weight {
        deep: 3,
        branch: 42,
    };
    assert_eq!(
        Weight {
            deep: 6,
            branch: 42,
        },
        w
    );
}

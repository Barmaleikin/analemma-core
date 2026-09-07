use std::collections::HashMap;

/// Type placeholder for uninitialized sparse cells: the ternary zero.
pub trait TernaryDefault { fn ternary_default() -> Self; }

impl TernaryDefault for crate::trit::Trit { fn ternary_default() -> Self { crate::trit::Trit::Z } }
impl TernaryDefault for crate::tryte::Tryte { fn ternary_default() -> Self { crate::tryte::Tryte::zero() } }
impl TernaryDefault for crate::word::Word { fn ternary_default() -> Self { crate::word::Word::zero() } }

/// Sparse one-dimensional ternary array with (possibly negative) index bounds.
#[derive(Clone, Debug)]
pub struct TernaryArray<T: Clone + Default + TernaryDefault> {
    data: HashMap<i32, T>,
    min_idx: i32,
    max_idx: i32,
}

impl<T: Clone + Default + TernaryDefault> TernaryArray<T> {
    pub fn new(min_idx: i32, max_idx: i32) -> Self {
        TernaryArray { data: HashMap::new(), min_idx, max_idx }
    }
    pub fn get(&self, idx: i32) -> T {
        assert!(idx >= self.min_idx && idx <= self.max_idx);
        self.data.get(&idx).cloned().unwrap_or_else(T::ternary_default)
    }
    pub fn get_mut(&mut self, idx: i32) -> &mut T {
        assert!(idx >= self.min_idx && idx <= self.max_idx);
        self.data.entry(idx).or_insert_with(T::default)
    }
    pub fn set(&mut self, idx: i32, value: T) {
        assert!(idx >= self.min_idx && idx <= self.max_idx);
        self.data.insert(idx, value);
    }
    pub fn len(&self) -> usize { (self.max_idx - self.min_idx + 1) as usize }
}

/// Sparse two-dimensional ternary array.
#[derive(Clone, Debug)]
pub struct TernaryArray2D<T: Clone + Default + TernaryDefault> {
    data: HashMap<(i32, i32), T>,
    min_i: i32, max_i: i32, min_j: i32, max_j: i32,
}

impl<T: Clone + Default + TernaryDefault> TernaryArray2D<T> {
    pub fn new(min_i: i32, max_i: i32, min_j: i32, max_j: i32) -> Self {
        TernaryArray2D { data: HashMap::new(), min_i, max_i, min_j, max_j }
    }
    pub fn get(&self, i: i32, j: i32) -> T {
        assert!(i >= self.min_i && i <= self.max_i && j >= self.min_j && j <= self.max_j);
        self.data.get(&(i, j)).cloned().unwrap_or_else(T::ternary_default)
    }
    pub fn get_mut(&mut self, i: i32, j: i32) -> &mut T {
        assert!(i >= self.min_i && i <= self.max_i && j >= self.min_j && j <= self.max_j);
        self.data.entry((i, j)).or_insert_with(T::default)
    }
    pub fn set(&mut self, i: i32, j: i32, value: T) {
        assert!(i >= self.min_i && i <= self.max_i && j >= self.min_j && j <= self.max_j);
        self.data.insert((i, j), value);
    }
}

/// Sparse three-dimensional ternary array.
#[derive(Clone, Debug)]
pub struct TernaryArray3D<T: Clone + Default + TernaryDefault> {
    data: HashMap<(i32, i32, i32), T>,
    min_i: i32, max_i: i32, min_j: i32, max_j: i32, min_k: i32, max_k: i32,
}

impl<T: Clone + Default + TernaryDefault> TernaryArray3D<T> {
    pub fn new(min_i: i32, max_i: i32, min_j: i32, max_j: i32, min_k: i32, max_k: i32) -> Self {
        TernaryArray3D { data: HashMap::new(), min_i, max_i, min_j, max_j, min_k, max_k }
    }
    pub fn get(&self, i: i32, j: i32, k: i32) -> T {
        assert!(i >= self.min_i && i <= self.max_i && j >= self.min_j && j <= self.max_j && k >= self.min_k && k <= self.max_k);
        self.data.get(&(i, j, k)).cloned().unwrap_or_else(T::ternary_default)
    }
    pub fn get_mut(&mut self, i: i32, j: i32, k: i32) -> &mut T {
        assert!(i >= self.min_i && i <= self.max_i && j >= self.min_j && j <= self.max_j && k >= self.min_k && k <= self.max_k);
        self.data.entry((i, j, k)).or_insert_with(T::default)
    }
    pub fn set(&mut self, i: i32, j: i32, k: i32, value: T) {
        assert!(i >= self.min_i && i <= self.max_i && j >= self.min_j && j <= self.max_j && k >= self.min_k && k <= self.max_k);
        self.data.insert((i, j, k), value);
    }
}

/// Sparse four-dimensional ternary array.
#[derive(Clone, Debug)]
pub struct TernaryArray4D<T: Clone + Default + TernaryDefault> {
    data: HashMap<(i32, i32, i32, i32), T>,
    min_a: i32, max_a: i32, min_b: i32, max_b: i32,
    min_c: i32, max_c: i32, min_d: i32, max_d: i32,
}

impl<T: Clone + Default + TernaryDefault> TernaryArray4D<T> {
    pub fn new(min_a: i32, max_a: i32, min_b: i32, max_b: i32, min_c: i32, max_c: i32, min_d: i32, max_d: i32) -> Self {
        TernaryArray4D { data: HashMap::new(), min_a, max_a, min_b, max_b, min_c, max_c, min_d, max_d }
    }
    pub fn get(&self, a: i32, b: i32, c: i32, d: i32) -> T {
        assert!(a >= self.min_a && a <= self.max_a && b >= self.min_b && b <= self.max_b && c >= self.min_c && c <= self.max_c && d >= self.min_d && d <= self.max_d);
        self.data.get(&(a, b, c, d)).cloned().unwrap_or_else(T::ternary_default)
    }
    pub fn get_mut(&mut self, a: i32, b: i32, c: i32, d: i32) -> &mut T {
        assert!(a >= self.min_a && a <= self.max_a && b >= self.min_b && b <= self.max_b && c >= self.min_c && c <= self.max_c && d >= self.min_d && d <= self.max_d);
        self.data.entry((a, b, c, d)).or_insert_with(T::default)
    }
    pub fn set(&mut self, a: i32, b: i32, c: i32, d: i32, value: T) {
        assert!(a >= self.min_a && a <= self.max_a && b >= self.min_b && b <= self.max_b && c >= self.min_c && c <= self.max_c && d >= self.min_d && d <= self.max_d);
        self.data.insert((a, b, c, d), value);
    }
}

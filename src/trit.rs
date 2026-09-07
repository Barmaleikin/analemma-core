/// Trit — the base unit of ternary information.
///
/// Balanced ternary: −1 (N), 0 (Z), +1 (P).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum Trit {
    #[default]
    N = -1,
    Z = 0,
    P = 1,
}

impl Trit {
    pub fn from_i8(v: i8) -> Option<Self> {
        match v { -1 => Some(Trit::N), 0 => Some(Trit::Z), 1 => Some(Trit::P), _ => None }
    }

    pub fn to_i8(self) -> i8 { self as i8 }

    /// Sign inversion.
    pub fn neg(self) -> Self {
        match self { Trit::N => Trit::P, Trit::Z => Trit::Z, Trit::P => Trit::N }
    }

    /// Tritwise multiplication.
    pub fn mul(self, other: Trit) -> Self {
        Trit::from_i8(self.to_i8() * other.to_i8()).unwrap()
    }

    /// Full ternary addition with carry.
    ///
    /// Returns `(sum, carry)`; the algebraic identity holds:
    /// `self + other + carry == sum + 3 * carry`.
    pub fn add(self, other: Trit, carry: Trit) -> (Trit, Trit) {
        match self.to_i8() + other.to_i8() + carry.to_i8() {
            -3 => (Trit::Z, Trit::N), -2 => (Trit::P, Trit::N),
            -1 => (Trit::N, Trit::Z), 0 => (Trit::Z, Trit::Z),
            1 => (Trit::P, Trit::Z), 2 => (Trit::N, Trit::P),
            3 => (Trit::Z, Trit::P), _ => unreachable!(),
        }
    }
}

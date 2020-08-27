use std::ops::Add;
use std::num::Wrapping;

/// Wrapper for a lattice scalar element. The underlying modulus is fixed to be
/// 2^16.
#[derive(Clone, Debug, PartialEq, Copy)]
pub struct LatticeScalar(pub Wrapping<u16>);

impl LatticeScalar {
    pub fn get_u16(&self) -> Wrapping<u16> {
        let LatticeScalar(x) = self;
        *x
    }

    pub fn round(&self) -> Wrapping<u16> {
        let LatticeScalar(x) = self;
        *x >> 8
    }

    pub fn sample() -> Self {
        LatticeScalar(Wrapping(rand::random::<u16>()))
    }
}

impl Add for LatticeScalar {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let LatticeScalar(x) = self;
        let LatticeScalar(y) = other;

        LatticeScalar(x + y)
    }
}

/// Wrapper for a lattice vector element. For profiling purposes, the dimension
/// of the vector is fixed to be 512.
pub struct LatticeVector(pub [LatticeScalar; 512]);

impl LatticeVector {
    pub fn inner_product(&self, v: &Self) -> LatticeScalar {
        let LatticeVector(u) = self;
        let LatticeVector(v) = v;

        let w = u.iter()
            .zip(v.iter())
            .map(|(x, y)| x.get_u16() * y.get_u16())
            .sum();

        LatticeScalar(w)
    }

    pub fn sample() -> Self {
        let mut v = [LatticeScalar(Wrapping(0)); 512];
        for i in 0..512 {
            v[i] = LatticeScalar::sample();
        }
        LatticeVector(v)
    }
}



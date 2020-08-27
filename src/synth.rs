use sha2::{Sha512, Digest};

use super::*;
use lattice::*;

use std::num::Wrapping;

/// Left, right, and target elements for the Kim synthesizer.
pub struct KimLeft(pub Vec<LatticeVector>);
pub struct KimRight(pub LatticeVector);
pub struct KimTarget(pub LatticeScalar);

impl Domain for KimLeft {
    fn sample() -> Self {
        KimLeft((0..16).map(|_| LatticeVector::sample()).collect())
    }
}

impl Domain for KimRight {
    fn sample() -> Self {
        KimRight(LatticeVector::sample())
    }
}

impl Domain for KimTarget {
    fn sample() -> Self {
        KimTarget(LatticeScalar::sample())
    }
}

/// Instantiation of the synthesizer trait for the Kim synthesizer scheme.
pub struct KimSynthesizer;

impl Synthesizer for KimSynthesizer {
    type LeftDomain = KimLeft;
    type RightDomain = KimRight;
    type TargetDomain = KimTarget;

    fn eval(left: Self::LeftDomain, right: Self::RightDomain) -> Result<Self::TargetDomain> {
        let KimLeft(left_vecs) = left;
        let KimRight(right_vec) = right;

        Ok(KimTarget(left_chain(left_vecs.into(), right_vec)))
    }
}


pub fn left_chain(left_vecs: Vec<LatticeVector>, right_vec: LatticeVector) -> LatticeScalar {
    let mut rand = LatticeScalar(Wrapping(0));

    for left_vec in left_vecs {
        let b = (left_vec.inner_product(&right_vec) + rand).round();

        let mut hasher = Sha512::new();
        let Wrapping(high_bits) = b >> 8;
        let Wrapping(low_bits) = b;

        hasher.input([high_bits as u8, low_bits as u8]);

        let digest = hasher.result();

        rand = LatticeScalar(Wrapping(digest.as_slice()[0] as u16));
    }
    rand
}



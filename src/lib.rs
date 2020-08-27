/// Kim Pseudorandom Synthesizer
///
/// This is a research prototype and should never be used for important data
///
///
/// This library contains some prototype implementations of Kim pseudorandom synthesizer scheme.
/// Pseudorandom synthesizers can be used as a building block for a variety of cryptographic
/// primitives including pseudorandom functions (blockciphers) and pseudorandom generators. The
/// security of the Kim synthesizer is based on hard problems on lattices.
///
/// For the accompanying research paper, see: https://eprint.iacr.org/2020/233.pdf.
///
/// Basic traits that model pseuorandom synthesizers are specified in lib.rs. Simple lattice
/// structs and operations can be found in lattice.rs and the instantiation of the synthesizer
/// traits for the kim synthesizer can be found in synth.rs.

#[macro_use]
extern crate error_chain;

pub mod errors {
    error_chain! {
        foreign_links {
            Io(::std::io::Error);
        }
    }
}

use errors::*;

#[macro_use]
pub mod lattice;
pub mod synth;
pub mod profile;

/// Trait for a synthesizer domain
pub trait Domain: Sized {
    fn sample() -> Self;
}

/// Trait for pseudorandom synthesizers
pub trait Synthesizer {
    // Types for the left, right, and target domain for a synthesizer
    type LeftDomain: Domain;
    type RightDomain: Domain;
    type TargetDomain: Domain;

    /* Synthesizer evaluation function */
    fn eval(left: Self::LeftDomain, right: Self::RightDomain) -> Result<Self::TargetDomain>;
}


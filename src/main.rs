extern crate prf;

use prf::lattice;
use prf::profile;

fn main() {
    prf::profile::profile_run_all(100).expect("Profile failed");
}

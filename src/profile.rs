extern crate rand;
extern crate time;

use super::*;
use synth::*;

pub fn profile_run_all(iterations: usize) -> Result<()> {
    profile_synthesizer(iterations)?;

    Ok(())
}

pub fn profile_synthesizer(iterations: usize) -> Result<()> {
    let mut cum_time = 0;

    for _ in 0..iterations {
        let avs = KimLeft::sample();
        let sv = KimRight::sample();

        let start = time::precise_time_ns();
        KimSynthesizer::eval(avs, sv)?;
        let end = time::precise_time_ns();

        let time = end - start;

        cum_time += time;
    }

    let avg_time = cum_time / (iterations as u64);

    println!("Kim Synthesizer Evaluation");
    println!("Iterations: {}", iterations);
    println!("Average evaluation time: {} ns", avg_time);

    Ok(())
}


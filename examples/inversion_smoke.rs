//! Ontological inversion demo: pull a "living-ish" particle toward its structured antipode.
//! Toy 2D — axis = x means +x (living side) reflects toward -x (other side), not erase.

use nalgebra::DVector;
use physicslang::{
    antipode, invert_toward_antipode, physics_step_ex, PhysicsLangKnobs, PhysicsParticle,
    StepContext,
};

fn main() {
    let concept = DVector::from_vec(vec![1.0, 0.0]); // concept axis
    let living = DVector::from_vec(vec![1.0, 0.15]); // "magma hamster" side of the coin
    let other = antipode(&living, &concept, None);
    println!("living  = ({:.3}, {:.3})", living[0], living[1]);
    println!("antipode= ({:.3}, {:.3})", other[0], other[1]);

    let sweet = invert_toward_antipode(&living, &concept, 0.25, None);
    println!("gain0.25= ({:.3}, {:.3})", sweet[0], sweet[1]);

    let mut knobs = PhysicsLangKnobs::default();
    knobs.inversion_gain = 0.25;
    knobs.goal_strength = 6.0;
    knobs.kuramoto_coupling = 0.0;
    knobs.charge_boost = 0.0;
    knobs.spring_k_token = 0.0;

    let mut p = vec![PhysicsParticle::new(0, living.clone(), 1.0, 0.0)];
    let ctx = StepContext {
        inversion_axis: Some(&concept),
        ..Default::default()
    };
    for step in 0..40 {
        physics_step_ex(&mut p, &knobs, &ctx);
        if step % 10 == 0 {
            println!(
                "step {step:02}: pos=({:.3}, {:.3})",
                p[0].pos[0], p[0].pos[1]
            );
        }
    }
    println!("final   = ({:.3}, {:.3})", p[0].pos[0], p[0].pos[1]);
    println!("ok");
}

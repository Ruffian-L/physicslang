use nalgebra::DVector;
use physicslang::{physics_step, PhysicsLangKnobs, PhysicsParticle};

fn main() {
    let knobs = PhysicsLangKnobs::default();
    let mut particles = vec![
        PhysicsParticle::new(0, DVector::from_vec(vec![0.0, 0.0]), 1.2, -1.0), // noun-ish
        PhysicsParticle::new(1, DVector::from_vec(vec![0.5, 0.1]), 1.0, 1.0),  // verb-ish
        PhysicsParticle::new(2, DVector::from_vec(vec![1.0, 0.0]), 0.8, -1.0),
    ];
    for (i, p) in particles.iter_mut().enumerate() {
        p.token_idx = i;
        p.doc_id = 1;
        p.sentence_id = 0;
    }
    let goal = DVector::from_vec(vec![0.5, 0.5]);
    for step in 0..20 {
        physics_step(&mut particles, &knobs, Some(&goal));
        if step % 5 == 0 {
            println!(
                "step {step}: p0=({:.3},{:.3}) p1=({:.3},{:.3})",
                particles[0].pos[0], particles[0].pos[1],
                particles[1].pos[0], particles[1].pos[1]
            );
        }
    }
    println!("ok");
}

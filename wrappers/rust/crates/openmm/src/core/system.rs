#[allow(unused_imports)]
use crate::preface::*;
use openmm_bindings as openmm;
use std::os::raw::c_int;

/// This type represents a molecular system. The definition of a `System` involves
/// four elements:
///
/// - The set of particles in the system
/// - The forces acting on them
/// - Pairs of particles whose separation should be constrained to a fixed value
/// - For periodic systems, the dimensions of the periodic box
///
/// The particles and constraints are defined directly by the `System` type, while
/// forces are defined by types that implement the [`Force`] trait.  After creating a
/// `System`, call [`add_particle()`] once for each particle, addConstraint() for each constraint,
/// and [`add_force()`] for each Force.
///
/// In addition, particles may be designated as virtual sites.  These are particles
/// whose positions are computed automatically based on the positions of other particles.
/// To define a virtual site, call setVirtualSite(), passing in a VirtualSite object
/// that defines the rules for computing its position.
///
/// [`add_particle()`]: Self::add_particle()
/// [`add_force()`]: Self::add_force()
pub struct System {
    cxx_system: openmm::System,
}

impl System {
    /// Create a new, empty system
    pub fn new() -> Self {
        let cxx_system = unsafe { openmm::System::new() };

        Self { cxx_system }
    }

    /// Add a particle with the given mass (in AMU) to the system and return its index
    pub fn add_particle(&mut self, mass: f64) -> usize {
        unsafe { self.cxx_system.addParticle(mass) as usize }
    }

    /// Get the mass (in AMU) of the particle at the given index
    pub fn particle_mass(&self, index: usize) -> f64 {
        if index >= self.num_particles() {
            panic!(
                "Particle index out of bounds: num_particles is {} but the index is {}",
                self.num_particles(),
                index
            )
        }
        unsafe { self.cxx_system.getParticleMass(index as c_int) }
    }

    /// Set the mass (in AMU) of the particle at the given index
    pub fn set_particle_mass(&mut self, index: usize, mass: f64) {
        if index >= self.num_particles() {
            panic!(
                "Particle index out of bounds: num_particles is {} but the index is {}",
                self.num_particles(),
                index
            )
        }
        unsafe { self.cxx_system.setParticleMass(index as c_int, mass) }
    }

    /// Get the number of particles in the System
    pub fn num_particles(&self) -> usize {
        unsafe { self.cxx_system.getNumParticles() as usize }
    }

    /// Add a force to the system
    pub fn add_force(&mut self, force: Box<dyn Force>) -> usize {
        let force_ptr = Box::into_raw(force);
        unsafe { self.cxx_system.addForce(force_ptr as *mut openmm::Force) as usize }
    }
}

impl Default for System {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for System {
    fn drop(&mut self) {
        unsafe { self.cxx_system.destruct() };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_destroy_system() {
        let system = System::new();
        drop(system)
    }

    #[test]
    fn set_get_system_particle_mass() {
        let mut system = System::new();

        let index = system.add_particle(14.0);
        assert_eq!(system.particle_mass(index), 14.0);

        system.set_particle_mass(index, 1.008);
        assert_eq!(system.particle_mass(index), 1.008);

        assert_eq!(system.num_particles(), 1);
    }

    fn test_force() {
        let mut system = System::new();
        let force = Box::new(crate::force::NonbondedForce::new());

        system.add_force(force);

        drop(system);
    }
}

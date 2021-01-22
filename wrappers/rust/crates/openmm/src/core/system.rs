#[allow(unused_imports)]
use crate::preface::*;
use openmm_bindings::c_bindings as openmm;
use std::os::raw::c_int;
use std::ptr::NonNull;
use std::marker::PhantomData;
use std::convert::TryFrom;

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
    ffi_system: NonNull<openmm::OpenMM_System>,
    _system_marker: PhantomData<openmm::OpenMM_System>,
}

impl System {
    /// Create a new, empty system
    pub fn new() -> Self {
        // SAFETY: OpenMM_System_create() returns a pointer to a new C System object
        let ptr = unsafe { openmm::OpenMM_System_create() };
        let ffi_system = NonNull::new(ptr).expect("OpenMM_System_create returned null pointer");

        Self { ffi_system, _system_marker: PhantomData }
    }

    /// Get a unique reference to the underlying system
    fn as_mut(&mut self) -> &mut openmm::OpenMM_System {
        // SAFETY: self.ffi_system is a unique non-null pointer to an initialized OpenMM_System,
        // and we are mutably borrowing self
        unsafe { self.ffi_system.as_mut() }
    }

    /// Get a shared reference to the underlying system
    fn as_ref(&self) -> &openmm::OpenMM_System {
        // SAFETY: self.ffi_system is a unique non-null pointer to an initialized OpenMM_System,
        // and we are immutably borrowing self
        unsafe { self.ffi_system.as_ref() } 
    }

    /// Add a particle with the given `mass` (in AMU) to the `System` and return its index.
    /// 
    /// If the mass is 0, integrators will ignore the particle and not modify its
    /// position or velocity. This may be used for virtual sites or to prevent a particle
    /// from moving.
    pub fn add_particle(&mut self, mass: f64) -> i32 {
        let idx = unsafe { openmm::OpenMM_System_addParticle(self.as_mut(), mass) };
        i32::try_from(idx).expect("Index is not a valid i32")
    }

    /// Get the mass (in AMU) of the particle at the given index
    ///
    /// If the mass is 0, Integrators will ignore the particle and not modify its position 
    /// or velocity. This is most often used for virtual sites, but can also be used as a 
    /// way to prevent a particle from moving.
    pub fn particle_mass(&self, index: i32) -> f64 {
        if index >= self.num_particles() {
            panic!(
                "Particle index out of bounds: num_particles is {} but the index is {}",
                self.num_particles(),
                index
            )
        }
        let index = c_int::try_from(index).expect("Index is not a valid c_int");
        // SAFETY: Index is in bounds (and this is double-checked in C++), and a simple double 
        // is returned
        unsafe { openmm::OpenMM_System_getParticleMass(self.as_ref(), index) }
    }

    /// Set the mass (in AMU) of the particle at the given index
    /// 
    /// If the mass is 0, Integrators will ignore the particle and not modify its position or 
    /// velocity. This is most often used for virtual sites, but can also be used as a way to 
    /// prevent a particle from moving.
    pub fn set_particle_mass(&mut self, index: i32, mass: f64) {
        if index >= self.num_particles() {
            panic!(
                "Particle index out of bounds: num_particles is {} but the index is {}",
                self.num_particles(),
                index
            )
        }
        let index = c_int::try_from(index).expect("Index is not a valid c_int");
        // SAFETY: Index is in bounds (and this is double-checked in C++), and a simple double 
        // is returned
        unsafe { openmm::OpenMM_System_setParticleMass(self.as_mut(), index, mass) }
    }

    /// Get the number of particles in the System
    pub fn num_particles(&self) -> i32 {
        // SAFETY: OpenMM_System_getNumParticles() does not mutate the target
        let n = unsafe { openmm::OpenMM_System_getNumParticles(self.as_ref()) };
        i32::try_from(n).expect("Number of particles is not a valid i32")
    }

    /// Add a force to the system
    pub fn add_force(&mut self, force: impl Force) -> i32 {
        // SAFETY: self.ffi_system takes ownership of force, which should be heap-allocated,
        // and drops it when ffi_system is dropped (in C++). The System is mutated, so 
        // a mutable pointer is essential.
        // Note: forces are freed in C++ with the delete operator, so must be allocated
        // in C++ too, with the new operator
        unsafe { 
            let force_ptr = force.into_ptr() as *mut openmm::OpenMM_Force;
            openmm::OpenMM_System_addForce(self.as_mut(), force_ptr) 
        }
    }
}

impl Default for System {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for System {
    fn drop(&mut self) {
        unsafe { openmm::OpenMM_System_destroy(self.ffi_system.as_ptr()) };
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

    #[test]
    fn test_force() {
        let mut system = System::new();
        let force = crate::force::NonbondedForce::new();

        system.add_force(force);

        drop(system);
    }
}

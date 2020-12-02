#[allow(unused_imports)]
use crate::preface::*;
use openmm_bindings as openmm;
use std::os::raw::c_int;

mod private {
    /// Sealed trait for forces. May be removed in the future if we get to the point that new forces
    /// can be defined entirely in Rust
    pub trait Sealed {}

    impl Sealed for super::nonbonded::NonbondedForce {}
}

/// Force types apply forces to the particles in a [`System`], or alter their behavior in other ways
///
/// More specifically, a `Force` object can do any or all of the following:
/// - Add a contribution to the force on each particle
/// - Add a contribution to the potential energy of the System
/// - Modify the positions and velocities of particles at the start of each time step
/// - Define parameters which are stored in the Context and can be modified by the user
/// - Change the values of parameters defined by other Force objects at the start of each time step
///
/// Forces may be organized into force groups. This is used for multiple time step integration,
/// and allows subsets of the Forces in a `System` to be evaluated at different times. By default,
/// all Forces are in group 0. Call [`Force::set_group()`] to change this. Some `Force` types may
/// provide additional methods to further split their computations into multiple groups. Be aware
/// that particular Platforms may place restrictions on the use of force groups, such as requiring
/// all nonbonded forces to be in the same group.
pub trait Force: private::Sealed {
    /// Get the force group this `Force` belongs to
    fn group(&self) -> usize {
        let self_ptr: *const Self = self;
        unsafe { openmm::Force_getForceGroup(self_ptr as *const openmm::Force) as usize }
    }
    /// Set the force group this `Force` belongs to
    ///
    /// Valid groups are in the range 0..=31.
    fn set_group(&mut self, group: usize) {
        if group > 31 {
            panic!("Force groups must be in the range 0..=31, not {}", group);
        }

        let self_ptr: *mut Self = self;
        unsafe { openmm::Force_setForceGroup(self_ptr as *mut openmm::Force, group as c_int) };
    }
    /// Does this `Force` use Periodic Boundary Conditions
    fn uses_pbc(&mut self) -> bool;

    // fn context_impl(&mut self, context: &mut Context) -> &mut ContextImpl {
    //     let self_ptr: *mut Self = self;
    //     unsafe { openmm::Force_getContextImpl(self_ptr as *mut openmm::Force, context) }
    // }
    // fn force_impl_in_context(&self, context: &Context) -> &ForceImpl {
    //     let self_ptr: *const Self = self;
    //     unsafe { openmm::Force_getImplInContext1(self_ptr as *const openmm::Force, context) }
    // }
    // fn force_impl_in_context_mut(&mut self, context: &mut Context) -> &mut ForceImpl {
    //     let self_ptr: *mut Self = self;
    //     unsafe { openmm::Force_getImplInContext(self_ptr as *mut openmm::Force, context) }
    // }
}

pub mod nonbonded;
pub use nonbonded::NonbondedForce;

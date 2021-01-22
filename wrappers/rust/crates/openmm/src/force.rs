#[allow(unused_imports)]
use crate::preface::*;
use openmm_bindings::c_bindings as openmm;
use std::os::raw::c_int;

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
pub trait Force {
    /// The underlying Force type
    type CxxForce;

    fn as_ref(&self) -> &Self::CxxForce;

    fn as_mut(&mut self) -> &mut Self::CxxForce;

    /// Convert the Force into a pointer to the underlying C++ Force class
    /// 
    /// # Safety:
    /// 
    /// The returned pointer should be allocated on the stack with the C++
    /// new operator, and should "own" the allocated memory - ie, it should
    /// not be aliased.
    unsafe fn into_ptr(self) -> *mut Self::CxxForce where Self: Sized {
        let mut self_mandrop = std::mem::ManuallyDrop::new(self);
        self_mandrop.as_mut() as *mut Self::CxxForce
    }

    /// Get the force group this `Force` belongs to
    fn group(&self) -> u8 {
        let ptr = self.as_ref() as *const Self::CxxForce;
        unsafe { openmm::OpenMM_Force_getForceGroup(ptr as *const openmm::OpenMM_Force) as u8 }
    }
    /// Set the force group this `Force` belongs to
    ///
    /// Valid groups are in the range 0..=31.
    fn set_group(&mut self, group: u8) {
        if group > 31 {
            panic!("Force groups must be in the range 0..=31, not {}", group);
        }

        let ptr = self.as_mut() as *mut Self::CxxForce;
        unsafe { openmm::OpenMM_Force_setForceGroup(ptr as *mut openmm::OpenMM_Force, group as c_int) };
    }
    /// Does this `Force` use Periodic Boundary Conditions
    fn uses_pbc(&self) -> bool {
        let ptr = self.as_ref() as *const Self::CxxForce;
        unsafe { openmm::OpenMM_Force_usesPeriodicBoundaryConditions(ptr as *const openmm::OpenMM_Force) != 0 }
    }

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

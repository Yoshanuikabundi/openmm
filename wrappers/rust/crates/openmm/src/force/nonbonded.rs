#[allow(unused_imports)]
use crate::preface::*;
use openmm_bindings::c_bindings as openmm;
use std::ptr::NonNull;

pub enum NonbondedMethod {
    /// A non-periodic cutoff scheme with a reaction field
    ///
    /// Interactions beyond the cutoff distance are ignored. Coulomb interactions closer than the
    /// cutoff distance are modified using the reaction field method.
    CutoffNonPeriodic,
    /// A periodic cutoff scheme with a reaction field following the minimum image convention
    ///
    /// Periodic boundary conditions are used, so that each particle interacts only with the nearest
    /// periodic copy of each other particle. Interactions beyond the cutoff distance are ignored.
    /// Coulomb interactions closer than the cutoff distance are modified using the reaction field
    /// method.
    CutoffPeriodic,
    /// A periodic scheme with Ewald summation used for all Coulomb interactions
    ///
    /// Periodic boundary conditions are used, and Ewald summation is used to compute the Coulomb
    /// interaction of each particle with all periodic copies of every other particle.
    Ewald,
    /// A periodic scheme using Particle Mesh Ewald for both LJ and Coulomb interactions
    ///
    /// Periodic boundary conditions are used, and Particle-Mesh Ewald (PME) summation is used to
    /// compute the interaction of each particle with all periodic copies of every other particle
    /// for both Coulomb and Lennard-Jones. No switching is used for either interaction.
    LjPme,
    /// No cutoff scheme and no periodic boundary conditions
    ///
    /// No cutoff is applied to nonbonded interactions. The full set of N^2 interactions is computed
    /// exactly. This necessarily means that periodic boundary conditions cannot be used. This is
    /// the default.
    NoCutoff,
    /// A periodic scheme using Particle Mesh Ewald for Coulomb interactions
    ///
    /// Periodic boundary conditions are used, and Particle-Mesh Ewald (PME) summation is used to
    /// compute the Coulomb interaction of each particle with all periodic copies of every other
    /// particle.
    Pme,
}

impl Default for NonbondedMethod {
    fn default() -> Self {
        Self::NoCutoff
    }
}

impl Into<openmm::OpenMM_NonbondedForce_NonbondedMethod::Type> for NonbondedMethod {
    fn into(self) -> openmm::OpenMM_NonbondedForce_NonbondedMethod::Type {
        match self {
            Self::CutoffNonPeriodic => openmm::OpenMM_NonbondedForce_NonbondedMethod::OpenMM_NonbondedForce_CutoffNonPeriodic,
            Self::CutoffPeriodic => openmm::OpenMM_NonbondedForce_NonbondedMethod::OpenMM_NonbondedForce_CutoffPeriodic,
            Self::Ewald => openmm::OpenMM_NonbondedForce_NonbondedMethod::OpenMM_NonbondedForce_Ewald,
            Self::LjPme => openmm::OpenMM_NonbondedForce_NonbondedMethod::OpenMM_NonbondedForce_LJPME,
            Self::NoCutoff => openmm::OpenMM_NonbondedForce_NonbondedMethod::OpenMM_NonbondedForce_NoCutoff,
            Self::Pme => openmm::OpenMM_NonbondedForce_NonbondedMethod::OpenMM_NonbondedForce_PME,
        }
    }
}

impl From<openmm::OpenMM_NonbondedForce_NonbondedMethod::Type> for NonbondedMethod {
    fn from(method: openmm::OpenMM_NonbondedForce_NonbondedMethod::Type) -> Self {
        match method {
            openmm::OpenMM_NonbondedForce_NonbondedMethod::OpenMM_NonbondedForce_CutoffNonPeriodic => Self::CutoffNonPeriodic,
            openmm::OpenMM_NonbondedForce_NonbondedMethod::OpenMM_NonbondedForce_CutoffPeriodic => Self::CutoffPeriodic,
            openmm::OpenMM_NonbondedForce_NonbondedMethod::OpenMM_NonbondedForce_Ewald => Self::Ewald,
            openmm::OpenMM_NonbondedForce_NonbondedMethod::OpenMM_NonbondedForce_LJPME => Self::LjPme,
            openmm::OpenMM_NonbondedForce_NonbondedMethod::OpenMM_NonbondedForce_NoCutoff => Self::NoCutoff,
            openmm::OpenMM_NonbondedForce_NonbondedMethod::OpenMM_NonbondedForce_PME => Self::Pme,
            i => panic!("{} is not a valid nonbonded method", i),
        }
    }
}

/// Nonbonded LJ and Coulomb forces with the Lorentz-Berthelot combining rule
///
/// This type implements nonbonded interactions between particles, including a Coulomb force to
/// represent electrostatics and a Lennard-Jones force to represent van der Waals interactions.
/// It optionally supports periodic boundary conditions and cutoffs for long range interactions.
/// Lennard-Jones interactions are calculated with the Lorentz-Berthelot combining rule: it uses
/// the arithmetic mean of the sigmas and the geometric mean of the epsilons for the two interacting
/// particles.
///
/// To use this type, create a NonbondedForce object, then call [`add_particle()`] once for each
/// particle in the [`System`] to define its parameters. The number of particles for which you
/// define nonbonded parameters must be exactly equal to the number of particles in the `System`, or
/// else an exception will be thrown when you try to create a Context. After a particle has been
/// added, you can modify its force field parameters by calling setParticleParameters(). This will
/// have no effect on Contexts that already exist unless you call updateParametersInContext().
///
/// NonbondedForce also lets you specify "exceptions", particular pairs of particles whose
/// interactions should be computed based on different parameters than those defined for the
/// individual particles. This can be used to completely exclude certain interactions from the force
/// calculation, or to alter how they interact with each other.
///
/// Many molecular force fields omit Coulomb and Lennard-Jones interactions between particles
/// separated by one or two bonds, while using modified parameters for those separated by three
/// bonds (known as "1-4 interactions"). This type provides a convenience method for this case
/// called createExceptionsFromBonds(). You pass to it a list of bonds and the scale factors to
/// use for 1-4 interactions. It identifies all pairs of particles which are separated by 1, 2,
/// or 3 bonds, then automatically creates exceptions for them.
///
/// When using a cutoff, by default Lennard-Jones interactions are sharply truncated at the cutoff
/// distance. Optionally you can instead use a switching function to make the interaction smoothly
/// go to zero over a finite distance range. To enable this, call setUseSwitchingFunction(). You
/// must also call setSwitchingDistance() to specify the distance at which the interaction should
/// begin to decrease. The switching distance must be less than the cutoff distance.
///
/// Another optional feature of this class (enabled by default) is to add a contribution to the
/// energy which approximates the effect of all Lennard-Jones interactions beyond the cutoff in a
/// periodic system. When running a simulation at constant pressure, this can improve the quality
/// of the result. Call setUseDispersionCorrection() to set whether this should be used.
///
/// In some applications, it is useful to be able to inexpensively change the parameters of small
/// groups of particles. Usually this is done to interpolate between two sets of parameters. For
/// example, a titratable group might have two states it can exist in, each described by a different
/// set of parameters for the atoms that make up the group. You might then want to smoothly
/// interpolate between the two states. This is done by first calling addGlobalParameter() to define
/// a Context parameter, then addParticleParameterOffset() to create a "parameter offset" that
/// depends on the Context parameter. Each offset defines the following:
///
///  - A Context parameter used to interpolate between the states.
///  - A single particle whose parameters are influenced by the Context parameter.
///  - Three scale factors (chargeScale, sigmaScale, and epsilonScale) that specify how the Context parameter affects the particle.
///
/// The "effective" parameters for a particle (those used to compute forces) are given by
/// ```text
/// charge = baseCharge + paramchargeScale
/// sigma = baseSigma + paramsigmaScale
/// epsilon = baseEpsilon + param*epsilonScale
/// ```
/// where the "base" values are the ones specified by addParticle() and "oaram" is the current value
/// of the Context parameter. A single Context parameter can apply offsets to multiple particles,
/// and multiple parameters can be used to apply offsets to the same particle. Parameters can also
/// be used to modify exceptions in exactly the same way by calling addExceptionParameterOffset().
///
/// [`add_particle()`]: Self::add_particle()
pub struct NonbondedForce {
    nbforce_ptr: NonNull<openmm::OpenMM_NonbondedForce>,
}

impl NonbondedForce {
    /// Create a new, unparametrised NonbondedForce
    pub fn new() -> Self {
        let ptr = unsafe { openmm::OpenMM_NonbondedForce_create() };
        let nbforce_ptr = NonNull::new(ptr).expect("OpenMM_NonbondedForce returned null pointer");

        Self { nbforce_ptr }
    }

    fn as_ptr(& self) -> *const openmm::OpenMM_NonbondedForce {
        self.nbforce_ptr.as_ptr()
    }

    fn as_mut_ptr(&mut self) -> *mut openmm::OpenMM_NonbondedForce {
        self.nbforce_ptr.as_ptr()
    }

    /// Add a particle with the given parameters
    ///
    /// This should be called once for each particle in the [`System`]. When it is called for the `i`th
    /// time, it specifies the parameters for the `i`th particle. For calculating the Lennard-Jones
    /// interaction between two particles, the arithmetic mean of the sigmas and the geometric mean
    /// of the epsilons for the two interacting particles is used (the Lorentz-Berthelot combining
    /// rule).
    pub fn add_particle(&mut self, charge: f64, sigma: f64, epsilon: f64) -> i32 {
        unsafe { openmm::OpenMM_NonbondedForce_addParticle(self.as_mut_ptr(), charge, sigma, epsilon) as i32 }
    }

    /// Get the method used to compute the nonbonded forces
    pub fn method(&self) -> NonbondedMethod {
        unsafe { openmm::OpenMM_NonbondedForce_getNonbondedMethod(self.as_ptr()).into() }
    }

    /// Set the method used to compute the nonbonded forces
    pub fn set_method(&mut self, method: NonbondedMethod) {
        unsafe { openmm::OpenMM_NonbondedForce_setNonbondedMethod(self.as_mut_ptr(), method.into()) }
    }
}

impl Default for NonbondedForce {
    fn default() -> Self {
        Self::new()
    }
}

impl Force for NonbondedForce {
    type CxxForce = openmm::OpenMM_NonbondedForce;

    fn as_ref(&self) -> &Self::CxxForce {
        // SAFETY: self.nbforce_ptr is a non-null pointer to initialised, properly sized memory,
        // and we are immutably borrowing it
        unsafe { self.nbforce_ptr.as_ref() }
    }

    fn as_mut(&mut self) -> &mut Self::CxxForce {
        // SAFETY: self.nbforce_ptr is a non-null pointer to initialised, properly sized memory,
        // and we are mutably borrowing it
        unsafe { self.nbforce_ptr.as_mut() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_force() {
        let _force = NonbondedForce::new();
    }

    #[test]
    fn test_force_group() {
        let mut force = NonbondedForce::new();

        assert_eq!(force.group(), 0);

        force.set_group(16);

        assert_eq!(force.group(), 16);
    }
}

/// Parameters for a time-step of the physics engine.
#[derive(Clone)]
#[cfg_attr(feature = "serde-serialize", derive(Serialize, Deserialize))]
pub struct IntegrationParameters {
    /// The timestep length (default: `1.0 / 60.0`)
    pub dt: f32,

    //    /// If `true` and if rapier is compiled with the `parallel` feature, this will enable rayon-based multithreading (default: `true`).
    //    ///
    //    /// This parameter is ignored if rapier is not compiled with is `parallel` feature.
    //    /// Refer to rayon's documentation regarding how to configure the number of threads with either
    //    /// `rayon::ThreadPoolBuilder::new().num_threads(4).build_global().unwrap()` or `ThreadPool::install`.
    //    /// Note that using only one thread with `multithreading_enabled` set to `true` will result on a slower
    //    /// simulation than setting `multithreading_enabled` to `false`.
    //    pub multithreading_enabled: bool,
    /// If `true`, the world's `step` method will stop right after resolving exactly one CCD event (default: `false`).
    /// This allows the user to take action during a timestep, in-between two CCD events.
    pub return_after_ccd_substep: bool,
    /// The Error Reduction Parameter in `[0, 1]` is the proportion of
    /// the positional error to be corrected at each time step (default: `0.2`).
    pub erp: f32,
    /// The Error Reduction Parameter for joints in `[0, 1]` is the proportion of
    /// the positional error to be corrected at each time step (default: `0.2`).
    pub joint_erp: f32,
    /// Each cached impulse are multiplied by this coefficient in `[0, 1]`
    /// when they are re-used to initialize the solver (default `1.0`).
    pub warmstart_coeff: f32,
    /// Contacts at points where the involved bodies have a relative
    /// velocity smaller than this threshold wont be affected by the restitution force (default: `1.0`).
    pub restitution_velocity_threshold: f32,
    /// Amount of penetration the engine wont attempt to correct (default: `0.005m`).
    pub allowed_linear_error: f32,
    /// The maximal distance separating two objects that will generate predictive contacts (default: `0.002`).
    pub prediction_distance: f32,
    /// Amount of angular drift of joint limits the engine wont
    /// attempt to correct (default: `0.001rad`).
    pub allowed_angular_error: f32,
    /// Maximum linear correction during one step of the non-linear position solver (default: `0.2`).
    pub max_linear_correction: f32,
    /// Maximum angular correction during one step of the non-linear position solver (default: `0.2`).
    pub max_angular_correction: f32,
    /// Maximum nonlinear SOR-prox scaling parameter when the constraint
    /// correction direction is close to the kernel of the involved multibody's
    /// jacobian (default: `0.2`).
    pub max_stabilization_multiplier: f32,
    /// Maximum number of iterations performed by the velocity constraints solver (default: `4`).
    pub max_velocity_iterations: usize,
    /// Maximum number of iterations performed by the position-based constraints solver (default: `1`).
    pub max_position_iterations: usize,
    /// Minimum number of dynamic bodies in each active island (default: `128`).
    pub min_island_size: usize,
    /// Maximum number of iterations performed by the position-based constraints solver for CCD steps (default: `10`).
    ///
    /// This should be sufficiently high so all penetration get resolved. For example, if CCD cause your
    /// objects to stutter, that may be because the number of CCD position iterations is too low, causing
    /// them to remain stuck in a penetration configuration for a few frames.
    ///
    /// The highest this number, the highest its computational cost.
    pub max_ccd_position_iterations: usize,
    /// Maximum number of substeps performed by the  solver (default: `1`).
    pub max_ccd_substeps: usize,
    /// Controls the number of Proximity::Intersecting events generated by a trigger during CCD resolution (default: `false`).
    ///
    /// If false, triggers will only generate one Proximity::Intersecting event during a step, even
    /// if another colliders repeatedly enters and leaves the triggers during multiple CCD substeps.
    ///
    /// If true, triggers will generate as many Proximity::Intersecting and Proximity::Disjoint/Proximity::WithinMargin
    /// events as the number of times a collider repeatedly enters and leaves the triggers during multiple CCD substeps.
    /// This is more computationally intensive.
    pub multiple_ccd_substep_sensor_events_enabled: bool,
    /// Whether penetration are taken into account in CCD resolution (default: `false`).
    ///
    /// If this is set to `false` two penetrating colliders will not be considered to have any time of impact
    /// while they are penetrating. This may end up allowing some tunelling, but will avoid stuttering effect
    /// when the constraints solver fails to completely separate two colliders after a CCD contact.
    ///
    /// If this is set to `true`, two penetrating colliders will be considered to have a time of impact
    /// equal to 0 until the constraints solver manages to separate them. This will prevent tunnelling
    /// almost completely, but may introduce stuttering effects when the constraints solver fails to completely
    /// separate two colliders after a CCD contact.
    // FIXME: this is a very binary way of handling penetration.
    // We should provide a more flexible solution by letting the user choose some
    // minimal amount of movement applied to an object that get stuck.
    pub ccd_on_penetration_enabled: bool,
}

impl IntegrationParameters {
    /// Creates a set of integration parameters with the given values.
    pub fn new(
        dt: f32,
        //        multithreading_enabled: bool,
        erp: f32,
        joint_erp: f32,
        warmstart_coeff: f32,
        restitution_velocity_threshold: f32,
        allowed_linear_error: f32,
        allowed_angular_error: f32,
        max_linear_correction: f32,
        max_angular_correction: f32,
        prediction_distance: f32,
        max_stabilization_multiplier: f32,
        max_velocity_iterations: usize,
        max_position_iterations: usize,
        max_ccd_position_iterations: usize,
        max_ccd_substeps: usize,
        return_after_ccd_substep: bool,
        multiple_ccd_substep_sensor_events_enabled: bool,
        ccd_on_penetration_enabled: bool,
    ) -> Self {
        IntegrationParameters {
            dt,
            //            multithreading_enabled,
            erp,
            joint_erp,
            warmstart_coeff,
            restitution_velocity_threshold,
            allowed_linear_error,
            allowed_angular_error,
            max_linear_correction,
            max_angular_correction,
            prediction_distance,
            max_stabilization_multiplier,
            max_velocity_iterations,
            max_position_iterations,
            // FIXME: what is the optimal value for min_island_size?
            // It should not be too big so that we don't end up with
            // huge islands that don't fit in cache.
            // However we don't want it to be too small and end up with
            // tons of islands, reducing SIMD parallelism opportunities.
            min_island_size: 128,
            max_ccd_position_iterations,
            max_ccd_substeps,
            return_after_ccd_substep,
            multiple_ccd_substep_sensor_events_enabled,
            ccd_on_penetration_enabled,
        }
    }

    /// The current time-stepping length.
    #[inline(always)]
    pub fn dt(&self) -> f32 {
        self.dt
    }

    /// The inverse of the time-stepping length, i.e. the steps per seconds (Hz).
    ///
    /// This is zero if `self.dt` is zero.
    #[inline(always)]
    pub fn inv_dt(&self) -> f32 {
        if self.dt == 0.0 {
            0.0
        } else {
            1.0 / self.dt
        }
    }

    /// Sets the time-stepping length.
    #[inline]
    pub fn set_dt(&mut self, dt: f32) {
        assert!(dt >= 0.0, "The time-stepping length cannot be negative.");
        self.dt = dt;
    }

    /// Sets the inverse time-stepping length (i.e. the frequency).
    ///
    /// This automatically recompute `self.dt`.
    #[inline]
    pub fn set_inv_dt(&mut self, inv_dt: f32) {
        if inv_dt == 0.0 {
            self.dt = 0.0
        } else {
            self.dt = 1.0 / inv_dt
        }
    }
}

impl Default for IntegrationParameters {
    fn default() -> Self {
        Self {
            dt: 1.0 / 60.0,
            //        multithreading_enabled:             true,
            return_after_ccd_substep: false,
            erp: 0.2,
            joint_erp: 0.2,
            warmstart_coeff: 1.0,
            restitution_velocity_threshold: 1.0,
            allowed_linear_error: 0.005,
            prediction_distance: 0.002,
            allowed_angular_error: 0.001,
            max_linear_correction: 0.2,
            max_angular_correction: 0.2,
            max_stabilization_multiplier: 0.2,
            max_velocity_iterations: 4,
            max_position_iterations: 1,
            // FIXME: what is the optimal value for min_island_size?
            // It should not be too big so that we don't end up with
            // huge islands that don't fit in cache.
            // However we don't want it to be too small and end up with
            // tons of islands, reducing SIMD parallelism opportunities.
            min_island_size: 128,
            max_ccd_position_iterations: 10,
            max_ccd_substeps: 1,
            multiple_ccd_substep_sensor_events_enabled: false,
            ccd_on_penetration_enabled: false,
        }
    }
}

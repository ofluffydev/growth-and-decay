use std::ops::Neg;

/// Represents the parameters and results of an exponential growth or decay process.
///
/// This struct is used to calculate the final value, rate, or time for exponential growth or decay.
/// It supports modifying the final value or time and recalculating the other parameters accordingly.
#[derive(Clone)]
pub struct ExponentialChange {
    /// The initial value (principal) at the start of the process.
    pub principal: f64,
    /// The final value after the specified time.
    pub final_value: f64,
    /// The growth or decay rate (as a fraction, e.g., 0.025 for 2.5%).
    pub rate: f64,
    /// The time over which the growth or decay occurs.
    pub time: f64,
}

/// Represents the parameters and results of an exponential decay process.
///
/// This struct is used to calculate the decay constant, time, and ratios for processes such as radioactive decay.
pub struct GrowthOrDecayRatios {
    /// The final ratio (Rt) after the specified time.
    pub rt: f64,
    /// The initial ratio (R0) at the start of the process.
    pub r0: f64,
    /// The decay constant, calculated based on the half-life or decay years.
    pub decay_constant: f64,
    /// The time elapsed during the decay process.
    pub time: f64,
    /// The half-life or characteristic decay time of the process.
    pub decay_years: f64,
}

impl ExponentialChange {
    /// Creates a new instance of `ExponentialChange`.
    ///
    /// # Parameters
    /// - `principal`: The initial value at the start of the process.
    /// - `final_value`: The final value after the specified time. Can be `None` if the rate is provided.
    /// - `rate`: The growth or decay rate. Can be `None` if the final value is provided.
    /// - `time`: The time over which the growth or decay occurs.
    ///
    /// # Panics
    /// Panics if both `final_value` and `rate` are not provided, as at least one must be specified.
    ///
    /// # Returns
    /// A new instance of `ExponentialChange` with calculated values.
    pub fn new(
        principal: f64,
        final_value: impl Into<Option<f64>>,
        rate: impl Into<Option<f64>>,
        time: f64,
    ) -> Self {
        let final_value = final_value.into();
        let rate = rate.into();

        // If neither final_value nor rate is provided, panic.
        assert!(
            !(final_value.is_none() && rate.is_none()),
            "Either final_value or rate must be provided."
        );

        // If the final value is provided, calculate the rate.
        let rate = rate.unwrap_or_else(|| {
            if final_value < Some(principal) {
                // Rearranged to solve for a negative rate
                (-(final_value.unwrap() / principal).ln() / time).neg()
            } else {
                (final_value.unwrap() / principal).powf(1.0 / time) - 1.0
            }
        });

        // If the rate is provided, calculate the final value.
        let final_value = final_value.unwrap_or_else(|| principal * (1.0 + rate).powf(time));

        Self {
            principal,
            final_value,
            rate,
            time,
        }
    }

    /// Modifies the final value of the instance and recalculates the time required.
    ///
    /// # Parameters
    /// - `new_final_value`: The new final value to set.
    ///
    /// # Behavior
    /// Updates the `final_value` field and recalculates the `time` field based on the current rate.
    pub fn modify_final_value(&mut self, new_final_value: f64) {
        // Update the final value
        self.final_value = new_final_value;

        // Recalculate the time using the correct formula
        self.time = ((self.final_value / self.principal).ln() / self.rate).abs(); // Time cannot be negative
    }

    /// Modifies the time of the instance and recalculates the final value.
    ///
    /// # Parameters
    /// - `new_time`: The new time to set.
    ///
    /// # Behavior
    /// Updates the `time` field and recalculates the `final_value` field based on the current rate.
    pub fn modify_final_time(&mut self, new_time: f64) {
        // Update the time and recalculate the final value.
        self.time = new_time;

        // Recalculate the final value using the appropriate formula
        self.final_value = if self.rate < 0.0 {
            self.principal * (self.rate * self.time).exp()
        } else {
            self.principal * (1.0 + self.rate).powf(self.time)
        };
    }
}

impl GrowthOrDecayRatios {
    /// Creates a new instance of `GrowthOrDecayRatios`.
    ///
    /// # Parameters
    /// - `rt`: The final ratio after the specified time. Can be `None` if the time is provided.
    /// - `r0`: The initial ratio at the start of the process.
    /// - `decay_years`: The half-life or characteristic decay time of the process.
    /// - `time`: The time elapsed during the decay process. Can be `None` if the final ratio is provided.
    ///
    /// # Panics
    /// Panics if both `rt` and `time` are not provided, as at least one must be specified.
    ///
    /// # Returns
    /// A new instance of `GrowthOrDecayRatios` with calculated values.
    pub fn new(
        rt: impl Into<Option<f64>>,
        r0: f64,
        decay_years: f64,
        time: impl Into<Option<f64>>,
    ) -> Self {
        let rt = rt.into();
        let time = time.into();

        // Assert that either nt or time is provided.
        assert!(
            rt.is_some() || time.is_some(),
            "Either nt or time must be provided."
        );

        // Use the provided time or calculate it from the ratio
        let time = time.map_or_else(
            || {
                let ratio = rt.unwrap() / r0;
                -(ratio.ln()) * decay_years
            },
            |time_value| time_value,
        );

        // Calculate the final ratio using the formula R = R0 * e^(-t / decay_years)
        let nt = rt.unwrap_or_else(|| r0 * (-time / decay_years).exp());

        // Calculate the decay constant for informational purposes
        let decay_constant = (2.0_f64).ln() / decay_years;

        Self {
            rt: nt,
            r0,
            decay_constant,
            time,
            decay_years,
        }
    }
}

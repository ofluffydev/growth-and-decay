use std::ops::Neg;

/// Holds the input values for the exponential growth calculation.
#[derive(Clone)]
pub struct ExponentialChange {
    pub principal: f64,
    pub final_value: f64,
    pub rate: f64,
    pub time: f64,
}

pub struct GrowthOrDecayRatios {
    pub rt: f64,
    pub r0: f64,
    pub decay_constant: f64,
    pub time: f64,
    pub decay_years: f64,
}

impl ExponentialChange {
    /// Creates a new instance of `ExponentialGrowthInput`.
    pub fn new(principal: f64, final_value: Option<f64>, rate: Option<f64>, time: f64) -> Self {
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

    /// Modifies the final value of the instance, updates the time accordingly.
    pub fn modify_final_value(&mut self, new_final_value: f64) {
        // Update the final value
        self.final_value = new_final_value;

        // Recalculate the time using the correct formula
        self.time = ((self.final_value / self.principal).ln() / self.rate).abs(); // Time cannot be negative
    }

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
    /// Creates a new instance of `DecayInput`.
    ///
    /// Computes time if not provided, assuming standard exponential decay.
    /// Panics if neither nt nor time is provided.
    pub fn new(rt: Option<f64>, r0: f64, decay_years: f64, time: Option<f64>) -> Self {
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

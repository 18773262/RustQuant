// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::curves::{Curve, CurveModel};
use crate::time::{today, DayCountConvention};
use time::Date;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// STRUCTS, ENUMS, AND TRAITS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Nelson-Siegel (1987) model parameters.
pub struct NelsonSiegel {
    beta0: f64,
    beta1: f64,
    beta2: f64,
    lambda: f64,
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPLEMENTATIONS, TRAITS, AND FUNCTIONS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl NelsonSiegel {
    /// Create a new Nelson-Siegel model.
    #[must_use]
    pub const fn new(beta0: f64, beta1: f64, beta2: f64, lambda: f64) -> Self {
        Self {
            beta0,
            beta1,
            beta2,
            lambda,
        }
    }
}

impl CurveModel for NelsonSiegel {
    /// Returns the forward rate for a given date.
    fn forward_rate(&self, date: Date) -> f64 {
        assert!(date > today(), "Date must be in the future.");

        let tau = DayCountConvention::default().day_count_factor(today(), date);

        let term1 = f64::exp(-tau / self.lambda);
        let term2 = (tau / self.lambda) * term1;

        self.beta0 + self.beta1 * term1 + self.beta2 * term2
    }

    /// Returns the spot rate for a given date.
    fn spot_rate(&self, date: Date) -> f64 {
        assert!(date > today(), "Date must be in the future.");

        let tau = DayCountConvention::default().day_count_factor(today(), date);

        let term1 = self.lambda * (1. - f64::exp(-tau / self.lambda)) / tau;
        let term2 = term1 - f64::exp(-tau / self.lambda);

        self.beta0 + self.beta1 * term1 + self.beta2 * term2
    }

    fn discount_factor(&self, date: Date) -> f64 {
        let tau = DayCountConvention::default().day_count_factor(today(), date);

        f64::exp(-self.spot_rate(date) * tau / 100.)
    }

    fn calibrate<C: Curve>(&self, _curve: C) -> Self {
        unimplemented!()
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// UNIT TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod tests_nelson_siegel {
    use super::*;
    // use crate::plot_vector;
    use time::Duration;

    #[test]
    fn test_nelson_siegel() {
        let ns = NelsonSiegel {
            beta0: 0.0806,
            beta1: -0.0031,
            beta2: -0.0625,
            lambda: 1.58,
        };

        let dates = (2..365 * 30)
            .map(|i| today() + Duration::days(i))
            .collect::<Vec<Date>>();

        let _forward_curve = dates
            .iter()
            .map(|date| ns.forward_rate(*date))
            .collect::<Vec<_>>();

        let _discount_curve = dates
            .iter()
            .map(|date| ns.discount_factor(*date))
            .collect::<Vec<_>>();

        // plot_vector!(forward_curve, "./images/nelson_siegel_forward.png");
        // plot_vector!(discount_curve, "./images/nelson_siegel_discount.png");
    }
}

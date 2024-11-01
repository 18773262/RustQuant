// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPORTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use super::{
    Asay82, Black76, BlackScholes73, GarmanKohlhagen83, GeneralisedBlackScholesMerton, Merton73,
    TypeFlag,
};
use crate::instruments::Payoff;
use crate::pricing::AnalyticOptionPricer;
use crate::time::{today, year_fraction};
use derive_builder::Builder;
use time::Date;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// STRUCTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// European vanilla option.
#[derive(Debug, Clone, Builder)]
pub struct EuropeanVanillaOption {
    /// The strike price of the option.
    pub strike: f64,

    /// The expiry date of the option.
    pub expiry: Date,

    /// The type of the option (call or put).
    pub type_flag: TypeFlag,
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPLEMENTATIONS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

macro_rules! european_vanilla_option_gbsm {
    ($gbsm_variant:ident) => {
        impl AnalyticOptionPricer<EuropeanVanillaOption, $gbsm_variant> {
            /// Print a report of the option price and greeks.
            pub fn report(&self) {
                use std::collections::HashMap;

                let map = HashMap::from([
                    ("price", self.price()),
                    ("delta", self.delta()),
                    ("gamma", self.gamma()),
                    ("theta", self.theta()),
                    ("vega", self.vega()),
                    ("rho", self.rho()),
                    ("vanna", self.vanna()),
                    ("charm", self.charm()),
                    ("lambda", self.lambda()),
                    ("zomma", self.zomma()),
                    ("speed", self.speed()),
                    ("color", self.color()),
                    ("vomma", self.vomma()),
                    ("ultima", self.ultima()),
                ]);

                println!("Model: {:?}", self.model);
                println!("Option: {:?}", self.option);
                println!("{:#?}", map);
                println!();
            }

            /// Calculate the price of the option.
            pub fn price(&self) -> f64 {
                let k = self.option.strike;
                let t = year_fraction(today(), self.option.expiry);
                let f = self.option.type_flag;

                self.model.price(k, t, f)
            }

            /// Calculate the delta of the option.
            pub fn delta(&self) -> f64 {
                let k = self.option.strike;
                let t = year_fraction(today(), self.option.expiry);
                let f = self.option.type_flag;

                self.model.delta(k, t, f)
            }

            /// Calculate the gamma of the option.
            pub fn gamma(&self) -> f64 {
                let k = self.option.strike;
                let t = year_fraction(today(), self.option.expiry);
                let f = self.option.type_flag;

                self.model.gamma(k, t, f)
            }

            /// Calculate the theta of the option.
            pub fn theta(&self) -> f64 {
                let k = self.option.strike;
                let t = year_fraction(today(), self.option.expiry);
                let f = self.option.type_flag;

                self.model.theta(k, t, f)
            }

            /// Calculate the vega of the option.
            pub fn vega(&self) -> f64 {
                let k = self.option.strike;
                let t = year_fraction(today(), self.option.expiry);
                let f = self.option.type_flag;

                self.model.vega(k, t, f)
            }

            /// Calculate the rho of the option.
            pub fn rho(&self) -> f64 {
                let k = self.option.strike;
                let t = year_fraction(today(), self.option.expiry);
                let f = self.option.type_flag;

                self.model.rho(k, t, f)
            }

            /// Calculate the vanna of the option.
            pub fn vanna(&self) -> f64 {
                let k = self.option.strike;
                let t = year_fraction(today(), self.option.expiry);
                let f = self.option.type_flag;

                self.model.vanna(k, t, f)
            }

            /// Calculate the charm of the option.
            pub fn charm(&self) -> f64 {
                let k = self.option.strike;
                let t = year_fraction(today(), self.option.expiry);
                let f = self.option.type_flag;

                self.model.charm(k, t, f)
            }

            /// Calculate the lambda of the option.
            pub fn lambda(&self) -> f64 {
                let k = self.option.strike;
                let t = year_fraction(today(), self.option.expiry);
                let f = self.option.type_flag;

                self.model.lambda(k, t, f)
            }

            /// Calculate the zomma of the option.
            pub fn zomma(&self) -> f64 {
                let k = self.option.strike;
                let t = year_fraction(today(), self.option.expiry);
                let f = self.option.type_flag;

                self.model.zomma(k, t, f)
            }

            /// Calculate the speed of the option.
            pub fn speed(&self) -> f64 {
                let k = self.option.strike;
                let t = year_fraction(today(), self.option.expiry);
                let f = self.option.type_flag;

                self.model.speed(k, t, f)
            }

            /// Calculate the color of the option.
            pub fn color(&self) -> f64 {
                let k = self.option.strike;
                let t = year_fraction(today(), self.option.expiry);
                let f = self.option.type_flag;

                self.model.color(k, t, f)
            }

            /// Calculate the vomma of the option.
            pub fn vomma(&self) -> f64 {
                let k = self.option.strike;
                let t = year_fraction(today(), self.option.expiry);
                let f = self.option.type_flag;

                self.model.vomma(k, t, f)
            }

            /// Calculate the ultima of the option.
            pub fn ultima(&self) -> f64 {
                let k = self.option.strike;
                let t = year_fraction(today(), self.option.expiry);
                let f = self.option.type_flag;

                self.model.ultima(k, t, f)
            }
        }
    };
}

european_vanilla_option_gbsm!(BlackScholes73);
european_vanilla_option_gbsm!(Merton73);
european_vanilla_option_gbsm!(Black76);
european_vanilla_option_gbsm!(Asay82);
european_vanilla_option_gbsm!(GarmanKohlhagen83);

impl Payoff for EuropeanVanillaOption {
    type Underlying = f64;

    fn payoff(&self, underlying: Self::Underlying) -> f64 {
        match self.type_flag {
            TypeFlag::Call => (underlying - self.strike).max(0.0),
            TypeFlag::Put => (self.strike - underlying).max(0.0),
        }
    }
}

impl EuropeanVanillaOption {
    /// Create a new vanilla option.
    pub fn new(strike: f64, expiry: Date, type_flag: TypeFlag) -> Self {
        Self {
            strike,
            expiry,
            type_flag,
        }
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod test_vanilla_option_monte_carlo {
    use super::*;
    use crate::instruments::AsianOption;
    use crate::instruments::AveragingMethod;
    use crate::instruments::StrikeFlag;
    use crate::pricing::monte_carlo_pricer::MonteCarloPricer;
    use crate::stochastics::StochasticProcessConfig;
    use crate::{
        instruments::{ExerciseFlag, OptionContractBuilder},
        models::GeometricBrownianMotion,
    };
    use std::time::Instant;
    use time::macros::date;

    #[test]
    fn test_vanilla_option_monte_carlo() {
        let underlying = 100.0;
        let strike = 100.0;
        let interest_rate = 0.05;
        let time_to_maturity = 1.0;
        let volatility = 0.2;

        let contract = OptionContractBuilder::default()
            .type_flag(TypeFlag::Call)
            .exercise_flag(ExerciseFlag::European {
                expiry: date!(2025 - 01 - 01),
            })
            .build()
            .unwrap();

        let option = VanillaOption::new(contract, strike);
        let process = GeometricBrownianMotion::new(interest_rate, volatility);

        let config =
            StochasticProcessConfig::new(underlying, 0.0, time_to_maturity, 1, 1_000_000, true);

        let start = Instant::now();
        let price = option.price_monte_carlo(&process, &config, interest_rate);
        println!("Elapsed time: {:?}", start.elapsed());

        println!("Price: {}", price);
    }

    #[test]
    fn test_asian_option_monte_carlo() {
        let underlying = 100.0;
        let strike = 100.0;
        let interest_rate = 0.05;
        let time_to_maturity = 1.0;
        let volatility = 0.2;

        let contract = OptionContractBuilder::default()
            .type_flag(TypeFlag::Call)
            .exercise_flag(ExerciseFlag::European {
                expiry: date!(2025 - 01 - 01),
            })
            .strike_flag(Some(StrikeFlag::Fixed))
            .build()
            .unwrap();

        let option = AsianOption::new(contract, AveragingMethod::ArithmeticDiscrete, Some(strike));
        let process = GeometricBrownianMotion::new(interest_rate, volatility);

        let config =
            StochasticProcessConfig::new(underlying, 0.0, time_to_maturity, 1000, 1000, true);

        let start = Instant::now();
        let price = option.price_monte_carlo(&process, &config, interest_rate);
        println!("Elapsed time: {:?}", start.elapsed());

        println!("Price: {}", price);
    }
}

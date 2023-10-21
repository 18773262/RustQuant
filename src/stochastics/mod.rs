// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// MONTE CARLO SIMULATION AND STOCHASTIC PROCESSES
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! Monte Carlo engines to simulate stochastic processes.
//!
//! The following is a list of stochastic processes that can be generated.
//!
//! - Brownian Motions:
//!   - Standard Brownian Motion
//!     - $dX(t) = dW(t)$
//!   - Arithmetic Brownian Motion
//!     - $dX(t) = \mu dt + \sigma dW(t)$
//!   - Geometric Brownian Motion
//!     - $dX(t) = \mu X(t) dt + \sigma X(t) dW(t)$
//!   - Fractional Brownian Motion
//! - Cox-Ingersoll-Ross (1985)
//!   - $dX(t) = \left[ \theta - \alpha X(t) \right] dt + \sigma \sqrt{r_t} dW(t)$
//! - Ornstein-Uhlenbeck process
//!   - $dX(t) = \theta \left[ \mu - X(t) \right] dt + \sigma dW(t)$
//! - Ho-Lee (1986)
//!   - $dX(t) = \theta(t) dt + \sigma dW(t)$
//! - Hull-White (1990)
//!   - $dX(t) = \left[ \theta(t) - \alpha X(t) \right]dt + \sigma dW(t)$
//! - Extended Vasicek (1990)
//!   - $dX(t) = \left[ \theta(t) - \alpha(t) X(t) \right] dt + \sigma dW(t)$
//! - Black-Derman-Toy (1990)
//!   - $d\ln[X(t)] = \left[ \theta(t) + \frac{\sigma'(t)}{\sigma(t)}\ln[X(t)] \right]dt + \sigma_t dW(t)$
//!
//! ```rust
//! use RustQuant::stochastics::*;
//!
//! fn main() {
//!     // Create new GBM with mu and sigma.
//!     let gbm = GeometricBrownianMotion::new(0.05, 0.9);
//!
//!     // Generate path using Euler-Maruyama scheme.
//!     // Parameters: x_0, t_0, t_n, n, sims, parallel.
//!     let output = (&gbm).euler_maruyama(10.0, 0.0, 0.5, 10, 1, false);
//!
//!     println!("GBM = {:?}", output.paths);
//! }
//! ```

pub use arithmetic_brownian_motion::*;
pub use black_derman_toy::*;
pub use brownian_motion::*;
pub use cox_ingersoll_ross::*;
pub use extended_vasicek::*;
pub use fractional_brownian_motion::*;
pub use fractional_cox_ingersoll_ross::*;
pub use fractional_ornstein_uhlenbeck::*;
pub use geometric_brownian_motion::*;
pub use ho_lee::*;
pub use hull_white::*;
pub use ornstein_uhlenbeck::*;
pub use process::*;

/// Arithmetic Brownian Motion.
pub mod arithmetic_brownian_motion;
/// Black-Derman-Toy short rate model.
pub mod black_derman_toy;
/// Standard Brownian Motion.
pub mod brownian_motion;
/// Cox-Ingersoll-Ross process.
pub mod cox_ingersoll_ross;
/// Extended Vasicek process.
pub mod extended_vasicek;
/// Fractional Brownian Motion.
pub mod fractional_brownian_motion;
/// Fractional Cox-Ingersoll-Ross process.
pub mod fractional_cox_ingersoll_ross;
/// Fractional Ornstein-Uhlenbeck process.
pub mod fractional_ornstein_uhlenbeck;
/// Geometric Brownian Motion.
pub mod geometric_brownian_motion;
/// Ho-Lee process.
pub mod ho_lee;
/// Hull-White model process.
pub mod hull_white;
/// Ornstein-Uhlenbeck process.
pub mod ornstein_uhlenbeck;
/// Defines `Trajectories` and `StochasticProcess`.
pub mod process;

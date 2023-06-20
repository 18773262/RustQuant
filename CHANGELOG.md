# Change Log

## 20 June 2023

- Simple linear regression using `nalgebra`.

## 12 June 2023

- Gradient descent optimizer for functions $f: \mathbb{R}^n \rightarrow \mathbb{R}$.

## 5 June 2023

- Additional stochastic process generators
  - Ho-Lee model
  - Hull-White model
  - Black-Derman-Toy model

## 26 May 2023

- Download time series data from [Yahoo! Finance](https://finance.yahoo.com/).

## 25 May 2023

- Read (write) from (to) `.csv`, `.json`, and `.parquet` files, using [Polars `DataFrames`](https://pola-rs.github.io/polars-book/).

## Older

- Arithmetic Brownian Motion generator.
- Gamma, exponential, and chi-squared distributions.
- Forward start option pricer (Rubinstein 1990 formula).
- Gap option and cash-or-nothing option pricers (currently adding more binary options).
- Asian option pricer (closed-form solution for continuous geometric average).
- Heston Model option pricer (uses the tanh-sinh quadrature numerical integrator).
- Tanh-sinh (double exponential) quadrature for evaluating integrals.
  - Plus other basic numerical integrators (midpoint, trapezoid, Simpson's 3/8).
- Characteristic functions and density functions for common distributions:
  - Gaussian, Bernoulli, Binomial, Poisson, Uniform, Chi-Squared, Gamma, and Exponential.
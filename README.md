# Growth and Decay

This crate provides utilities for modeling and calculating exponential growth and decay processes. It includes functionality for solving problems related to population growth, radioactive decay, and other phenomena that follow exponential patterns.

Note this crate specifically updates values as changed when using `modify` functions. Any direct updates to struct fields do not gurantee this.

## Features

- **Exponential Growth and Decay Calculations**: Compute final values, rates, or time for exponential processes.
- **Customizable Inputs**: Specify principal values, rates, time, or final values to solve for missing parameters.
- **Carbon Dating Support**: Perform calculations related to radioactive decay, including half-life and decay constants.

## Usage

### Exponential Growth Example

```rust
use growth_and_decay::ExponentialChange;

fn main() {
    let mut growth = ExponentialChange::new(1_200_000.0, None, Some(0.025), 18.0);
    println!("Initial population: {}", growth.principal);
    println!("Growth rate: {}", growth.rate);
    println!("Time: {} years", growth.time);
    println!("Final population: {}", growth.final_value);

    // Modify the final value and calculate the time required
    growth.modify_final_value(2_000_000.0);
    println!("Time to reach 2 million: {:.2} years", growth.time);
}
```

### Exponential Decay Example

```rust
use growth_and_decay::GrowthOrDecayRatios;

fn main() {
    let decay = GrowthOrDecayRatios::new(None, 1.0 / (10f64).powi(12), 8223.0, Some(8500.0));
    println!("Initial ratio (R0): {:.4e}", decay.r0);
    println!("Decay constant: {:.4e}", decay.decay_constant);
    println!("Time elapsed: {:.4e} years", decay.time);
    println!("Final ratio (Rt): {:.4e}", decay.rt);
}
```

## Installation

Add this crate to your project:

```shell
cargo add growth-and-decay
```

Then, import it into your project:

```rust
use growth_and_decay::{ExponentialChange, GrowthOrDecayRatios};
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

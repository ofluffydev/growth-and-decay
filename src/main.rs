#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]
#![deny(clippy::cargo)]

use growth_and_decay::{ExponentialChange, GrowthOrDecayRatios};

const SEPARATOR: &str = "----------------------------------------";

/// This program demonstrates the use of the `growth-and-decay` crate to calculate
/// various values for exponential growth and decay processes.
///
/// It includes examples for:
/// - Calculating final values based on principal, rate, and time.
/// - Modifying final values or time and recalculating other parameters.
/// - Solving real-world problems such as population growth and radioactive decay.
fn main() {
    println!("Meow\n");
    println!("{SEPARATOR}");

    let mut inputs = [
        ExponentialChange::new(1_200_000.0, None, Some(0.025), 18.0),
        ExponentialChange::new(5000.0, Some(2000.0), None, 3.0),
        ExponentialChange::new(1000.0, Some(100.0), None, 50.0),
    ];

    // Copy the original input 2 for later use
    let input2_original = inputs[1].clone();

    for (index, input) in inputs.iter().enumerate() {
        println!("Item #{}:", index + 1);

        let result = input.final_value;

        // Print the entire equation
        println!(
            "Final value = Principal * (1 + Rate) ^ Time\n\
            Final value = {} * (1 + {}) ^ {}\n\
            Final value = {}",
            input.principal, input.rate, input.time, result
        );

        println!("End value after {} time units: {result:.2}", input.time);

        println!("{SEPARATOR}");
    }

    println!("Problem one, \"When will the population reach 2 million?\"");
    inputs[0].modify_final_value(2_000_000.0);
    let time = inputs[0].time;
    println!("It will reach 2 million when the time is {time:.2} years.");
    assert!(
        (time - 20.433).abs() < 1.0,
        "Problem one for Exponential growth and decay: Time does not match expected value. Expected 20.433, got {time}"
    );

    println!("{SEPARATOR}");

    println!(
        "Problem two, \"If the rate remains the same, how many bacteria will remain after 5 hours?\""
    );
    inputs[1].modify_final_time(5.0);
    let result = inputs[1].final_value;
    let result = (result * 1000.0).ceil() / 1000.0;
    println!(
        "Final value = Principal * (1 + Rate) ^ Time\n\
            Final value = {} * (1 + {}) ^ {}\n\
            Final value = {}",
        inputs[1].principal, inputs[1].rate, inputs[1].time, result
    );

    assert!(
        (result - 1085.768).abs() < f64::EPSILON,
        "Problem two for Exponential growth and decay: Final value does not match expected value. Expected 1085.768, got {result}"
    );

    println!("{SEPARATOR}");

    println!("Problem three, \"In what year will there only be 1 person left?\"");
    inputs[2].modify_final_value(1.0);
    let time = inputs[2].time;
    println!(
        "Final value = Principal * (1 + Rate) ^ Time\n\
            Final value = {} * (1 + {}) ^ {}\n\
            Final value = {}",
        inputs[2].principal, inputs[2].rate, inputs[2].time, time
    );
    assert!(
        (time - 150.169).abs() < 1.0,
        "Problem three for Exponential growth and decay: Time does not match expected value. Expected 150.169, got {time}"
    );

    let decay_input = GrowthOrDecayRatios::new(None, 1.0 / (10f64).powi(12), 8223.0, Some(8500.0));

    println!("II. Carbon Dating:");
    println!(
        "R = {:.4e}e^({:.4e}t)",
        decay_input.rt, decay_input.decay_constant
    );
    println!("R0 = {:.4e}", decay_input.r0);
    println!("t = {:.4e}", decay_input.time);
    println!("Decay constant = {:.4e}", decay_input.decay_constant); // Formatting for precision
    println!("Decay years = {:.4e}", decay_input.decay_years);
    println!("Final value: {:.4e}", decay_input.rt); // More concise phrasing for the final value

    assert!(
        (inputs[0].time - 20.433) < 1.0,
        "Problem one for Exponential growth and decay: Final value does not match expected value. Expected 20.433, got {}",
        inputs[0].time
    );
    assert!(
        (input2_original.rate - -0.305).abs() < 1.0,
        "Problem two for Exponential growth and decay: Final value does not match expected value. Expected -0.305, got {}",
        input2_original.rate
    );
    assert!(
        (decay_input.rt - 3.55693e-13_f64).abs() < f64::EPSILON,
        "Final value does not match expected value."
    );
}

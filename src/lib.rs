pub fn calculate_balance_from_initial_deposit(
    initial_deposit: f64,
    annual_growth_rate: f64,
    compounding_frequency: i32,
    time_period: i32,
) -> f64 {
    let base = 1_f64 + annual_growth_rate / compounding_frequency as f64;
    let power = compounding_frequency * time_period;
    initial_deposit * base.powi(power)
}

pub fn calculate_balance_from_recurring_deposit(
    monthly_recurring_deposit: f64,
    annual_growth_rate: f64,
    compounding_frequency: i32,
    time_period: i32,
) -> f64 {
    let base = 1_f64 + annual_growth_rate / compounding_frequency as f64;
    let power = compounding_frequency * time_period;
    12_f64 / compounding_frequency as f64 * monthly_recurring_deposit * (base.powi(power) - 1_f64)
        / annual_growth_rate
        / compounding_frequency as f64
}

pub fn calculate_total_balance(
    initial_deposit: f64,
    monthly_recurring_deposit: f64,
    annual_growth_rate: f64,
    compounding_frequency: i32,
    time_period: i32,
) -> f64 {
    calculate_balance_from_initial_deposit(
        initial_deposit,
        annual_growth_rate,
        compounding_frequency,
        time_period,
    ) + calculate_balance_from_recurring_deposit(
        monthly_recurring_deposit,
        annual_growth_rate,
        compounding_frequency,
        time_period,
    )
}

fn main() {
    println!("hey tests");
}

fn remaining_fuel(start_fuel: u32, burn_rate: u32, duration: u32) -> u32 {
    start_fuel - (burn_rate * duration)
    // panic!("Panic from function")
}

fn is_low_earth_orbit(altitude_km: f64) -> bool {
    altitude_km >= 160.0 && altitude_km <= 2000.0
}

fn telemetry_conf_msg(pilot_name: &str) -> String {
    format!("Telemetry online, pilot {pilot_name}.")
}

fn validate_altitude(alt_km: f64) -> Result<f64, String> {
    if alt_km < 100.0 {
        Err("Altitude too low. Abort launch".to_string())
    } else {
        Ok(alt_km)
    }
}

//all value to be print must implement partialeq and debug trait if used with assert_eq and asseert_ne
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fuel_level_check() {
        let rem_fuel = remaining_fuel(3000, 10, 50);
        assert_eq!(rem_fuel, 2500);
    }

    #[test]
    #[should_panic(expected = "from function")] //slice here
    fn no_fuel_panic() {
        remaining_fuel(0, 10, 10);
        // remaining_fuel(500, 10, 10);
    }

    #[test]
    fn no_fuel_level_check() {
        let rem_fuel = remaining_fuel(3000, 10, 50);
        assert_ne!(rem_fuel, 0);
    }

    #[test]
    fn critical_temperature() {
        panic!("Critical temperature reached ! System must shut down.");
    }

    #[test]
    fn leo_check() {
        assert!(is_low_earth_orbit(165.0), "Wrong altitude");
        assert!(is_low_earth_orbit(2000.0), "Wrong altitude");
        assert!(!is_low_earth_orbit(159.0), "Wrong altitude");
    }

    #[test]
    fn telemetry_valid_msg() {
        let msg = telemetry_conf_msg("Curtis");
        assert!(msg.contains("Kurt"), "Wrong pilot ({msg})");
    }

    #[test]
    fn low_altitude() {
        let result = validate_altitude(50.0);
        assert!(result.is_err());
        assert!(matches!(result, Err(msg) if msg.contains("too low")));
    }

    #[test]
    fn correct_altitude() {
        let result = validate_altitude(150.0);
        assert_eq!(result, Ok(150.0));
    }
}

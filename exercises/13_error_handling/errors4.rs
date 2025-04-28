#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<Self, CreationError> {
        //short case simple version
        // (value > 0)
        //     .then_some(value as u64)
        //     .ok_or_else(|| {
        //         if value == 0 {
        //             CreationError::Zero
        //         }
        //         else {
        //             CreationError::Negative
        //         }
        //     })
        //     .map(Self)
        
        //short case advanced version
        // (value > 0)
        // .then(|| Ok(Self(value as u64)))
        // .unwrap_or_else(|| {
        //     if value == 0 {
        //         Err(CreationError::Zero)
        //     }
        //     else {
        //         Err(CreationError::Negative)
        //     }
        // })
        
        //more structured version
        match value { 
            v if v < 0 => Err(CreationError::Negative),
            0 => Err(CreationError::Zero),
            v => Ok(Self(v as u64))
        }
        
        // TODO: This function shouldn't always return an `Ok`.
        // Ok(Self(value as u64))
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        assert_eq!(
            PositiveNonzeroInteger::new(10),
            Ok(PositiveNonzeroInteger(10)),
        );
        assert_eq!(
            PositiveNonzeroInteger::new(-10),
            Err(CreationError::Negative),
        );
        assert_eq!(PositiveNonzeroInteger::new(0), Err(CreationError::Zero));
    }
}

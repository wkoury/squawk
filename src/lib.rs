use rand::Rng;

fn generate_squawk() -> u32 {
    let mut rng = rand::thread_rng();
    let ret: u32 = rng.gen_range(0..4095);
    ret
}

pub fn format_squawk_as_string() -> String {
    format_args!("{:04o}", generate_squawk()).to_string()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_squawk_length() {
        let expected_length = 4;
        for _ in 1..9999 {
            let actual_length = format_squawk_as_string().chars().count();
            assert_eq!(expected_length, actual_length);
        }
    }

    #[test]
    fn test_squawk_value_never_above_4095() {
        for _ in 1..9999 {
            assert!(generate_squawk() <= 4095);
        }
    }

}

pub fn collect_bits(number: u64) -> Vec<u8> {
    let mut bits = Vec::new();
    for i in 0..64 {
        let bit = ((number >> i) & 1) as u8;
        bits.push(bit);
    }

    bits
}

pub fn get_last_one_bit(number: u64) -> usize {
    let bits = collect_bits(number);
    let mut index = 0;

    for i in 0..bits.len() {
        if bits[i] == 1 {
            index = i;
        }
    }

    index
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collect_bits_zero() {
        let bits = collect_bits(0);
        assert_eq!(bits.len(), 64);
        
        // All bits should be 0
        for bit in bits {
            assert_eq!(bit, 0);
        }
    }

    #[test]
    fn test_collect_bits_one() {
        let bits = collect_bits(1);
        assert_eq!(bits.len(), 64);
        
        // First bit should be 1, rest should be 0
        assert_eq!(bits[0], 1);
        for i in 1..64 {
            assert_eq!(bits[i], 0);
        }
    }

    #[test]
    fn test_collect_bits_power_of_two() {
        // Test 2^3 = 8
        let bits = collect_bits(8);
        assert_eq!(bits.len(), 64);
        
        // Bit at position 3 should be 1, rest should be 0
        for i in 0..64 {
            if i == 3 {
                assert_eq!(bits[i], 1);
            } else {
                assert_eq!(bits[i], 0);
            }
        }
    }

    #[test]
    fn test_collect_bits_multiple_bits() {
        // Test 5 = 101 in binary (bits 0 and 2 set)
        let bits = collect_bits(5);
        assert_eq!(bits.len(), 64);
        
        assert_eq!(bits[0], 1); // 2^0 = 1
        assert_eq!(bits[1], 0);
        assert_eq!(bits[2], 1); // 2^2 = 4
        
        for i in 3..64 {
            assert_eq!(bits[i], 0);
        }
    }

    #[test]
    fn test_collect_bits_large_number() {
        // Test 2^63 (largest bit)
        let bits = collect_bits(1u64 << 63);
        assert_eq!(bits.len(), 64);
        
        // Only bit 63 should be set
        for i in 0..63 {
            assert_eq!(bits[i], 0);
        }
        assert_eq!(bits[63], 1);
    }

    #[test]
    fn test_get_last_one_bit_zero() {
        // Zero has no one bits, should return 0 (default)
        assert_eq!(get_last_one_bit(0), 0);
    }

    #[test]
    fn test_get_last_one_bit_single_bit() {
        assert_eq!(get_last_one_bit(1), 0);    // 2^0
        assert_eq!(get_last_one_bit(2), 1);    // 2^1
        assert_eq!(get_last_one_bit(4), 2);    // 2^2
        assert_eq!(get_last_one_bit(8), 3);    // 2^3
        assert_eq!(get_last_one_bit(16), 4);   // 2^4
    }

    #[test]
    fn test_get_last_one_bit_multiple_bits() {
        assert_eq!(get_last_one_bit(3), 1);    // 11 binary - last one at position 1
        assert_eq!(get_last_one_bit(5), 2);    // 101 binary - last one at position 2
        assert_eq!(get_last_one_bit(7), 2);    // 111 binary - last one at position 2
        assert_eq!(get_last_one_bit(12), 3);   // 1100 binary - last one at position 3
    }

    #[test]
    fn test_get_last_one_bit_large_numbers() {
        // Test with higher bit positions
        assert_eq!(get_last_one_bit(1024), 10);     // 2^10
        assert_eq!(get_last_one_bit(1025), 10);     // 2^10 + 1, last one at position 10
        assert_eq!(get_last_one_bit(2048), 11);     // 2^11
    }

    #[test]
    fn test_get_last_one_bit_max_bit() {
        // Test with the highest possible bit (2^63)
        assert_eq!(get_last_one_bit(1u64 << 63), 63);
        
        // Test with all bits set
        assert_eq!(get_last_one_bit(u64::MAX), 63);
    }

    #[test]
    fn test_bit_consistency() {
        // Test that collect_bits and get_last_one_bit are consistent
        for number in [1, 2, 4, 8, 15, 31, 63, 127, 255, 1023] {
            let bits = collect_bits(number);
            let last_one = get_last_one_bit(number);
            
            // Find the actual last one bit manually
            let mut expected_last = 0;
            for i in 0..64 {
                if bits[i] == 1 {
                    expected_last = i;
                }
            }
            
            assert_eq!(last_one, expected_last, "Inconsistency for number {}", number);
        }
    }

    #[test]
    fn test_edge_case_patterns() {
        // Test alternating pattern: 10101010... (0xAAAAAAAAAAAAAAAA)
        let alternating = 0xAAAAAAAAAAAAAAAAu64;
        let last_one = get_last_one_bit(alternating);
        assert_eq!(last_one, 63); // Highest bit in alternating pattern

        // Test all lower 32 bits set
        let lower_32 = (1u64 << 32) - 1;
        assert_eq!(get_last_one_bit(lower_32), 31);
    }
}
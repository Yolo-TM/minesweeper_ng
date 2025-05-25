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
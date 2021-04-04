pub fn add_binary(l: &[bool], r: &[bool]) -> Vec<bool> {
    let mut result = vec![false; l.len() + 1];

    for i in (0..l.len()).rev() {
        let carry_in = result[i + 1];

        let mut bit_result = false;
        let mut carry_out = false;
        let mut on_count: u8 = 0;

        for bit in [carry_in, l[i], r[i]].iter() {
            if *bit {
                on_count += 1
            }
        }

        if on_count >= 2 {
            carry_out = true;
        }
        if on_count == 1 || on_count == 3 {
            bit_result = true;
        }

        result[i + 1] = bit_result;
        result[i] = carry_out;
    }

    result
}

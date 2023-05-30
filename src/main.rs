fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn first_test() {
        assert_eq!(1, 1);
    }

    #[test]
    fn test_shifting() {
        let original_values: [u32; 8] = [
            0xf0f0f0f0, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
            0x00000000,
        ];

        let mut work_values = original_values.clone();

        // Read will interleave all of the bits like our in pins PIO call
        let mut read = move || {
            let mut out: u32 = 0;
            let mut value_index = 0;
            for _bitindex in 0..32 {
                let work_value = &mut work_values[value_index];
                let bit = if (*work_value) & 0x80000000 == 0x80000000 {
                    0x1
                } else {
                    0x0
                };
                out = out << 1;
                out = out | bit;

                *work_value = (*work_value) << 1;
                value_index = (value_index + 1) % work_values.len();
            }
            out
        };

        let mut final_values : [u32;8] = [0;8];

        // Need to read 8 times to get all of the bits shifted in.
        for _ in 0..8 {
            let mut v = read();
            let mut value_index = 0;
            for _ in 0..32 {
                let bit = if 0 == (v & 0x80000000) {
                    0x0
                } else {
                    0x1
                };
                let value = &mut final_values[value_index];
                (*value) = (*value) << 1;
                (*value) = (*value) | bit;
                v = v << 1;
                value_index = (value_index + 1) % final_values.len();
            }
        }

        assert_eq!(original_values, final_values);
    }
}

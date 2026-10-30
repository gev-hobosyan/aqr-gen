pub struct GFMath {
    logs: [u8; 256],
    exponents: [u8; 256]
}

impl GFMath {
    pub fn init() -> Self {
        let mut logs = [0; 256];
        let mut exponents = [0; 256];

        let mut value = 1u8;
        for exponent in 1u8..=255u8 {
            value = if value > 127 {
                (((value as u16) << 1) ^ 285) as u8
            } else {
                value << 1
            };

            logs[value as usize] = exponent % 255;
            exponents[(exponent % 255) as usize] = value;
        }

        Self {
            logs,
            exponents,
        }
    }

    pub fn multiply(&self, a: u8, b: u8) -> u8 {
        if a == 0 && b == 0 {
            0
        } else {
            self.exponents[(self.logs[a as usize] as usize + self.logs[b as usize] as usize) % 255]
        }
    }

    pub fn divide(&self, a: u8, b: u8) -> u8 {
        self.exponents[(self.logs[a as usize] as usize + self.logs[b as usize] as usize * 254) % 255]
    }

    pub fn pol_multiply(&self, pol1: Vec<u8>, pol2: Vec<u8>) -> Vec<u8> {
        let mut coefficients = vec![0; pol1.len() + pol2.len() - 1];

        for i in 0..coefficients.len() {
            for pi1 in 0..=i {
                let pi2 = i - pi1;

                if pi1 >= pol1.len() || pi2 >= pol2.len() {
                    continue
                }

                coefficients[i] ^= self.multiply(pol1[pi1], pol2[pi2]);
            }
        }

        coefficients
    }

    pub fn pol_divide(&self, pol1: Vec<u8>, pol2: Vec<u8>) -> Vec<u8> {
        let quotient_length = pol1.len() - pol2.len() + 1;
        let mut rest = pol1.clone();
        for i in 0..quotient_length {
            if rest[0] != 0 {
                let factor = self.divide(rest[0], pol2[0]);
                let mut subtr = vec![0; rest.len()];
                let product = self.pol_multiply(pol2.clone(), vec![factor]);

                subtr[..product.len()].clone_from_slice(&product);

                rest = rest.iter().enumerate().map(|(index, value)| {
                    value ^ subtr[index]
                }).collect();
                rest.remove(0);
            } else {
                rest.remove(0);
            }
        }

        rest
    }

    pub fn get_pol_generator(&self, degree: u8) -> Vec<u8> {
        let mut last_poly = vec![1];
        for i in 0..(degree as usize) {
            last_poly = self.pol_multiply(last_poly, vec![1, self.exponents[i]])
        }

        last_poly
    }
}
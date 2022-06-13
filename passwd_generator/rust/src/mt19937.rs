pub struct MT19937 {
    mt: [u32; 624],
    mti: usize,
}

impl MT19937 {
    pub fn new(seed: u32) -> Self {
        let mut mt = MT19937 {
            mt: [0; 624],
            mti: 0,
        };

        mt.mt[0] = seed;

        for i in 1..624 {
            mt.mt[i] = 1812433253u32.wrapping_mul(mt.mt[i - 1] ^ mt.mt[i - 1] >> 30);
            mt.mt[i] += i as u32;
        }

        mt
    }

    pub fn new_with_slice(seed: &[u32]) -> Self {
        let mut mt = MT19937::new(19_650_218);
        let mut i = 1;
        let mut j = 0;
        let mut k = if 624 > seed.len() { 624 } else { seed.len() };

        while k != 0 {
            let tmp = (mt.mt[i - 1] ^ (mt.mt[i - 1] >> 30)).wrapping_mul(1_664_525u32);
            mt.mt[i] = (mt.mt[i] ^ tmp)
                .wrapping_add(seed[j])
                .wrapping_add(j as u32);
            mt.mt[i] &= 0xffff_ffffu32;

            i += 1;
            j += 1;
            if i >= 624 {
                i = 1;
                mt.mt[0] = mt.mt[623];
            }
            if j >= seed.len() {
                j = 0;
            }

            k -= 1;
        }

        k = 623;
        while k != 0 {
            let tmp = (mt.mt[i - 1] ^ (mt.mt[i - 1] >> 30)).wrapping_mul(1_566_083_941);
            mt.mt[i] = (mt.mt[i] ^ tmp).wrapping_sub(i as u32);
            mt.mt[i] &= 0xffff_ffffu32;

            i += 1;
            if i >= 624 {
                i = 1;
                mt.mt[0] = mt.mt[623];
            }
            k -= 1;
        }

        mt.mt[0] = 0x8000_0000u32;

        mt
    }

    pub fn gen_u32(&mut self) -> u32 {
        if self.mti == 0 {
            self.twist();
        }

        let mut y = self.mt[self.mti];
        y ^= y >> 11;
        y ^= (y << 7) & 2636928640;
        y ^= (y << 15) & 4022730752;
        y ^= y >> 18;

        self.mti = (self.mti + 1) % 624;

        y
    }

    pub fn gen_below(&mut self, stop: u32) -> u32 {
        // don't call the gen_u32 again to generate a correct value!
        // because if the stop is small, it will call the function many times.
        self.gen_u32() % stop
    }

    pub fn choice<T: Copy>(&mut self, array: &[T]) -> T {
        array[self.gen_below(array.len() as u32) as usize]
    }

    pub fn shuffle<T>(&mut self, array: &mut [T]) {
        for i in (1..array.len()).rev() {
            let j = self.gen_below(i as u32 + 1) as usize;
            array.swap(i, j);
        }
    }

    fn twist(&mut self) {
        for i in 0..624 {
            let y = (self.mt[i] & 0x80000000).wrapping_add(self.mt[(i + 1) % 624] & 0x7fffffff);
            self.mt[i] = (y >> 1) ^ self.mt[(i + 397) % 624];

            if y % 2 != 0 {
                self.mt[i] ^= 0x9908b0df;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::MT19937;

    #[test]
    fn test_mt19937() {
        // copied from http://www.math.sci.hiroshima-u.ac.jp/m-mat/MT/MT2002/CODES/mt19937ar.out
        let correct: Vec<u32> = vec![
            1067595299, 955945823, 477289528, 4107218783, 4228976476, 3344332714, 3355579695,
            227628506, 810200273, 2591290167, 2560260675, 3242736208, 646746669, 1479517882,
            4245472273, 1143372638, 3863670494, 3221021970, 1773610557, 1138697238, 1421897700,
            1269916527, 2859934041, 1764463362, 3874892047, 3965319921, 72549643, 2383988930,
            2600218693, 3237492380, 2792901476, 725331109, 605841842, 271258942, 715137098,
            3297999536, 1322965544, 4229579109, 1395091102, 3735697720, 2101727825, 3730287744,
            2950434330, 1661921839, 2895579582, 2370511479, 1004092106, 2247096681, 2111242379,
            3237345263, 4082424759, 219785033, 2454039889, 3709582971, 835606218, 2411949883,
            2735205030, 756421180, 2175209704, 1873865952, 2762534237, 4161807854, 3351099340,
            181129879, 3269891896, 776029799, 2218161979, 3001745796, 1866825872, 2133627728,
            34862734, 1191934573, 3102311354, 2916517763, 1012402762, 2184831317, 4257399449,
            2899497138, 3818095062, 3030756734, 1282161629, 420003642, 2326421477, 2741455717,
            1278020671, 3744179621, 271777016, 2626330018, 2560563991, 3055977700, 4233527566,
            1228397661, 3595579322, 1077915006, 2395931898, 1851927286, 3013683506, 1999971931,
            3006888962, 1049781534,
        ];
        let mut mt = MT19937::new_with_slice(&[0x123, 0x234, 0x345, 0x456]);
        let mut mine: Vec<u32> = Vec::with_capacity(100);
        for _ in 0..100 {
            mine.push(mt.gen_u32());
        }
        assert_eq!(mine, correct);
    }
}

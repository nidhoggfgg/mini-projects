/*
   this is just a simple implement of MT19937
   see https://en.wikipedia.org/wiki/Mersenne_Twister for more infomation
   and the original in c http://www.math.sci.hiroshima-u.ac.jp/m-mat/MT/MT2002/CODES/mt19937ar.c
   already tested.
*/

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
            mt.mt[i] = 1812433253u32.wrapping_mul(mt.mt[i - 1] ^ mt.mt[i - 1] >> 30) + i as u32;
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

        let y = self.mt[self.mti];
        let y = y ^ y >> 11;
        let y = y ^ y << 7 & 2636928640;
        let y = y ^ y << 15 & 4022730752;
        let y = y ^ y >> 18;

        self.mti = (self.mti + 1) % 624;

        0xffffffff & y
    }

    fn twist(&mut self) {
        for i in 0..624 {
            let y = (self.mt[i] & 0x80000000).wrapping_add(self.mt[(i + 1) % 624] & 0x7fffffff);
            self.mt[i] = (y >> 1) ^ self.mt[(i + 397) % 624];

            if y % 2 != 0 {
                self.mt[i] = self.mt[i] ^ 0x9908b0df;
            }
        }
    }
}

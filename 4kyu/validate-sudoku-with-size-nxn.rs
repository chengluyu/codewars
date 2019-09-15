// https://www.codewars.com/kata/validate-sudoku-with-size-nxn/train/rust

struct Sudoku{
    data: Vec<Vec<u32>>,
}

impl Sudoku{
    fn is_valid(&self) -> bool {
        let len = self.data.len();
        let root = (len as f32).sqrt().floor() as usize;
        let checksum = (1u32..=(len as u32)).fold(0u32, |s, x| s ^ x);
        len > 0 && {
            root * root == len
        } && { // the
            self.data.iter().fold(true, |b, row| {
                b && row.len() == len
            })
        } && {
            self.data.iter().fold(true, |acc, row| {
                acc && row.iter().fold(0u32, |s, x| s ^ x) == checksum
            })
        } && {
            (0usize..len).fold(true, |b, c| {
                b && checksum == (0usize..len).fold(0u32, |s, r| s ^ self.data[r][c])
            })
        } && {
            let mut ans = true;
            for br in (0..len).step_by(root) {
                for bc in (0..len).step_by(root) {
                    let mut sum = 0u32;
                    for r in 0..root {
                        for c in 0..root {
                            sum ^= self.data[br + r][bc + c];
                        }
                    }
                    ans &= sum == checksum;
                }
            }
            ans
        }
    }
}

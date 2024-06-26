use std::sync::Mutex;
// Pseudo Random Number Generator for Quick Sort
lazy_static::lazy_static! {
    static ref RG: Mutex<RandGen> = Mutex::new(RandGen::new(34052));
}
pub fn rand(max: usize) -> usize {
    RG.lock().unwrap().next_v(max)
}
pub struct RandGen {
    curr: usize,
    mul: usize,
    inc: usize,
    modulo: usize,
}

impl RandGen {
    pub fn new(curr: usize) -> Self {
        RandGen {
            curr,
            mul: 56394237,
            inc: 346423491,
            modulo: 23254544563,
        }
    }

    pub fn next_v(&mut self, max: usize) -> usize {
        self.curr = (self.curr * self.mul + self.inc) % self.modulo;
        self.curr % max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_rands_pringout() {
        let mut r = RandGen::new(12);
        for _ in 0..1000 {
            println!("--{}", r.next_v(std::i32::MAX as usize));
        }
        panic!();
    }
}

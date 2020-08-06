pub trait Field: Sized {
    fn new(num: usize, prime: usize) -> Self;

    fn num(&self) -> usize;
    fn prime(&self) -> usize;

    fn repr(&self) {
        println!("Field_{}({})",self.num(), self.prime());
    }

    fn eq_(&self, other: &Self) -> bool {
        self.num() == other.num() && self.prime() == self.prime()
    }

    fn ne_(&self, other: &Self) -> bool {
        self.num() != other.num() || self.prime() != self.prime()
    }

    fn add(&self, other: &Self) -> Self {
        if self.prime() != other.prime() {
            panic!("Cannot add two num()bers in different Fields.");
        }
        let num = (self.num() + other.num()) % self.prime();
        Self::new(num, self.prime())
    }

    fn sub(&self, other: &Self) -> Self {
        if self.prime() != other.prime() {
            panic!("Cannot add two num()bers in different Fields.");
        }

        let i_num = self.num() as isize - other.num() as isize;
        
        let num = if i_num >= 0 {
            (i_num % self.prime() as isize) as usize
        } else {
            ((i_num % self.prime() as isize) + self.prime() as isize) as usize
        };
        
        Self::new(num, self.prime())
    }

    fn mul(&self, other: &Self) -> Self {
        if self.prime() != other.prime() {
            panic!("Cannot multi two num()bers in different Fields.");
        }
        let num = (self.num() * other.num()) % self.prime();
        Self::new(num, self.prime())
    }

    fn pow(&self, other: i32) -> Self {
        let num = if other >= 0 {
            // self.num().pow(other as u32)  % self.prime()
            mod_pow(self.num(), other as usize, self.prime())
        } else {
            // self.num().pow((self.prime() as isize - 1 + other as isize) as u32) % self.prime()
            mod_pow(self.num(), (self.prime() as isize - 1 + other as isize) as usize , self.prime())
        };

        Self::new(num, self.prime())
    }

    fn div(&self, other: &Self) -> Self {
        if self.prime() != other.prime() {
            panic!("Cannot multi two num()bers in different Fields.");
        }

        // let num = (self.num() * other.num().pow(other.prime() as u32 - 2)) % other.prime();
        let num = (self.num() * mod_pow(other.num(), other.prime() - 2, other.prime())) % other.prime();

        Self::new(num, self.prime())
    }

}

// to be check
// from https://rob.co.bb/posts/2019-02-10-modular-exponentiation-in-rust/
pub fn mod_pow(mut base: usize, mut exp: usize, modulus: usize) -> usize {
    if modulus == 1 { return 0 }
    let mut result = 1;
    base = base % modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = result * base % modulus;
        }
        exp = exp >> 1;
        base = base * base % modulus
    }
    result
}



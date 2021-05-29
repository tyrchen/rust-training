pub struct Fibonacci {
    a: u64,
    b: u64,
    cur: u8,
    total: u8,
}

impl Fibonacci {
    pub fn new(total: u8) -> Self {
        Self {
            a: 0,
            b: 0,
            cur: 0,
            total,
        }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur == self.total {
            return None;
        }
        if self.a == 0 {
            self.a = 1;
            self.b = 1;
        } else {
            let c = self.a + self.b;
            self.a = self.b;
            self.b = c;
        }
        self.cur += 1;
        Some(self.a)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut fibo = Fibonacci::new(10);
        assert_eq!(fibo.next(), Some(1));
        for item in fibo {
            println!("item: {}", item);
        }
    }
}

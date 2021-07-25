pub struct  Fibonacci {
    a: u32,
    b: u32,
    cur: u16,
    total: u16,
}

impl Fibonacci {
    pub fn new(total: u16) -> Self { Self { a: 0, b: 0, cur: 0, total: total } }
}

impl Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur == self.total {
            return None
        }

        if self.cur == 0 {
            self.a = 0;
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
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let mut fib = Fibonacci::new(10);
        assert_eq!(fib.next(), Some(0));
        for item in fib {
            println!("item: {}", item);
        }
    }
}
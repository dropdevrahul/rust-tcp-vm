pub mod stack {
    const MAX_SIZE:i64 = 1000000;

    pub struct Stack {
        pub top: i64,
        pub store: Vec<Vec<u8>>,
    }

    impl Stack {
        pub fn peek(&self) -> Result<Vec<u8>, String> {
            if self.top < 0 {
               return Err(String::from("stack underflow"));
            }

            let top = match  u64::try_from(self.top) {
                Ok(n) => n as usize,
                Err(error) => return Err(error.to_string()),
            };
            

            Ok(self.store[top].clone())
        }

        pub fn push(&mut self, buff: Vec<u8>) -> Result<(), String> {
            if self.top == MAX_SIZE - 1 {
               return Err(String::from("stack overflow"));
            }

            self.top = self.top + 1;
            self.store.push(buff);
            Ok(())
        }
    }
}

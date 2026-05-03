/*
  Problem 46: Module Visibility

  Create a struct Account with a private field balance: f64 and public methods
  new(initial), deposit(amount), withdraw(amount), and balance().
  The balance should never go negative. Withdraw returns Result<(), String>.

  Run the tests for this problem with:
    cargo test --test module_visibility_test
*/

pub struct Account {
    balance: f64,
}

impl Account {
    pub fn new(initial: f64) -> Self {
        Account { balance: initial }
    }

    pub fn deposit(&mut self, amount: f64) {
        self.balance += amount;
    }

    pub fn withdraw(&mut self, amount: f64) -> Result<(), String> {
        if self.balance >= amount {
            self.balance -= amount;
            Ok(())
        } else {
            Err("Failed".to_string())
        }
    }

    pub fn balance(&self) -> f64 {
        self.balance
    }
}

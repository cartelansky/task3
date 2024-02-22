trait Account {
    fn deposit(&mut self, amount: f32);
    fn withdraw(&mut self, amount: f32);
    fn balance(&self) -> f32;
}

struct BankAccount {
    account_number: u32,
    holder_name: String,
    balance: f32,
}

impl Account for BankAccount {
    fn deposit(&mut self, amount: f32) {
        self.balance += amount;
    }
    fn withdraw(&mut self, amount: f32) {
        self.balance -= amount;
    }
    fn balance(&self) -> f32 {
        self.balance
    }
}

fn main() {
    let mut account1 = BankAccount {
        account_number: 123456,
        holder_name: "Elon Musk".to_string(),
        balance: 100.0,
    };

    let mut account2 = BankAccount {
        account_number: 789456,
        holder_name: String::from("Kendall Roy"),
        balance: 250.0,
    };

    account1.deposit(275.0);
    account2.withdraw(150.0);

    println!(
        "account number: {}, holder name: {}, balance: {}",
        account1.account_number,
        account1.holder_name,
        account1.balance()
    );
    println!(
        "account number: {}, holder name: {}, balance: {}",
        account2.account_number,
        account2.holder_name,
        account2.balance()
    );
}

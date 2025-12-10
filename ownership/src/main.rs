// fn main() {
//     let s1 = String::from("hello");
//     let len = calculate_length(&s1);
//     println!("The length of '{}' is {}.", s1, len);

//     let s2 = s1.clone(); // s1 is moved to s2
//     println!("s1 is now '{}'", s1);
// }

// fn calculate_length(s: &String) -> usize {
//     s.len()
// }

// fn main() {
//     let mut x = 5;
//     let r = &mut x;

//     *r += 1;

//     println!("The value of x is: {}", x);
//     println!("The value of r is: {}", r);
// }

fn main() {
    let mut account = BankAccount {
        owner: "Sameer".to_string(),
        balance: 1000.0,
    };

    account.withdraw(300.0);
    account.check_balance();
}

struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount {
    fn withdraw(&mut self, amount: f64) {
        println!("Withdrawing ${} from {}", amount, self.owner);
        self.balance -= amount;
    }

    fn check_balance(&self) {
        println!(
            "Account Owned by {} has balance ${}",
            self.owner, self.balance
        );
    }
}

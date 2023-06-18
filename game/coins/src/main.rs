/*
    Implementation fo coin system with enum form chapter 6 of Rust book
*/

#[derive(Debug)]
enum Coin {
    Copper,
    Nickel,
    Silver,
    Gold,
    Platinum,
}

impl Coin {
    fn get_coin(index: i32) -> Coin {
        match index.abs() % 5 {
            0 => Coin::Copper,
            1 => Coin::Nickel,
            2 => Coin::Silver,
            3 => Coin::Gold,
            4 => Coin::Platinum,
            _ => panic!("Invalid index"),
        }
    }

    fn get_index(coin: Coin) -> i32 {
        match coin {
            Coin::Copper => 0,
            Coin::Nickel => 1,
            Coin::Silver => 2,
            Coin::Gold => 3,
            Coin::Platinum => 4,
        }
    }
}

struct Account {
    owner: String,
    balance: [i32; 5],
}

impl Account {
    fn init(owner: String) -> Account {
        Account {
            owner,
            balance: [0; 5],
        }
    }

    fn add(&mut self, coin: Coin, amount: i32) {
        match coin {
            Coin::Copper => self.balance[0] += amount,
            Coin::Nickel => self.balance[1] += amount,
            Coin::Silver => self.balance[2] += amount,
            Coin::Gold => self.balance[3] += amount,
            Coin::Platinum => self.balance[4] += amount,
        }
        let new_balance = self.get_balance_of(&coin);
        println!(
            "New transaction for {}: {:?}: {} => {} ({})",
            self.owner,
            coin,
            new_balance - amount,
            new_balance,
            amount
        )
    }

    fn round(&mut self) {
        for i in 0..(self.balance.len() - 1) {
            // let p: i32 = ((i + 1) % 2 * 8 + 8) as i32;
            let p = 16;
            let b: i32 = self.balance[i];
            self.balance[i] = b % p;
            self.balance[i + 1] += b / p;
        }
    }

    fn get_balance_in(&mut self, coin: Coin) -> i32 {
        self.round();
        let coin_index = Coin::get_index(coin);
        let mut sum = 0;
        for i in 0..(self.balance.len()) {
            if coin_index <= i as i32 {
                sum += self.balance[i] * i32::pow(16, (i as i32 - coin_index) as u32)
            }
        }
        return sum
    }

    fn get_balance_of(&self, coin: &Coin) -> i32 {
        match coin {
            Coin::Copper => self.balance[0],
            Coin::Nickel => self.balance[1],
            Coin::Silver => self.balance[2],
            Coin::Gold => self.balance[3],
            Coin::Platinum => self.balance[4],
        }
    }

    fn print(&self) {
        for i in 0..self.balance.len() {
            println!("{:?}: {}", Coin::get_coin(i as i32), self.balance[i])
        }
        println!()
    }
}

fn main() {
    let mut account = Account::init(String::from("Alex"));

    account.add(Coin::Copper, -1333);
    account.add(Coin::Nickel, 19);
    account.add(Coin::Silver, 71);
    account.add(Coin::Gold, -8);
    account.add(Coin::Platinum, 4);
    account.print();
    println!("Balance in Nickle: {}", account.get_balance_in(Coin::Nickel));
    account.print();
    println!("Amount of Silver: {}", account.get_balance_of(&Coin::Silver));
}

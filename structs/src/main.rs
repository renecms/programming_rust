mod investment_account;
use std::rc::Rc;

use investment_account::{InvestmentAccount, InvestmentType};

fn main() {
    let tesla = Rc::new(String::from("TSLA"));
    let bitcoin = Rc::new(String::from("BTC"));
    let o = Rc::new(String::from("O"));
    let mut account = InvestmentAccount::new("Rene".to_string(), 15605.);
    println!("{:?}", account);

    println!("{}", account.add_funds(15678000.).unwrap());
    println!("{:?}", account);

    let result = account.buy_asset(tesla.clone(), InvestmentType::STOCKS, 5., 970.12);
    println!("{:?}", result);

    let result = account.buy_asset(bitcoin.clone(), InvestmentType::CRYPTOCURRENCY, 1., 47658.34);
    println!("{:?}", result);

    let result = account.buy_asset(o.clone(), InvestmentType::REITS, 5., 870.43);
    println!("{:?}", result);

    let result = account.buy_asset(bitcoin.clone(), InvestmentType::CRYPTOCURRENCY, 6., 42658.34);
    println!("{:?}", result);

    account.show_operations();
    account.show_portfolio();

    println!("bitcoin strong count {}", Rc::strong_count(&bitcoin))
}

mod investment_account;
use investment_account::{InvestmentAccount, InvestmentType};

fn main() {
    let mut account = InvestmentAccount::new("Rene".to_string(), 15605.);
    println!("{:?}", account);

    println!("{}", account.add_funds(15678000.).unwrap());
    println!("{:?}", account);

    let result = account.buy_asset("TSLA".to_string(), InvestmentType::STOCKS, 5., 970.12);
    println!("{:?}", result);

    let result = account.buy_asset("BTC".to_string(), InvestmentType::CRYPTOCURRENCY, 1., 47658.34);
    println!("{:?}", result);

    let result = account.buy_asset("O".to_string(), InvestmentType::REITS, 5., 870.43);
    println!("{:?}", result);

    let result = account.buy_asset("BTC".to_string(), InvestmentType::CRYPTOCURRENCY, 6., 42658.34);
    println!("{:?}", result);

    println!("{:?}", account);
}

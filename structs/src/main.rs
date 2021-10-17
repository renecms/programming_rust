mod investment_account;
use investment_account::{InvestmentAccount, InvestmentType};

fn main() {
    let tesla =String::from("TSLA");
    let bitcoin = String::from("BTC");
    let o = String::from("O");
    let mut account = InvestmentAccount::new("Rene".to_string(), 15605.);
    println!("{:?}", account);

    println!("{}", account.add_funds(15678000.).unwrap());
    println!("{:?}", account);

    let result = account.buy_asset(&tesla, InvestmentType::STOCKS, 5., 970.12);
    println!("{:?}", result);

    let result = account.buy_asset(&bitcoin, InvestmentType::CRYPTOCURRENCY, 1., 47658.34);
    println!("{:?}", result);

    let result = account.buy_asset(&o, InvestmentType::REITS, 5., 870.43);
    println!("{:?}", result);

    let result = account.buy_asset(&bitcoin, InvestmentType::CRYPTOCURRENCY, 6., 42658.34);
    println!("{:?}", result);

    account.show_operations();
    account.show_portfolio();
}

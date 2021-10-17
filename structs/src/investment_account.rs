use std::{collections::HashMap, rc::Rc};

#[derive(Debug)]
pub enum InvestmentType {
    CRYPTOCURRENCY,
    STOCKS,
    REITS
}

#[derive(Debug)]
pub struct Operation {
    asset_name: Rc<String>,
    asset_type: InvestmentType,
    quantity: f64,
    amount: f64
}

#[derive(Debug)]
pub struct InvestmentAccount {
    owner: String,
    balance: f64,
    operations: Vec<Operation>,
    portfolio: HashMap<Rc<String>, f64>
}

impl InvestmentAccount { 
    pub fn new(owner: String, balance: f64) -> Self {
        InvestmentAccount { owner, balance, operations: vec!(), portfolio: HashMap::new() }
    } 

    pub fn add_funds(&mut self, amount: f64) -> Result<String, String> {
        if amount <= 0. {
            return Err("Amount to be added should be > 0".to_string());
        }
        self.balance = self.balance + amount;
        Ok(format!("Adding ${:.02} to the balance. New balance is ${:.02}", amount, self.balance))
    }

    pub fn buy_asset(&mut self, asset_name: Rc<String>, asset_type: InvestmentType, quantity: f64, cost: f64) -> Result<String, String> {
        let operation_total = quantity * cost;
        if self.is_operation_allowed(operation_total) {
            self.save_operation(asset_name.clone(), asset_type, quantity, operation_total);
            self.update_balance(operation_total);
            self.update_portfolio(asset_name.clone(), quantity);
            Ok(format!("Bought {} {} for ${:.02}", quantity, asset_name, operation_total))
        } else {
            Err(format!("Not enough funds. Balance is ${:.02} and operation total is ${:.02}", self.balance, operation_total))
        }
    }

    pub fn show_operations(&self){
        for operation in &self.operations {
            println!("{:?}", operation);
        }
    }

    pub fn show_portfolio(&self){
        for (asset_name, amount) in &self.portfolio {
            println!("{}: {}", asset_name, amount);
        }
    }

    fn is_operation_allowed(&self, operation_total: f64) -> bool {
        self.balance >= operation_total
    }

    fn save_operation(&mut self, asset_name: Rc<String>, asset_type: InvestmentType, quantity: f64, operation_total: f64) {
        let operation = Operation { asset_name: asset_name.clone(), asset_type, quantity, amount: operation_total };
        self.operations.push(operation);
    }

    fn update_balance(&mut self, operation_total: f64) {
        self.balance = self.balance - operation_total;
    }

    fn update_portfolio(&mut self, asset_name: Rc<String>, quantity: f64) {
        if self.portfolio.contains_key(&asset_name) {
            let value = self.portfolio.get_mut(&asset_name).unwrap();
            *value = *value + quantity;
        } else {
            self.portfolio.insert(asset_name.clone(), quantity);
        }
    }
}

#[cfg(test)]
mod test {
    use std::rc::Rc;

    use super::InvestmentAccount;
    #[test]
    fn add_123_funds_test_success(){
        let mut account = InvestmentAccount::new("Rene".to_string(), 0.);
    
        account.add_funds(123.).unwrap();
        assert_eq!(account.balance, 123.)
    }
    
    #[test]
    fn add_negative_funds_test_should_fail(){
        let mut account = InvestmentAccount::new("Rene".to_string(), 0.);
    
        match account.add_funds(-123.) {
            Err(_) => (),
            Ok(_) => panic!("should have returned an error")
        }
        assert_eq!(account.balance, 0.)
    }

    #[test]
    #[should_panic(expected="Amount to be added should be > 0")]
    fn add_negative_funds_test_should_panic(){
        let mut account = InvestmentAccount::new("Rene".to_string(), 0.);
    
        account.add_funds(-123.).unwrap();
    }
}

use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]

pub struct Sellable {
    price: u128,
    name: String,
    description: String,
}

impl fmt::Display for Sellable {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        write!(f, "Name: {},\n Price:{},\n Description: {} items",self.name, self.price, self.description)
    }
}

pub fn create_sellable( self_price: u128, selfname: String, self_description: String) -> Sellable {
    return Sellable {
        name: selfname,
        price: self_price,
        description: self_description,
    }
}

pub struct User {
    name: String,
    balance: u128,
    owned: HashMap<String, Sellable>,
}

impl User {
    pub fn add_sellable(&mut self, object: Sellable) {
        self.owned.insert(
            object.name.clone(), 
            create_sellable(object.price, object.name.clone(), object.description)
        );
    }
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        write!(f, "Username: {}, current balance: {}, He has {} items \n",self.name, self.balance, self.owned.capacity())
    }
}

pub fn create_user(selfname: String) -> User {
    return User {
        name: selfname,
        balance: 0,
        owned: HashMap::new()
    }
}

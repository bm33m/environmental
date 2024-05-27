/*
DIY project no. 25003
Write a trait for an virtual currency model.

DIY project no. 25006
Write a rust crate to implement a trait for a virtual currency model.
Let the users record the values of a virtual currency, indicating the highes and lows of each day.
Let the program send messages to users when the new high or low record has been reached.


*/


pub trait Currency {
    fn get_currency_symbol(&self) -> String;
    fn get_code(&self) -> String;
    fn get_name(&self) -> String;
    fn get_network(&self) -> String;
    fn get_address(&self) -> Address;
    fn get_balance(&self) -> Balance;
    fn get_currency_info(&self) -> (Vec<CurrencyInfo>, String, String, i64, f64);
}

#[derive(Clone, Debug)]
pub struct Address {
    id: i64,
}

#[derive(Clone, Debug)]
pub struct Balance {
    value: f64,
}

#[derive(Clone, Debug)]
pub struct CurrencyInfo {
    pub data: CurrencyData,
    pub indicators: Indicators,
    pub stats: Stats,
}

#[derive(Clone, Debug)]
pub struct Stats {
    pub lowest_price: f64,
    pub highest_price: f64,
    pub price_lowest_date: String,
    pub price_lowest_time: String,
    pub price_highest_date: String,
    pub price_highest_time: String,
}

#[derive(Clone, Debug)]
pub struct CurrencyData {
    pub code: String,
    pub day: i32,
    pub date: String,
    pub month: String,
    pub day_of_the_week: String,
    pub time: String,
    pub network: String,
    pub price: f64,
}

#[derive(Clone, Debug)]
pub struct Indicators {
    pub up: bool,
    pub down: bool,
    pub previous: f64,
    pub current: f64,
    pub rate: f64,
    pub code: String,
    pub date: String,
    pub time: String,
}

#[derive(Clone, Debug)]
pub struct Coin {
    name: String,
    code: String,
    symbol: String,
    network: String,
    address: Address,
    balance: Balance,
}

#[derive(Clone, Debug)]
pub struct CurrencyModel {
    pub coin: Coin,
    pub symbol: String,
    pub network: String,
    pub sample: Vec<CurrencyData>,
}

impl Currency for CurrencyModel {
    fn get_currency_symbol(&self) -> String {
        self.symbol.clone()
    }

    fn get_code(&self) -> String {
        self.coin.code.clone()
    }

    fn get_name(&self) -> String {
        self.coin.name.clone()
    }

    fn get_network(&self) -> String {
        self.network.clone()
    }

    fn get_address(&self) -> Address {
        self.coin.address.clone()
    }

    fn get_balance(&self) -> Balance {
        self.coin.balance.clone()
    }

    fn get_currency_info(&self) -> (Vec<CurrencyInfo>, String, String, i64, f64) {
        let len = self.sample.len();
        assert!(len > 0);
        let mut info: Vec<CurrencyInfo> = Vec::new();
        let id = self.coin.address.id;
        let value = self.coin.balance.value;
        let symbol = self.coin.symbol.clone();
        let network = self.coin.network.clone();
        let mut price_highest_date = &self.sample[0].date;
        let mut price_lowest_date = price_highest_date;
        let mut previous_price = self.sample[0].price;
        let mut highest_price = previous_price;
        let mut lowest_price = previous_price;
        let mut price_highest_time = &self.sample[0].time;
        let mut price_lowest_time = price_highest_time;
        for data in &self.sample {
            let current_price = data.price;
            let indicators = Indicators {
                up: (current_price > previous_price),
                down: (current_price < previous_price),
                previous: previous_price,
                current: current_price,
                rate: price_increase_or_decrease_rate(current_price, previous_price),
                code: data.code.clone(),
                date: data.date.clone(),
                time: data.time.clone(),
            };
            if current_price > highest_price {
                highest_price = current_price;
                price_highest_date = &data.date;
                price_highest_time = &data.time;
            } else if current_price < lowest_price {
                lowest_price = current_price;
                price_lowest_date = &data.date;
                price_lowest_time = &data.time;
            }
            let stats = Stats{
                lowest_price: lowest_price,
                highest_price: highest_price,
                price_lowest_date: price_lowest_date.to_string(),
                price_lowest_time: price_lowest_time.to_string(),
                price_highest_date: price_highest_date.to_string(),
                price_highest_time: price_highest_time.to_string(),
            };
            info.push(CurrencyInfo{data: (*data).clone(), indicators: indicators, stats: stats});
            previous_price = current_price;
        }

        (info, symbol, network, id, value)
    }
}

impl CurrencyModel {
    pub fn get_currency_info_x(&self) -> (Vec<CurrencyInfo>, String, String, i64, f64) {
      self.get_currency_info()
    }
}

impl Coin {
    pub fn get_currency_symbol(&self) -> String {
        self.symbol.clone()
    }

    pub fn get_network(&self) -> String {
        self.network.clone()
    }
}

pub fn new(name: &str, code: &str, symbol: &str, id: i64)-> Coin {
    Coin {
        name: name.to_string(), // String,
        code: code.to_string(), // String,
        symbol: symbol.to_string(), // String,
        network: name.to_string(), // String,
        address: Address{id: id}, // Address,
        balance: Balance{value: 0.00}, // Balance,
    }
}

pub fn price_increase_or_decrease_rate(current_price: f64, previous_price: f64) -> f64 {
    assert!(previous_price > 0.00);
    if current_price > previous_price {
        (100.00 * (current_price - previous_price)) / previous_price
    } else {
        (100.00 * (previous_price - current_price)) / previous_price
    }
}

pub fn add_data(code: &str, day: i32, date: &str, month: &str,
    day_of_the_week: &str, time: &str, network: &str, price: f64,
    sample: &mut Vec<CurrencyData>,) {
    let data = CurrencyData {
        code: code.to_string(), // String,
        day: day, // i32,
        date: date.to_string(), // String,
        month: month.to_string(), // String,
        day_of_the_week: day_of_the_week.to_string(), // String,
        time: time.to_string(), // String,
        network: network.to_string(), // String,
        price: price, // f64,
    };
    sample.push(data);
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_price_increase_or_decrease_rate() {
        let price_info = price_increase_or_decrease_rate(234.00, 123.00);
        assert!(price_info == 90.2439024390244);
    }

    #[test]
    fn test_get_currency_info() {
        let coin_x = Coin {
            name: "Ubhozo".to_string(), // String,
            code: "Ubh".to_string(), // String,
            symbol: "Ubh".to_string(), // String,
            network: "Ubhozo".to_string(), // String,
            address: Address{id: 9024390243902442}, // Address,
            balance: Balance{value: 0.00}, // Balance,
        };
        let mut currency_x = CurrencyModel {
            coin: coin_x.clone(), // Coin,
            symbol: coin_x.symbol, // String,
            network: coin_x.network, // String,
            sample: Vec::<CurrencyData>::new(), // Vec<CurrencyData>,
        };
        // add_data(code: &str, day: i32, date: &str, month: &str,
        //    day_of_the_week: &str, time: &str, network: &str, price: f64,
        //    sample: &mut Vec<CurrencyData>,)
        add_data("Ubh", 1, "2024/05/14", "May",
            "Tuesday", "13:00:00", "Ubhozo", 13.67,
            &mut currency_x.sample,);
        add_data("Ubh", 2, "2024/05/15", "May",
            "Wednesday", "13:00:00", "Ubhozo", 12.00,
            &mut currency_x.sample,);
        add_data("Ubh", 3, "2024/05/16", "May",
            "Thursday", "11:30:50", "Ubhozo", 14.90,
            &mut currency_x.sample,);
        let info = currency_x.get_currency_info();
        assert!(info.0[2].stats.lowest_price == 12.00)
    }
}

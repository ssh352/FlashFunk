#![allow(dead_code, unused_imports, unused_must_use, unused_variables)]

use ctpbee_rs::app::CtpbeeR;
use ctpbee_rs::ac::Ac;
use ctpbee_rs::structs::{BarData, TickData, LoginForm};
use ctpbee_rs::ctp::api::MdApi;
use std::thread;
use actix::Addr;
use std::borrow::Borrow;
use ctpbee_rs::interface::Interface;

struct Strategy {
    pub name: String,
    pub addr: Option<Addr<CtpbeeR>>,
}

impl Ac for Strategy {
    fn on_bar(&mut self, bar: BarData) {
        let name = self.name.clone();
        println!("{} got bar", name);
    }

    fn on_tick(&mut self, tick: TickData) {
        let name = self.name.clone();
        println!("{} got tick {:?}", name, self.get_addr());
    }

    fn init(&mut self, runtime: Addr<CtpbeeR>) {
        self.addr = Some(runtime);
    }

    fn get_addr(&mut self) -> &Addr<CtpbeeR> {
        self.addr.as_ref().unwrap()
    }
}

#[actix_rt::main]
async fn main() {
    // create main actor
    let mut account = CtpbeeR::new("ctpbee".to_string());
    let str = Strategy { name: "hello".to_string(), addr: None };
    let str2 = Strategy { name: "bug".to_string(), addr: None };
    // ADD strategy to main Actor
    account.add_strategy(Box::new(str));
    account.add_strategy(Box::new(str2));
    let (addr, x) = account.run_forever();
    let copy = addr.clone();
    // here is the call c++ code
    let mut md_api = MdApi::new("name".to_string(), "id".to_string(), "bug".to_string(), addr);
    md_api.init();
    let trading_day = md_api.get_trading_day();
    println!("trading day:{} ", trading_day);
    let login_form = LoginForm {
        user_id: "089131".to_string(),
        password: "350888".to_string(),
        broke_id: "9999".to_string(),
        app_id: "simnow_client_test".to_string(),
        md_address: "tcp://218.202.237.33:10112".to_string(),
        td_address: "tcp://218.202.237.33:10102".to_string(),
        auth_code: "0000000000000000".to_string(),
        production_info: "".to_string(),
    };
    md_api.connect(&login_form);
    // wait
    x.await;
}
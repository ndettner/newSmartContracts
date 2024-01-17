// Code generated by schema tool; DO NOT EDIT.

#![allow(dead_code)]
#![allow(unused_imports)]

use wasmlib::*;

use crate::*;

#[derive(Clone)]
pub struct ArrayOfImmutableString {
    pub(crate) proxy: Proxy,
}

impl ArrayOfImmutableString {
    pub fn length(&self) -> u32 {
        self.proxy.length()
    }

    pub fn get_string(&self, index: u32) -> ScImmutableString {
        ScImmutableString::new(self.proxy.index(index))
    }
}

#[derive(Clone)]
pub struct Immutableshop1State {
    pub(crate) proxy: Proxy,
}

impl Immutableshop1State {
    pub fn new() -> Immutableshop1State {
        Immutableshop1State {
            proxy: state_proxy(),
        }
    }

    // earned Money for every Shop
    pub fn earned_money(&self) -> MapStringToImmutableUint64 {
        MapStringToImmutableUint64 { proxy: self.proxy.root(STATE_EARNED_MONEY) }
    }

    // Hname of Shop
    pub fn festival_hname(&self) -> ScImmutableHname {
        ScImmutableHname::new(self.proxy.root(STATE_FESTIVAL_HNAME))
    }

    // all finished products which are ready to be deployed
    pub fn finished_products(&self) -> ArrayOfImmutableProduct {
        ArrayOfImmutableProduct { proxy: self.proxy.root(STATE_FINISHED_PRODUCTS) }
    }

    // current owner of this smart contract
    pub fn owner(&self) -> ScImmutableAgentID {
        ScImmutableAgentID::new(self.proxy.root(STATE_OWNER))
    }

    // amount of products produced per Shop
    pub fn produced_products(&self) -> MapStringToImmutableUint64 {
        MapStringToImmutableUint64 { proxy: self.proxy.root(STATE_PRODUCED_PRODUCTS) }
    }

    // all Products in Production
    pub fn producing_products(&self) -> ArrayOfImmutableStatisticProduct {
        ArrayOfImmutableStatisticProduct { proxy: self.proxy.root(STATE_PRODUCING_PRODUCTS) }
    }

    // product ID
    pub fn product_id(&self) -> ScImmutableUint64 {
        ScImmutableUint64::new(self.proxy.root(STATE_PRODUCT_ID))
    }

    // Product Templates
    pub fn product_templates(&self) -> ArrayOfImmutableProductTemplate {
        ArrayOfImmutableProductTemplate { proxy: self.proxy.root(STATE_PRODUCT_TEMPLATES) }
    }

    // array of all Shops
    pub fn shop_names(&self) -> ArrayOfImmutableString {
        ArrayOfImmutableString { proxy: self.proxy.root(STATE_SHOP_NAMES) }
    }

    // amount of products sold per shop
    pub fn sold_products(&self) -> MapStringToImmutableUint64 {
        MapStringToImmutableUint64 { proxy: self.proxy.root(STATE_SOLD_PRODUCTS) }
    }

    // sum of total earned Money
    pub fn total_earned_money(&self) -> ScImmutableUint64 {
        ScImmutableUint64::new(self.proxy.root(STATE_TOTAL_EARNED_MONEY))
    }

    // amount of al produced products
    pub fn total_produced_products(&self) -> ScImmutableUint64 {
        ScImmutableUint64::new(self.proxy.root(STATE_TOTAL_PRODUCED_PRODUCTS))
    }

    // amount of total sold products
    pub fn total_sold_products(&self) -> ScImmutableUint64 {
        ScImmutableUint64::new(self.proxy.root(STATE_TOTAL_SOLD_PRODUCTS))
    }
}

#[derive(Clone)]
pub struct ArrayOfMutableString {
    pub(crate) proxy: Proxy,
}

impl ArrayOfMutableString {
    pub fn append_string(&self) -> ScMutableString {
        ScMutableString::new(self.proxy.append())
    }

    pub fn clear(&self) {
        self.proxy.clear_array();
    }

    pub fn length(&self) -> u32 {
        self.proxy.length()
    }

    pub fn get_string(&self, index: u32) -> ScMutableString {
        ScMutableString::new(self.proxy.index(index))
    }
}

#[derive(Clone)]
pub struct Mutableshop1State {
    pub(crate) proxy: Proxy,
}

impl Mutableshop1State {
    pub fn new() -> Mutableshop1State {
        Mutableshop1State {
            proxy: state_proxy(),
        }
    }
    pub fn as_immutable(&self) -> Immutableshop1State {
        Immutableshop1State { proxy: self.proxy.root("") }
    }

    // earned Money for every Shop
    pub fn earned_money(&self) -> MapStringToMutableUint64 {
        MapStringToMutableUint64 { proxy: self.proxy.root(STATE_EARNED_MONEY) }
    }

    // Hname of Shop
    pub fn festival_hname(&self) -> ScMutableHname {
        ScMutableHname::new(self.proxy.root(STATE_FESTIVAL_HNAME))
    }

    // all finished products which are ready to be deployed
    pub fn finished_products(&self) -> ArrayOfMutableProduct {
        ArrayOfMutableProduct { proxy: self.proxy.root(STATE_FINISHED_PRODUCTS) }
    }

    // current owner of this smart contract
    pub fn owner(&self) -> ScMutableAgentID {
        ScMutableAgentID::new(self.proxy.root(STATE_OWNER))
    }

    // amount of products produced per Shop
    pub fn produced_products(&self) -> MapStringToMutableUint64 {
        MapStringToMutableUint64 { proxy: self.proxy.root(STATE_PRODUCED_PRODUCTS) }
    }

    // all Products in Production
    pub fn producing_products(&self) -> ArrayOfMutableStatisticProduct {
        ArrayOfMutableStatisticProduct { proxy: self.proxy.root(STATE_PRODUCING_PRODUCTS) }
    }

    // product ID
    pub fn product_id(&self) -> ScMutableUint64 {
        ScMutableUint64::new(self.proxy.root(STATE_PRODUCT_ID))
    }

    // Product Templates
    pub fn product_templates(&self) -> ArrayOfMutableProductTemplate {
        ArrayOfMutableProductTemplate { proxy: self.proxy.root(STATE_PRODUCT_TEMPLATES) }
    }

    // array of all Shops
    pub fn shop_names(&self) -> ArrayOfMutableString {
        ArrayOfMutableString { proxy: self.proxy.root(STATE_SHOP_NAMES) }
    }

    // amount of products sold per shop
    pub fn sold_products(&self) -> MapStringToMutableUint64 {
        MapStringToMutableUint64 { proxy: self.proxy.root(STATE_SOLD_PRODUCTS) }
    }

    // sum of total earned Money
    pub fn total_earned_money(&self) -> ScMutableUint64 {
        ScMutableUint64::new(self.proxy.root(STATE_TOTAL_EARNED_MONEY))
    }

    // amount of al produced products
    pub fn total_produced_products(&self) -> ScMutableUint64 {
        ScMutableUint64::new(self.proxy.root(STATE_TOTAL_PRODUCED_PRODUCTS))
    }

    // amount of total sold products
    pub fn total_sold_products(&self) -> ScMutableUint64 {
        ScMutableUint64::new(self.proxy.root(STATE_TOTAL_SOLD_PRODUCTS))
    }
}

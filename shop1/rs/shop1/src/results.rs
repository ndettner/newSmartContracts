// Code generated by schema tool; DO NOT EDIT.

#![allow(dead_code)]
#![allow(unused_imports)]

use wasmlib::*;

use crate::*;

#[derive(Clone)]
pub struct ImmutableCallCheckProductResults {
    pub proxy: Proxy,
}

impl ImmutableCallCheckProductResults {
    // shows if SC can produce Product
    pub fn can_produce(&self) -> ScImmutableBool {
        ScImmutableBool::new(self.proxy.root(RESULT_CAN_PRODUCE))
    }

    // optional error message
    pub fn message(&self) -> ScImmutableString {
        ScImmutableString::new(self.proxy.root(RESULT_MESSAGE))
    }
}

#[derive(Clone)]
pub struct MutableCallCheckProductResults {
    pub proxy: Proxy,
}

impl MutableCallCheckProductResults {
    pub fn new() -> MutableCallCheckProductResults {
        MutableCallCheckProductResults {
            proxy: results_proxy(),
        }
    }

    // shows if SC can produce Product
    pub fn can_produce(&self) -> ScMutableBool {
        ScMutableBool::new(self.proxy.root(RESULT_CAN_PRODUCE))
    }

    // optional error message
    pub fn message(&self) -> ScMutableString {
        ScMutableString::new(self.proxy.root(RESULT_MESSAGE))
    }
}

#[derive(Clone)]
pub struct ImmutableCallPingShopResults {
    pub proxy: Proxy,
}

impl ImmutableCallPingShopResults {
    // shows if ping was successful
    pub fn ping_successful(&self) -> ScImmutableBool {
        ScImmutableBool::new(self.proxy.root(RESULT_PING_SUCCESSFUL))
    }
}

#[derive(Clone)]
pub struct MutableCallPingShopResults {
    pub proxy: Proxy,
}

impl MutableCallPingShopResults {
    pub fn new() -> MutableCallPingShopResults {
        MutableCallPingShopResults {
            proxy: results_proxy(),
        }
    }

    // shows if ping was successful
    pub fn ping_successful(&self) -> ScMutableBool {
        ScMutableBool::new(self.proxy.root(RESULT_PING_SUCCESSFUL))
    }
}

#[derive(Clone)]
pub struct ArrayOfImmutableProduct {
    pub(crate) proxy: Proxy,
}

impl ArrayOfImmutableProduct {
    pub fn length(&self) -> u32 {
        self.proxy.length()
    }


    pub fn get_product(&self, index: u32) -> ImmutableProduct {
        ImmutableProduct { proxy: self.proxy.index(index) }
    }
}

#[derive(Clone)]
pub struct ImmutableCallRecieveProductsResults {
    pub proxy: Proxy,
}

impl ImmutableCallRecieveProductsResults {
    // products that are ready to ship to russfest
    pub fn product(&self) -> ArrayOfImmutableProduct {
        ArrayOfImmutableProduct { proxy: self.proxy.root(RESULT_PRODUCT) }
    }
}

#[derive(Clone)]
pub struct ArrayOfMutableProduct {
    pub(crate) proxy: Proxy,
}

impl ArrayOfMutableProduct {

    pub fn append_product(&self) -> MutableProduct {
        MutableProduct { proxy: self.proxy.append() }
    }
    pub fn clear(&self) {
        self.proxy.clear_array();
    }

    pub fn length(&self) -> u32 {
        self.proxy.length()
    }


    pub fn get_product(&self, index: u32) -> MutableProduct {
        MutableProduct { proxy: self.proxy.index(index) }
    }
}

#[derive(Clone)]
pub struct MutableCallRecieveProductsResults {
    pub proxy: Proxy,
}

impl MutableCallRecieveProductsResults {
    pub fn new() -> MutableCallRecieveProductsResults {
        MutableCallRecieveProductsResults {
            proxy: results_proxy(),
        }
    }

    // products that are ready to ship to russfest
    pub fn product(&self) -> ArrayOfMutableProduct {
        ArrayOfMutableProduct { proxy: self.proxy.root(RESULT_PRODUCT) }
    }
}

#[derive(Clone)]
pub struct ArrayOfImmutableStatisticProduct {
    pub(crate) proxy: Proxy,
}

impl ArrayOfImmutableStatisticProduct {
    pub fn length(&self) -> u32 {
        self.proxy.length()
    }


    pub fn get_statistic_product(&self, index: u32) -> ImmutableStatisticProduct {
        ImmutableStatisticProduct { proxy: self.proxy.index(index) }
    }
}

#[derive(Clone)]
pub struct ArrayOfImmutableProductTemplate {
    pub(crate) proxy: Proxy,
}

impl ArrayOfImmutableProductTemplate {
    pub fn length(&self) -> u32 {
        self.proxy.length()
    }


    pub fn get_product_template(&self, index: u32) -> ImmutableProductTemplate {
        ImmutableProductTemplate { proxy: self.proxy.index(index) }
    }
}

#[derive(Clone)]
pub struct ImmutableCallGetShopStatisticsResults {
    pub proxy: Proxy,
}

impl ImmutableCallGetShopStatisticsResults {
    // earnings of shop
    pub fn earnings(&self) -> ScImmutableUint64 {
        ScImmutableUint64::new(self.proxy.root(RESULT_EARNINGS))
    }

    // max production step for this shop
    pub fn max_productions_step(&self) -> ScImmutableUint64 {
        ScImmutableUint64::new(self.proxy.root(RESULT_MAX_PRODUCTIONS_STEP))
    }

    // produced Products for this Shop
    pub fn produced_products(&self) -> ScImmutableUint64 {
        ScImmutableUint64::new(self.proxy.root(RESULT_PRODUCED_PRODUCTS))
    }

    // all Products that currently are in production
    pub fn production(&self) -> ArrayOfImmutableStatisticProduct {
        ArrayOfImmutableStatisticProduct { proxy: self.proxy.root(RESULT_PRODUCTION) }
    }

    // all ProductTemplates that exist for this shop
    pub fn production_templates(&self) -> ArrayOfImmutableProductTemplate {
        ArrayOfImmutableProductTemplate { proxy: self.proxy.root(RESULT_PRODUCTION_TEMPLATES) }
    }

    // sold porducts for this shop
    pub fn sold_products(&self) -> ScImmutableUint64 {
        ScImmutableUint64::new(self.proxy.root(RESULT_SOLD_PRODUCTS))
    }
}

#[derive(Clone)]
pub struct ArrayOfMutableStatisticProduct {
    pub(crate) proxy: Proxy,
}

impl ArrayOfMutableStatisticProduct {

    pub fn append_statistic_product(&self) -> MutableStatisticProduct {
        MutableStatisticProduct { proxy: self.proxy.append() }
    }
    pub fn clear(&self) {
        self.proxy.clear_array();
    }

    pub fn length(&self) -> u32 {
        self.proxy.length()
    }


    pub fn get_statistic_product(&self, index: u32) -> MutableStatisticProduct {
        MutableStatisticProduct { proxy: self.proxy.index(index) }
    }
}

#[derive(Clone)]
pub struct ArrayOfMutableProductTemplate {
    pub(crate) proxy: Proxy,
}

impl ArrayOfMutableProductTemplate {

    pub fn append_product_template(&self) -> MutableProductTemplate {
        MutableProductTemplate { proxy: self.proxy.append() }
    }
    pub fn clear(&self) {
        self.proxy.clear_array();
    }

    pub fn length(&self) -> u32 {
        self.proxy.length()
    }


    pub fn get_product_template(&self, index: u32) -> MutableProductTemplate {
        MutableProductTemplate { proxy: self.proxy.index(index) }
    }
}

#[derive(Clone)]
pub struct MutableCallGetShopStatisticsResults {
    pub proxy: Proxy,
}

impl MutableCallGetShopStatisticsResults {
    pub fn new() -> MutableCallGetShopStatisticsResults {
        MutableCallGetShopStatisticsResults {
            proxy: results_proxy(),
        }
    }

    // earnings of shop
    pub fn earnings(&self) -> ScMutableUint64 {
        ScMutableUint64::new(self.proxy.root(RESULT_EARNINGS))
    }

    // max production step for this shop
    pub fn max_productions_step(&self) -> ScMutableUint64 {
        ScMutableUint64::new(self.proxy.root(RESULT_MAX_PRODUCTIONS_STEP))
    }

    // produced Products for this Shop
    pub fn produced_products(&self) -> ScMutableUint64 {
        ScMutableUint64::new(self.proxy.root(RESULT_PRODUCED_PRODUCTS))
    }

    // all Products that currently are in production
    pub fn production(&self) -> ArrayOfMutableStatisticProduct {
        ArrayOfMutableStatisticProduct { proxy: self.proxy.root(RESULT_PRODUCTION) }
    }

    // all ProductTemplates that exist for this shop
    pub fn production_templates(&self) -> ArrayOfMutableProductTemplate {
        ArrayOfMutableProductTemplate { proxy: self.proxy.root(RESULT_PRODUCTION_TEMPLATES) }
    }

    // sold porducts for this shop
    pub fn sold_products(&self) -> ScMutableUint64 {
        ScMutableUint64::new(self.proxy.root(RESULT_SOLD_PRODUCTS))
    }
}

#[derive(Clone)]
pub struct MapStringToImmutableUint64 {
    pub(crate) proxy: Proxy,
}

impl MapStringToImmutableUint64 {
    pub fn get_uint64(&self, key: &str) -> ScImmutableUint64 {
        ScImmutableUint64::new(self.proxy.key(&string_to_bytes(key)))
    }
}

#[derive(Clone)]
pub struct ImmutableGetEarningsResults {
    pub proxy: Proxy,
}

impl ImmutableGetEarningsResults {
    // earnings per shop
    pub fn earnings_per_shop(&self) -> MapStringToImmutableUint64 {
        MapStringToImmutableUint64 { proxy: self.proxy.root(RESULT_EARNINGS_PER_SHOP) }
    }

    // total earnings of shop
    pub fn total_earnings(&self) -> ScImmutableUint64 {
        ScImmutableUint64::new(self.proxy.root(RESULT_TOTAL_EARNINGS))
    }
}

#[derive(Clone)]
pub struct MapStringToMutableUint64 {
    pub(crate) proxy: Proxy,
}

impl MapStringToMutableUint64 {
    pub fn clear(&self) {
        self.proxy.clear_map();
    }

    pub fn get_uint64(&self, key: &str) -> ScMutableUint64 {
        ScMutableUint64::new(self.proxy.key(&string_to_bytes(key)))
    }
}

#[derive(Clone)]
pub struct MutableGetEarningsResults {
    pub proxy: Proxy,
}

impl MutableGetEarningsResults {
    pub fn new() -> MutableGetEarningsResults {
        MutableGetEarningsResults {
            proxy: results_proxy(),
        }
    }

    // earnings per shop
    pub fn earnings_per_shop(&self) -> MapStringToMutableUint64 {
        MapStringToMutableUint64 { proxy: self.proxy.root(RESULT_EARNINGS_PER_SHOP) }
    }

    // total earnings of shop
    pub fn total_earnings(&self) -> ScMutableUint64 {
        ScMutableUint64::new(self.proxy.root(RESULT_TOTAL_EARNINGS))
    }
}

#[derive(Clone)]
pub struct ImmutableGetMaxProductionStepsResults {
    pub proxy: Proxy,
}

impl ImmutableGetMaxProductionStepsResults {
    // max production steps
    pub fn max_production_steps(&self) -> ScImmutableUint64 {
        ScImmutableUint64::new(self.proxy.root(RESULT_MAX_PRODUCTION_STEPS))
    }
}

#[derive(Clone)]
pub struct MutableGetMaxProductionStepsResults {
    pub proxy: Proxy,
}

impl MutableGetMaxProductionStepsResults {
    pub fn new() -> MutableGetMaxProductionStepsResults {
        MutableGetMaxProductionStepsResults {
            proxy: results_proxy(),
        }
    }

    // max production steps
    pub fn max_production_steps(&self) -> ScMutableUint64 {
        ScMutableUint64::new(self.proxy.root(RESULT_MAX_PRODUCTION_STEPS))
    }
}

#[derive(Clone)]
pub struct ImmutableGetOwnerResults {
    pub proxy: Proxy,
}

impl ImmutableGetOwnerResults {
    // current owner of this smart contract
    pub fn owner(&self) -> ScImmutableAgentID {
        ScImmutableAgentID::new(self.proxy.root(RESULT_OWNER))
    }
}

#[derive(Clone)]
pub struct MutableGetOwnerResults {
    pub proxy: Proxy,
}

impl MutableGetOwnerResults {
    pub fn new() -> MutableGetOwnerResults {
        MutableGetOwnerResults {
            proxy: results_proxy(),
        }
    }

    // current owner of this smart contract
    pub fn owner(&self) -> ScMutableAgentID {
        ScMutableAgentID::new(self.proxy.root(RESULT_OWNER))
    }
}

#[derive(Clone)]
pub struct ImmutableGetProducedProductsResults {
    pub proxy: Proxy,
}

impl ImmutableGetProducedProductsResults {
    // all produced Products per Shop
    pub fn produced_products_per_shop(&self) -> MapStringToImmutableUint64 {
        MapStringToImmutableUint64 { proxy: self.proxy.root(RESULT_PRODUCED_PRODUCTS_PER_SHOP) }
    }

    // amount of all produced Products
    pub fn total_produced_products(&self) -> ScImmutableUint64 {
        ScImmutableUint64::new(self.proxy.root(RESULT_TOTAL_PRODUCED_PRODUCTS))
    }
}

#[derive(Clone)]
pub struct MutableGetProducedProductsResults {
    pub proxy: Proxy,
}

impl MutableGetProducedProductsResults {
    pub fn new() -> MutableGetProducedProductsResults {
        MutableGetProducedProductsResults {
            proxy: results_proxy(),
        }
    }

    // all produced Products per Shop
    pub fn produced_products_per_shop(&self) -> MapStringToMutableUint64 {
        MapStringToMutableUint64 { proxy: self.proxy.root(RESULT_PRODUCED_PRODUCTS_PER_SHOP) }
    }

    // amount of all produced Products
    pub fn total_produced_products(&self) -> ScMutableUint64 {
        ScMutableUint64::new(self.proxy.root(RESULT_TOTAL_PRODUCED_PRODUCTS))
    }
}

#[derive(Clone)]
pub struct ImmutableGetProductsInProductionResults {
    pub proxy: Proxy,
}

impl ImmutableGetProductsInProductionResults {
    // all products in Production
    pub fn products_in_production(&self) -> ArrayOfImmutableStatisticProduct {
        ArrayOfImmutableStatisticProduct { proxy: self.proxy.root(RESULT_PRODUCTS_IN_PRODUCTION) }
    }

    // amount of all products in production
    pub fn total_products_in_production(&self) -> ScImmutableUint64 {
        ScImmutableUint64::new(self.proxy.root(RESULT_TOTAL_PRODUCTS_IN_PRODUCTION))
    }
}

#[derive(Clone)]
pub struct MutableGetProductsInProductionResults {
    pub proxy: Proxy,
}

impl MutableGetProductsInProductionResults {
    pub fn new() -> MutableGetProductsInProductionResults {
        MutableGetProductsInProductionResults {
            proxy: results_proxy(),
        }
    }

    // all products in Production
    pub fn products_in_production(&self) -> ArrayOfMutableStatisticProduct {
        ArrayOfMutableStatisticProduct { proxy: self.proxy.root(RESULT_PRODUCTS_IN_PRODUCTION) }
    }

    // amount of all products in production
    pub fn total_products_in_production(&self) -> ScMutableUint64 {
        ScMutableUint64::new(self.proxy.root(RESULT_TOTAL_PRODUCTS_IN_PRODUCTION))
    }
}

#[derive(Clone)]
pub struct ImmutableGetSoldProductsResults {
    pub proxy: Proxy,
}

impl ImmutableGetSoldProductsResults {
    // all sold Products per Shop
    pub fn sold_products_per_shop(&self) -> MapStringToImmutableUint64 {
        MapStringToImmutableUint64 { proxy: self.proxy.root(RESULT_SOLD_PRODUCTS_PER_SHOP) }
    }

    // all sold Products
    pub fn total_sold_products(&self) -> ScImmutableUint64 {
        ScImmutableUint64::new(self.proxy.root(RESULT_TOTAL_SOLD_PRODUCTS))
    }
}

#[derive(Clone)]
pub struct MutableGetSoldProductsResults {
    pub proxy: Proxy,
}

impl MutableGetSoldProductsResults {
    pub fn new() -> MutableGetSoldProductsResults {
        MutableGetSoldProductsResults {
            proxy: results_proxy(),
        }
    }

    // all sold Products per Shop
    pub fn sold_products_per_shop(&self) -> MapStringToMutableUint64 {
        MapStringToMutableUint64 { proxy: self.proxy.root(RESULT_SOLD_PRODUCTS_PER_SHOP) }
    }

    // all sold Products
    pub fn total_sold_products(&self) -> ScMutableUint64 {
        ScMutableUint64::new(self.proxy.root(RESULT_TOTAL_SOLD_PRODUCTS))
    }
}

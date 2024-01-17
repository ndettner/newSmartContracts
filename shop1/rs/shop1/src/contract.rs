// Code generated by schema tool; DO NOT EDIT.

#![allow(dead_code)]

use wasmlib::*;

use crate::*;

pub struct CallCheckProductCall<'a> {
    pub func:    ScFunc<'a>,
    pub params:  MutableCallCheckProductParams,
    pub results: ImmutableCallCheckProductResults,
}

pub struct CallCreateNewProductCall<'a> {
    pub func:   ScFunc<'a>,
    pub params: MutableCallCreateNewProductParams,
}

pub struct CallPayStoreCall<'a> {
    pub func:   ScFunc<'a>,
    pub params: MutableCallPayStoreParams,
}

pub struct CallPingShopCall<'a> {
    pub func:    ScFunc<'a>,
    pub results: ImmutableCallPingShopResults,
}

pub struct CallRecieveProductsCall<'a> {
    pub func:    ScFunc<'a>,
    pub results: ImmutableCallRecieveProductsResults,
}

pub struct CallSendProductCall<'a> {
    pub func: ScFunc<'a>,
}

pub struct InitCall<'a> {
    pub func:   ScInitFunc<'a>,
    pub params: MutableInitParams,
}

pub struct ProduceCall<'a> {
    pub func: ScFunc<'a>,
}

pub struct SetOwnerCall<'a> {
    pub func:   ScFunc<'a>,
    pub params: MutableSetOwnerParams,
}

pub struct StartProductionCall<'a> {
    pub func: ScFunc<'a>,
}

pub struct CallGetShopStatisticsCall<'a> {
    pub func:    ScView<'a>,
    pub params:  MutableCallGetShopStatisticsParams,
    pub results: ImmutableCallGetShopStatisticsResults,
}

pub struct GetEarningsCall<'a> {
    pub func:    ScView<'a>,
    pub results: ImmutableGetEarningsResults,
}

pub struct GetMaxProductionStepsCall<'a> {
    pub func:    ScView<'a>,
    pub results: ImmutableGetMaxProductionStepsResults,
}

pub struct GetOwnerCall<'a> {
    pub func:    ScView<'a>,
    pub results: ImmutableGetOwnerResults,
}

pub struct GetProducedProductsCall<'a> {
    pub func:    ScView<'a>,
    pub results: ImmutableGetProducedProductsResults,
}

pub struct GetProductsInProductionCall<'a> {
    pub func:    ScView<'a>,
    pub results: ImmutableGetProductsInProductionResults,
}

pub struct GetSoldProductsCall<'a> {
    pub func:    ScView<'a>,
    pub results: ImmutableGetSoldProductsResults,
}

pub struct ScFuncs {
}

impl ScFuncs {
    pub fn call_check_product(ctx: &impl ScFuncClientContext) -> CallCheckProductCall {
        let mut f = CallCheckProductCall {
            func:    ScFunc::new(ctx, HSC_NAME, HFUNC_CALL_CHECK_PRODUCT),
            params:  MutableCallCheckProductParams { proxy: Proxy::nil() },
            results: ImmutableCallCheckProductResults { proxy: Proxy::nil() },
        };
        ScFunc::link_params(&mut f.params.proxy, &f.func);
        ScFunc::link_results(&mut f.results.proxy, &f.func);
        f
    }

    pub fn call_create_new_product(ctx: &impl ScFuncClientContext) -> CallCreateNewProductCall {
        let mut f = CallCreateNewProductCall {
            func:    ScFunc::new(ctx, HSC_NAME, HFUNC_CALL_CREATE_NEW_PRODUCT),
            params:  MutableCallCreateNewProductParams { proxy: Proxy::nil() },
        };
        ScFunc::link_params(&mut f.params.proxy, &f.func);
        f
    }

    pub fn call_pay_store(ctx: &impl ScFuncClientContext) -> CallPayStoreCall {
        let mut f = CallPayStoreCall {
            func:    ScFunc::new(ctx, HSC_NAME, HFUNC_CALL_PAY_STORE),
            params:  MutableCallPayStoreParams { proxy: Proxy::nil() },
        };
        ScFunc::link_params(&mut f.params.proxy, &f.func);
        f
    }

    pub fn call_ping_shop(ctx: &impl ScFuncClientContext) -> CallPingShopCall {
        let mut f = CallPingShopCall {
            func:    ScFunc::new(ctx, HSC_NAME, HFUNC_CALL_PING_SHOP),
            results: ImmutableCallPingShopResults { proxy: Proxy::nil() },
        };
        ScFunc::link_results(&mut f.results.proxy, &f.func);
        f
    }

    pub fn call_recieve_products(ctx: &impl ScFuncClientContext) -> CallRecieveProductsCall {
        let mut f = CallRecieveProductsCall {
            func:    ScFunc::new(ctx, HSC_NAME, HFUNC_CALL_RECIEVE_PRODUCTS),
            results: ImmutableCallRecieveProductsResults { proxy: Proxy::nil() },
        };
        ScFunc::link_results(&mut f.results.proxy, &f.func);
        f
    }

    pub fn call_send_product(ctx: &impl ScFuncClientContext) -> CallSendProductCall {
        CallSendProductCall {
            func: ScFunc::new(ctx, HSC_NAME, HFUNC_CALL_SEND_PRODUCT),
        }
    }

    pub fn init(ctx: &impl ScFuncClientContext) -> InitCall {
        let mut f = InitCall {
            func:    ScInitFunc::new(ctx, HSC_NAME, HFUNC_INIT),
            params:  MutableInitParams { proxy: Proxy::nil() },
        };
        ScInitFunc::link_params(&mut f.params.proxy, &f.func);
        f
    }

    pub fn produce(ctx: &impl ScFuncClientContext) -> ProduceCall {
        ProduceCall {
            func: ScFunc::new(ctx, HSC_NAME, HFUNC_PRODUCE),
        }
    }

    pub fn set_owner(ctx: &impl ScFuncClientContext) -> SetOwnerCall {
        let mut f = SetOwnerCall {
            func:    ScFunc::new(ctx, HSC_NAME, HFUNC_SET_OWNER),
            params:  MutableSetOwnerParams { proxy: Proxy::nil() },
        };
        ScFunc::link_params(&mut f.params.proxy, &f.func);
        f
    }

    pub fn start_production(ctx: &impl ScFuncClientContext) -> StartProductionCall {
        StartProductionCall {
            func: ScFunc::new(ctx, HSC_NAME, HFUNC_START_PRODUCTION),
        }
    }

    pub fn call_get_shop_statistics(ctx: &impl ScViewClientContext) -> CallGetShopStatisticsCall {
        let mut f = CallGetShopStatisticsCall {
            func:    ScView::new(ctx, HSC_NAME, HVIEW_CALL_GET_SHOP_STATISTICS),
            params:  MutableCallGetShopStatisticsParams { proxy: Proxy::nil() },
            results: ImmutableCallGetShopStatisticsResults { proxy: Proxy::nil() },
        };
        ScView::link_params(&mut f.params.proxy, &f.func);
        ScView::link_results(&mut f.results.proxy, &f.func);
        f
    }

    pub fn get_earnings(ctx: &impl ScViewClientContext) -> GetEarningsCall {
        let mut f = GetEarningsCall {
            func:    ScView::new(ctx, HSC_NAME, HVIEW_GET_EARNINGS),
            results: ImmutableGetEarningsResults { proxy: Proxy::nil() },
        };
        ScView::link_results(&mut f.results.proxy, &f.func);
        f
    }

    pub fn get_max_production_steps(ctx: &impl ScViewClientContext) -> GetMaxProductionStepsCall {
        let mut f = GetMaxProductionStepsCall {
            func:    ScView::new(ctx, HSC_NAME, HVIEW_GET_MAX_PRODUCTION_STEPS),
            results: ImmutableGetMaxProductionStepsResults { proxy: Proxy::nil() },
        };
        ScView::link_results(&mut f.results.proxy, &f.func);
        f
    }

    pub fn get_owner(ctx: &impl ScViewClientContext) -> GetOwnerCall {
        let mut f = GetOwnerCall {
            func:    ScView::new(ctx, HSC_NAME, HVIEW_GET_OWNER),
            results: ImmutableGetOwnerResults { proxy: Proxy::nil() },
        };
        ScView::link_results(&mut f.results.proxy, &f.func);
        f
    }

    pub fn get_produced_products(ctx: &impl ScViewClientContext) -> GetProducedProductsCall {
        let mut f = GetProducedProductsCall {
            func:    ScView::new(ctx, HSC_NAME, HVIEW_GET_PRODUCED_PRODUCTS),
            results: ImmutableGetProducedProductsResults { proxy: Proxy::nil() },
        };
        ScView::link_results(&mut f.results.proxy, &f.func);
        f
    }

    pub fn get_products_in_production(ctx: &impl ScViewClientContext) -> GetProductsInProductionCall {
        let mut f = GetProductsInProductionCall {
            func:    ScView::new(ctx, HSC_NAME, HVIEW_GET_PRODUCTS_IN_PRODUCTION),
            results: ImmutableGetProductsInProductionResults { proxy: Proxy::nil() },
        };
        ScView::link_results(&mut f.results.proxy, &f.func);
        f
    }

    pub fn get_sold_products(ctx: &impl ScViewClientContext) -> GetSoldProductsCall {
        let mut f = GetSoldProductsCall {
            func:    ScView::new(ctx, HSC_NAME, HVIEW_GET_SOLD_PRODUCTS),
            results: ImmutableGetSoldProductsResults { proxy: Proxy::nil() },
        };
        ScView::link_results(&mut f.results.proxy, &f.func);
        f
    }
}

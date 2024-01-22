use shop1::ScFuncs;
use wasmlib::*;

use crate::*;
const MAX_PROCESSING_STEPS: u64 = 3;

pub fn func_call_check_product(ctx: &ScFuncContext, f: &CallCheckProductContext) {}

pub fn func_call_create_new_product(ctx: &ScFuncContext, f: &CallCreateNewProductContext) {
    let musician_name = f.params.musician_name();
    ctx.require(
        musician_name.exists(),
        "No Musician name given in Parameters",
    );

    let price = f.params.price();
    ctx.require(price.exists(), "No price was given in Parameters");

    let product_type = f.params.product_type();
    ctx.require(product_type.exists(), "No product type was given");

    let shop_name = f.params.shop_name();
    ctx.require(shop_name.exists(), "No Shopname was given");

    ctx.require(
        f.state.festival_hname().exists(),
        "No festivale destination was set",
    );

    let new_product_template = shop1::ProductTemplate {
        musician: musician_name.value(),
        price: price.value(),
        product_type: product_type.value(),
        shop_name: shop_name.value(),
    };

    // check to Russfest SC if I can create a matching product
    let func = ScFuncs::call_check_product(ctx);

    func.params.product().set_value(&new_product_template);

    func.func
        .of_contract(f.state.festival_hname().value())
        .call();

    ctx.require(
        func.results.can_produce().value(),
        &func.results.message().value(),
    );

    // if russfest sents OK, save template to state

    if func.results.can_produce().value() {
        f.state
            .product_templates()
            .append_product_template()
            .set_value(&new_product_template);

        // if shop name is new save name to state, initialize earnings and produced products to 0
        let mut shop_is_new = true;
        for i in 0..f.state.shop_names().length() {
            if shop_name
                .value()
                .eq(&f.state.shop_names().get_string(i).value())
            {
                shop_is_new = false;
            }
        }

        if shop_is_new {
            f.state
                .shop_names()
                .append_string()
                .set_value(&shop_name.value());

            f.state
                .produced_products()
                .get_uint64(&shop_name.value())
                .set_value(0);

            f.state
                .earned_money()
                .get_uint64(&shop_name.value())
                .set_value(0);
            f.state
                .sold_products()
                .get_uint64(&shop_name.value())
                .set_value(0);
        }
    }

    // get id and update
    let id = f.state.product_id().value();
    f.state.product_id().set_value(id + 1);

    // create new product and save to production
    let new_product = shop1::StatisticProduct {
        id: id,
        musician: new_product_template.musician,
        price: new_product_template.price,
        product_type: new_product_template.product_type,
        production_step: 0,
        shop_name: new_product_template.shop_name,
        timestamp: ctx.timestamp(),
    };

    f.state
        .producing_products()
        .append_statistic_product()
        .set_value(&new_product);
}

pub fn func_call_pay_store(ctx: &ScFuncContext, f: &CallPayStoreContext) {
    let shop_name = f.params.shop_name();
    ctx.require(shop_name.exists(), "No shop name was given in Parameters");

    let incoming_money = ctx.allowance().base_tokens();
    ctx.log("INCOMING MONEY");
    ctx.log(&incoming_money.to_string());

    let shop_money = f.state.earned_money().get_uint64(&shop_name.value());
    ctx.require(shop_money.exists(), "No earning for this shop");

    // set total earned money
    f.state
        .total_earned_money()
        .set_value(f.state.total_earned_money().value() + incoming_money);

    // set earned money for shop
    f.state
        .earned_money()
        .get_uint64(&shop_name.value())
        .set_value(
            incoming_money
                + f.state
                    .earned_money()
                    .get_uint64(&shop_name.value())
                    .value(),
        );

    // set total sold products
    f.state
        .total_sold_products()
        .set_value(f.state.total_sold_products().value() + 1);

    // set sold products for shop
    f.state
        .sold_products()
        .get_uint64(&shop_name.value())
        .set_value(
            1 + f
                .state
                .sold_products()
                .get_uint64(&shop_name.value())
                .value(),
        );
}

pub fn func_call_ping_shop(ctx: &ScFuncContext, f: &CallPingShopContext) {
    let festival_hname = ctx.caller().hname();

    f.state.festival_hname().set_value(festival_hname);

    f.results.ping_successful().set_value(true);
}

pub fn func_call_recieve_products(ctx: &ScFuncContext, f: &CallRecieveProductsContext) {
    for i in 0..f.state.finished_products().length() {
        let statistic_product = f.state.finished_products().get_product(i);
        if statistic_product.exists() {
            f.results
                .product()
                .append_product()
                .set_value(&statistic_product.value());
        }
    }
    f.state.finished_products().clear();
}

pub fn func_call_send_product(ctx: &ScFuncContext, f: &CallSendProductContext) {
    let func = ScFuncs::call_send_product(ctx);
    func.func
        .of_contract(f.state.festival_hname().value())
        .call();
}

pub fn func_init(ctx: &ScFuncContext, f: &InitContext) {
    if f.params.owner().exists() {
        f.state.owner().set_value(&f.params.owner().value());
        return;
    }
    f.state.owner().set_value(&ctx.request_sender());

    f.state.total_earned_money().set_value(0);
    f.state.product_id().set_value(0);
    f.state.total_sold_products().set_value(0);
}

pub fn func_produce(ctx: &ScFuncContext, f: &ProduceContext) {
    let mut new_producing_array: Vec<shop1::StatisticProduct> = Vec::new();
    let mut new_finished_products_array: Vec<shop1::StatisticProduct> = Vec::new();

    // check all products in production. If product is at max steps it is finished
    // save all further unfinished and finished products in different arrays
    for i in 0..f.state.producing_products().length() {
        let product_proxy = f.state.producing_products().get_statistic_product(i);
        if product_proxy.exists() {
            let mut product = product_proxy.value();
            let new_step = product.production_step + 1;
            // set new production step and timestamp
            product.production_step = new_step;
            product.timestamp = ctx.timestamp();
            if product.production_step <= MAX_PROCESSING_STEPS {
                new_producing_array.push(product);
            } else {
                new_finished_products_array.push(product);
            }
        }
    }

    // clear state of producing products and reenter all unfinished products
    for i in 0..f.state.producing_products().length() {
        f.state
            .producing_products()
            .get_statistic_product(i)
            .delete();
    }
    f.state.producing_products().clear();

    for i in 0..new_producing_array.len() {
        f.state
            .producing_products().append_statistic_product()
            .set_value(&new_producing_array[i]);
    }

    ctx.log("Finished new Producing Products State array");

    // save all finished products#
    for i in 0..new_finished_products_array.len() {
        let mut is_new = true;
        let finished_product = &new_finished_products_array[i];
        for j in 0..f.state.finished_products().length() {
            let state_finished_product_proxy = f.state.finished_products().get_product(j);
            if state_finished_product_proxy.exists() {
                let mut state_finished_product = state_finished_product_proxy.value();
                if finished_product
                    .musician
                    .eq(&state_finished_product.musician)
                    && finished_product
                        .product_type
                        .eq(&state_finished_product.product_type)
                {
                    is_new = false;
                    state_finished_product.stock = state_finished_product.stock + 1;
                    f.state
                        .finished_products()
                        .get_product(j)
                        .set_value(&state_finished_product);
                    break;
                }
            }
        }

        if is_new {
            let new_product = shop1::Product {
                musician: finished_product.musician.clone(),
                product_type: finished_product.product_type.clone(),
                price: finished_product.price,
                shop_name: finished_product.shop_name.clone(),
                stock: 1,
            };
            f.state
                .finished_products()
                .append_product()
                .set_value(&new_product);
        }

        // account for finished products
        f.state
            .produced_products()
            .get_uint64(&finished_product.shop_name)
            .set_value(
                f.state
                    .produced_products()
                    .get_uint64(&finished_product.shop_name)
                    .value()
                    + 1,
            );

        f.state
            .total_produced_products()
            .set_value(f.state.total_produced_products().value() + 1);
    }

    ctx.log("Saved all finished products and accounted");

    // produce new product for every template available
    for i in 0..f.state.product_templates().length() {
        let product_template = f.state.product_templates().get_product_template(i).value();
        let product_id = f.state.product_id().value();
        let producing_product = shop1::StatisticProduct {
            id: product_id,
            musician: product_template.musician,
            price: product_template.price,
            product_type: product_template.product_type,
            shop_name: product_template.shop_name,
            production_step: 0,
            timestamp: ctx.timestamp(),
        };

        f.state.product_id().set_value(product_id + 1);

        f.state
            .producing_products()
            .append_statistic_product()
            .set_value(&producing_product);
    }

    // send products to festival
    let func = ScFuncs::call_send_product(ctx);
    func.func.call();
    /* TODO add in production?
    let f = contract::ScFuncs::produce(ctx);
    let seconds = ctx.random(100) as i32;
    f.func.transfer_iotas(1).delay(seconds).post();
    */
}

pub fn func_set_owner(ctx: &ScFuncContext, f: &SetOwnerContext) {
    f.state.owner().set_value(&f.params.owner().value());
}

pub fn func_start_production(ctx: &ScFuncContext, f: &StartProductionContext) {
    let f = ScFuncs::produce(ctx);
    f.func.transfer_base_tokens(1).post();
}

pub fn view_call_get_shop_statistics(ctx: &ScViewContext, f: &CallGetShopStatisticsContext) {
    let shop_name_proxy = f.params.shop_name();
    ctx.require(shop_name_proxy.exists(), "No shopName was given in params");

    let shop_name = shop_name_proxy.value();

    // earnings
    f.results
        .earnings()
        .set_value(f.state.earned_money().get_uint64(&shop_name).value());

    // max processing steps
    f.results
        .max_productions_step()
        .set_value(MAX_PROCESSING_STEPS);

    // number of produced products
    f.results
        .produced_products()
        .set_value(f.state.produced_products().get_uint64(&shop_name).value());

    // number of sold products
    f.results
        .sold_products()
        .set_value(f.state.sold_products().get_uint64(&shop_name).value());

    // product in production belonging to the shop
    for i in 0..f.state.producing_products().length() {
        let product_proxy = f.state.producing_products().get_statistic_product(i);
        if product_proxy.exists() {
            let product = product_proxy.value();
            if product.shop_name.eq(&shop_name) {
                let statistic_product = shop1::StatisticProduct {
                    id: product.id,
                    musician: product.musician,
                    price: product.price,
                    product_type: product.product_type,
                    shop_name: product.shop_name,
                    production_step: product.production_step,
                    timestamp: product.timestamp,
                };
                f.results
                    .production()
                    .append_statistic_product()
                    .set_value(&statistic_product);
            }
        }
    }

    // templates belonging to the shop
    for i in 0..f.state.product_templates().length() {
        let template_proxy = f.state.product_templates().get_product_template(i);
        if template_proxy.exists() {
            let template = template_proxy.value();
            if template.shop_name.eq(&shop_name) {
                f.results
                    .production_templates()
                    .append_product_template()
                    .set_value(&template);
            }
        }
    }
}

pub fn view_get_earnings(ctx: &ScViewContext, f: &GetEarningsContext) {
    f.results
        .total_earnings()
        .set_value(f.state.total_earned_money().value());

    for i in 0..f.state.shop_names().length() {
        let shop_name = f.state.shop_names().get_string(i).value();
        f.results
            .earnings_per_shop()
            .get_uint64(&shop_name)
            .set_value(f.state.earned_money().get_uint64(&shop_name).value());
    }
}

pub fn view_get_max_production_steps(ctx: &ScViewContext, f: &GetMaxProductionStepsContext) {
    f.results
        .max_production_steps()
        .set_value(MAX_PROCESSING_STEPS);
}

pub fn view_get_owner(ctx: &ScViewContext, f: &GetOwnerContext) {
    f.results.owner().set_value(&f.state.owner().value());
}

pub fn view_get_produced_products(ctx: &ScViewContext, f: &GetProducedProductsContext) {
    f.results
        .total_produced_products()
        .set_value(f.state.total_produced_products().value());

    for i in 0..f.state.shop_names().length() {
        let shop_name = f.state.shop_names().get_string(i).value();

        f.results
            .produced_products_per_shop()
            .get_uint64(&shop_name)
            .set_value(f.state.produced_products().get_uint64(&shop_name).value());
    }
}

pub fn view_get_products_in_production(ctx: &ScViewContext, f: &GetProductsInProductionContext) {
    for i in 0..f.state.producing_products().length() {
        let product_in_production = f.state.producing_products().get_statistic_product(i);
        if product_in_production.exists() {
            f.results
                .products_in_production()
                .append_statistic_product()
                .set_value(
                    &f.state
                        .producing_products()
                        .get_statistic_product(i)
                        .value(),
                );
        }
    }

    let mut count: u64 = 0;

    for i in 0..f.state.producing_products().length() {
        if f.state
            .producing_products()
            .get_statistic_product(i)
            .exists()
        {
            count = count + 1;
        }
    }
    f.results.total_products_in_production().set_value(count);
}

pub fn view_get_sold_products(ctx: &ScViewContext, f: &GetSoldProductsContext) {
    f.results
        .total_sold_products()
        .set_value(f.state.total_sold_products().value());

    for i in 0..f.state.shop_names().length() {
        let shop_name = f.state.shop_names().get_string(i).value();
        f.results
            .sold_products_per_shop()
            .get_uint64(&shop_name)
            .set_value(f.state.sold_products().get_uint64(&shop_name).value());
    }
}

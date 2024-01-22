use std::collections::HashMap;
use russfest::ScFuncs;
use wasmlib::*;

use crate::*;

// states for shop request
const TRUE: &str = "TRUE";
const REQUESTING: &str = "REQUESTING";
const DENIED: &str = "DENIED";
const DELETED: &str = "DELETED";

pub fn func_accept_shop(ctx: &ScFuncContext, f: &AcceptShopContext) {
    let shop_name = f.params.shop_name().value();
    let shop_proxy: russfest::MutableShop = f.state.shops().get_shop(&shop_name);

    if !shop_proxy.exists() {
        f.state
            .error_messages()
            .get_string(&&ctx.request_id().to_string())
            .set_value("Shop does not exist, could'nt accept");
        return;
    }

    let shop = shop_proxy.value();

    let mut new_shop = russfest::Shop {
        shop_name: shop.shop_name,
        musician_name: shop.musician_name,
        fee: shop.fee,
        shop_owner: shop.shop_owner,
        is_registered: shop.is_registered,
        shop_hname: shop.shop_hname,
    };

    // link shop to musician if no shop is associated
    for i in 0..f.state.musicians().length() {
        let musician = f.state.musicians().get_musician(i);
        if musician.value().name.eq(&new_shop.musician_name) {
            let mut updated_musician: russfest::Musician = musician.value();
            if updated_musician.shop.is_empty() {
                updated_musician.shop = new_shop.shop_name.clone();
                f.state
                    .musicians()
                    .get_musician(i)
                    .set_value(&updated_musician);

                new_shop.is_registered = String::from(TRUE);
                f.state.shops().get_shop(&shop_name).set_value(&new_shop);
            } else {
                new_shop.is_registered = String::from(DENIED);
                f.state.shops().get_shop(&shop_name).set_value(&new_shop);
            }
        }
    }
}

pub fn func_add_musician(ctx: &ScFuncContext, f: &AddMusicianContext) {
    let _musician_name = f.params.name().value();

    let shop = f.params.shop();
    let mut _shop_name: String = String::new();

    if shop.exists() {
        _shop_name = shop.value().to_string();
    }

    let musician = russfest::Musician {
        name: _musician_name,
        shop: _shop_name,
    };

    let mut musician_unique: bool = true;

    for i in 0..f.state.musicians().length() {
        if musician
            .name
            .eq(&f.state.musicians().get_musician(i).value().name)
        {
            musician_unique = false;
        }
    }

    if !musician_unique {
        f.state
            .error_messages()
            .get_string(&ctx.request_id().to_string())
            .set_value("Musician already exists");
        return;
    }

    f.state.musicians().append_musician().set_value(&musician);
}

pub fn func_buy_merch(ctx: &ScFuncContext, f: &BuyMerchContext) {
    let musician_name = f.params.musician();
    ctx.require(
        musician_name.exists(),
        "No Musician Name was given in parameters",
    );

    let product_type = f.params.product_type();
    ctx.require(
        product_type.exists(),
        "No product type was given in parameters",
    );

    let shop = f.state.shops().get_shop(&f.params.shop_name().value());

    if !shop.exists() {
        f.state
            .error_messages()
            .get_string(&ctx.request_id().to_string())
            .set_value("No shop with this name exists");
        return;
    }

    let mut index: u32 = 0;
    let mut product_found = false;

    for i in 0..f.state.products().length() {
        let current_product = f.state.products().get_product(i);
        if current_product.value().musician.eq(&musician_name.value())
            && current_product
                .value()
                .product_type
                .eq(&product_type.value())
        {
            index = i;
            product_found = true;
            break;
        }
    }

    if !product_found {
        f.state
            .error_messages()
            .get_string(&ctx.request_id().to_string())
            .set_value("No matching product found");
        return;
    }

    let mut product: russfest::Product = f.state.products().get_product(index).value();

    // Product has to have at least a stock of 1
    if product.stock == 0 {
        f.state
            .error_messages()
            .get_string(&ctx.request_id().to_string())
            .set_value("Out of stock");
        return;
    }

    // paid IOTAS have to be the same amount as price of product
    let incoming_balance: u64 = ctx.allowance().base_tokens();
    ctx.require(
        incoming_balance == product.price,
        "Paid IOTAS don't match product price",
    );

    // reduce stock of product by 1
    product.stock = product.stock - 1;
    f.state.products().get_product(index).set_value(&product);

    // only a portion will be sent to the shop. Fees will be collected by Russfest SC
    let shop_amount = (incoming_balance * (100 - shop.value().fee)) / 100;

    ctx.log("SHOP AMOUNT");
    ctx.log(&shop_amount.to_string());
    // sent money to shop
    let s_c_address: ScHname = shop.value().shop_hname;

    let func = ScFuncs::call_pay_store(ctx);
    func.params.shop_name().set_value(&shop.value().shop_name);
    func.func
        .transfer_base_tokens(shop_amount)
        .of_contract(s_c_address)
        .post();

    // save earned money
    f.state
        .earned_money()
        .set_value(incoming_balance - shop_amount + f.state.earned_money().value());
}

pub fn func_call_check_product(ctx: &ScFuncContext, f: &CallCheckProductContext) {
    let caller = ctx.caller();

    // check that caller is a SC
    ctx.require(!caller.is_address(), "Only SCs can call this function");

    // check that all parameters exist

    let product_template = f.params.product();
    ctx.require(
        product_template.exists(),
        "No Product Template was given in Paramters",
    );

    let musician_name = product_template.value().musician;
    ctx.require(
        !musician_name.is_empty(),
        "Product Template not complete missing Musician",
    );

    let product_type = product_template.value().product_type;
    ctx.require(
        !product_type.is_empty(),
        "Product Template not complete missing Product Type",
    );

    let shop_name = product_template.value().shop_name;
    ctx.require(
        !shop_name.is_empty(),
        "Product Template not complete missing Shop Name",
    );

    let product_price = product_template.value().price;
    ctx.require(
        product_price > 0,
        "Product Template not complete price lesser than 1",
    );

    // get associated shop and check if caller is allowed to create products for the requested musician
    let shop = f.state.shops().get_shop(&shop_name);
    ctx.require(shop.exists(), "No shop with this name exists");

    ctx.require(
        shop.value().shop_hname == caller.hname(),
        "Caller is not associated with this shop",
    );

    ctx.require(
        shop.value().is_registered.eq(&String::from(TRUE)),
        "Shop is not registered",
    );

    ctx.require(
        shop.value().musician_name.eq(&musician_name),
        "Shop can't sell merch for this musician",
    );

    // check if Festival already know this type of product

    let mut is_new = true;

    for i in 0..f.state.products().length() {
        let product = f.state.products().get_product(i).value();
        if product.musician.eq(&musician_name) && product.product_type.eq(&product_type) {
            is_new = false;
        }
    }

    // send answer
    f.results.can_produce().set_value(is_new);

    if !is_new {
        f.results.message().set_value("Product already exists");
    }

    if is_new {
        // create new product template and save product to state
        let new_product = russfest::Product {
            musician: musician_name,
            price: product_price,
            product_type: product_type,
            stock: 0,
            shop_name: shop_name,
        };

        f.state.products().append_product().set_value(&new_product);
    }
}

pub fn func_call_create_new_product(ctx: &ScFuncContext, f: &CallCreateNewProductContext) {
    let musician_name = f.params.musician_name().value();
    let shop_name = f.params.shop_name().value();
    let price = f.params.price().value();
    let product_type = f.params.product_type().value();

    let shop = f.state.shops().get_shop(&shop_name);

    if !shop.exists() {
        f.state
            .error_messages()
            .get_string(&ctx.request_id().to_string())
            .set_value("A shop with this name doesn't exist");
        return;
    }

    if shop.value().shop_owner != ctx.caller() {
        f.state
            .error_messages()
            .get_string(&ctx.request_id().to_string())
            .set_value("Caller is not Owner of Shop");
        return;
    }

    let func = ScFuncs::call_create_new_product(ctx);
    func.params.musician_name().set_value(&musician_name);
    func.params.price().set_value(price);
    func.params.product_type().set_value(&product_type);
    func.params.shop_name().set_value(&shop_name);
    func.func.of_contract(shop.value().shop_hname).call();
}

pub fn func_call_pay_store(ctx: &ScFuncContext, f: &CallPayStoreContext) {
    // TODO
}

pub fn func_call_ping_shop(ctx: &ScFuncContext, f: &CallPingShopContext) {
    // TODO
}

pub fn func_call_recieve_products(ctx: &ScFuncContext, f: &CallRecieveProductsContext) {
    // TODO
}

pub fn func_call_send_product(ctx: &ScFuncContext, f: &CallSendProductContext) {
    let func = ScFuncs::call_recieve_products(ctx);
    func.func.of_contract(ctx.caller().hname()).call();

    for i in 0..func.results.product().length() {
        let recieved_product: russfest::Product = func.results.product().get_product(i).value();
        let mut is_new = true;
        for j in 0..f.state.products().length() {
            let mut product = f.state.products().get_product(j).value();
            if recieved_product.musician.eq(&product.musician)
                && recieved_product.product_type.eq(&product.product_type)
            {
                product.stock = product.stock + recieved_product.stock;
                f.state.products().get_product(j).set_value(&product);
                is_new = false
            }
        }
        if is_new {
            f.state
                .products()
                .append_product()
                .set_value(&recieved_product);
        }
    }
}

pub fn func_cancel_shop_request(ctx: &ScFuncContext, f: &CancelShopRequestContext) {
    let shop_name = f.params.name();
    let shop_owner = ctx.caller();

    let shop = f.state.shops().get_shop(&shop_name.value());

    // require that a shop with this name exists
    ctx.require(shop.exists(), "No Shop with this Name exists");

    // require that the shop has the same owner as the caller
    if shop
        .value()
        .shop_owner
        .to_string()
        .ne(&shop_owner.to_string())
    {
        f.state
            .error_messages()
            .get_string(&ctx.request_id().to_string())
            .set_value("Caller is not Owner of Shop");
        return;
    }

    let mut new_shop_map: HashMap<String, russfest::Shop> = HashMap::new();
    let mut new_shop_name_array: Vec<String> = Vec::new();

    // Make a copy from shopNamesArray & ShopsMap
    for i in 0..f.state.shopnames().length() {
        let current_shop_name = f.state.shopnames().get_string(i).value();

        if !current_shop_name.eq(&shop_name.value()) {
            new_shop_name_array.push(current_shop_name.clone());
            let shop_copy: russfest::Shop = f.state.shops().get_shop(&current_shop_name).value();
            new_shop_map.insert(current_shop_name, shop_copy);
        }
    }
    // clear state
    f.state.shopnames().clear();
    // TODO As of discord answer from  clear doesn't work on map level.
    // Therefore a workaround is needed
    f.state.shops().clear();

    let mut shop_to_be_deleted: russfest::Shop =
        f.state.shops().get_shop(&shop_name.value()).value();
    shop_to_be_deleted.is_registered = String::from(DELETED);
    f.state
        .shops()
        .get_shop(&shop_name.value())
        .set_value(&shop_to_be_deleted);

    ctx.log("Before Re Enter Shop NAMES");

    // re-enter shopNames without canceled shop
    for i in 0..new_shop_name_array.len() {
        let insert_shop_name = new_shop_name_array[i].clone();
        f.state
            .shopnames()
            .append_string()
            .set_value(&insert_shop_name);
    }

    for (shop_name_map, shop) in &new_shop_map {
        f.state.shops().get_shop(&shop_name_map).set_value(&shop);
    }

    // re-enter ShopMap without canceled shop
    /* This does not work, because at current version state map can't be cleared
    for (shop_name_map, shop) in &new_shop_map {
        f.state.shops().get_shop(&shop_name_map).set_value(&shop);
    } */
}

pub fn func_deny_shop(ctx: &ScFuncContext, f: &DenyShopContext) {
    let shop_name = f.params.shop_name().value();
    let shop_proxy: russfest::MutableShop = f.state.shops().get_shop(&shop_name);

    if !shop_proxy.exists() {
        f.state
            .error_messages()
            .get_string(&ctx.request_id().to_string())
            .set_value("Shop does not exist");
        return;
    }

    if !shop_proxy.value().is_registered.eq(REQUESTING) {
        f.state
            .error_messages()
            .get_string(&ctx.request_id().to_string())
            .set_value("Shop is already registered");
        return;
    }

    let shop = shop_proxy.value();
    let new_shop = russfest::Shop {
        shop_name: shop.shop_name,
        musician_name: shop.musician_name,
        fee: shop.fee,
        shop_owner: shop.shop_owner,
        is_registered: String::from(DENIED),
        shop_hname: shop.shop_hname,
    };

    // set shop to denied, after that shopOwner can make changes
    shop_proxy.delete();

    f.state.shops().get_shop(&shop_name).set_value(&new_shop);
}

pub fn func_init(ctx: &ScFuncContext, f: &InitContext) {
    if f.params.owner().exists() {
        f.state.owner().set_value(&f.params.owner().value());
        return;
    }
    f.state.owner().set_value(&ctx.request_sender());
}

pub fn func_request_shop_licence(ctx: &ScFuncContext, f: &RequestShopLicenceContext) {
    let shop = russfest::Shop {
        shop_name: f.params.name().value(),
        musician_name: f.params.musician_name().value(),
        fee: f.params.fee().value(),
        shop_owner: ctx.caller(),
        is_registered: String::from(REQUESTING),
        shop_hname: f.params.shop_hname().value(),
    };

    // check if shopSC can be pinged
    let func = ScFuncs::call_ping_shop(ctx);
    func.func.of_contract(shop.shop_hname).call();

    if !func.results.ping_successful().value() {
        f.state
            .error_messages()
            .get_string(&ctx.request_id().to_string())
            .set_value("Could not ping shop");
        return;
    }

    let musicians: russfest::ArrayOfMutableMusician = f.state.musicians();

    // map can't be cleared therefore duplicates are checked within key-array
    for i in 0..f.state.shopnames().length() {
        ctx.require(
            !shop
                .shop_name
                .eq(&f.state.shopnames().get_string(i).value()),
            "Shop already exists",
        )
    }
    /* ctx.require(
           !f.state.shops().get_shop(&shop.shop_name).exists(),
           "Shop already exists",
       );
    */

    let mut musician_exists: bool = false;

    // a musician can only have one shop
    for i in 0..musicians.length() {
        let musician = musicians.get_musician(i);
        if musician.value().name.eq(&shop.musician_name) {
            musician_exists = true;
            if !musician.value().shop.is_empty() {
                f.state
                    .error_messages()
                    .get_string(&ctx.request_id().to_string())
                    .set_value("Musician already has an Shop associated");
                return;
            }
        }
    }

    if !musician_exists {
        f.state
            .error_messages()
            .get_string(&ctx.request_id().to_string())
            .set_value("No musician exists");
        return;
    }

    // add shop to map
    f.state.shops().get_shop(&shop.shop_name).set_value(&shop);

    // add Shopname to array
    f.state
        .shopnames()
        .append_string()
        .set_value(&shop.shop_name);
}

pub fn func_set_owner(ctx: &ScFuncContext, f: &SetOwnerContext) {
    f.state.owner().set_value(&f.params.new_owner().value());
}

pub fn func_update_denied_shop_request(ctx: &ScFuncContext, f: &UpdateDeniedShopRequestContext) {
    let shop_name = f.params.shop_name();
    let fee = f.params.newfee();
    let new_hname = f.params.new_hname();

    let shop_proxy = f.state.shops().get_shop(&shop_name.value());

    if !shop_proxy.exists() {
        f.state
            .error_messages()
            .get_string(&ctx.request_id().to_string())
            .set_value("No shop matches given name");
        return;
    }

    let shop = shop_proxy.value();

    let mut updated_shop = russfest::Shop {
        shop_name: shop.shop_name,
        musician_name: shop.musician_name,
        fee: shop.fee,
        shop_owner: shop.shop_owner,
        is_registered: shop.is_registered,
        shop_hname: shop.shop_hname,
    };

    if fee.exists() {
        updated_shop.fee = fee.value();
    }

    if new_hname.exists() {
        updated_shop.shop_hname = new_hname.value();
    }

    updated_shop.is_registered = String::from(REQUESTING);

    f.state
        .shops()
        .get_shop(&updated_shop.shop_name)
        .set_value(&updated_shop);
}

pub fn view_call_get_shop_statistics(ctx: &ScViewContext, f: &CallGetShopStatisticsContext) {
    ctx.log("WRONG VIEW CALLED");
}

pub fn view_get_agend_id(ctx: &ScViewContext, f: &GetAgendIDContext) {
    f.results.sc_agent_id().set_value(&ctx.account_id())
}

pub fn view_get_all_open_shop_requests(ctx: &ScViewContext, f: &GetAllOpenShopRequestsContext) {
    // return all shops with open requests
    let shop_names = f.state.shopnames();

    for i in 0..shop_names.length() {
        let shop_name = shop_names.get_string(i).value();
        let shop: russfest::Shop = f.state.shops().get_shop(&shop_name).value();
        if shop.is_registered.eq(REQUESTING) {
            f.results.open_shop_request().append_shop().set_value(&shop);
        }
    }
}

pub fn view_get_all_products(ctx: &ScViewContext, f: &GetAllProductsContext) {
    for i in 0..f.state.products().length() {
        f.results
            .products()
            .append_product()
            .set_value(&f.state.products().get_product(i).value());
    }
}

pub fn view_get_all_registered_shops(ctx: &ScViewContext, f: &GetAllRegisteredShopsContext) {
    let shop_names = f.state.shopnames();
    for i in 0..shop_names.length() {
        let shop_name = shop_names.get_string(i).value();
        let shop: russfest::Shop = f.state.shops().get_shop(&shop_name).value();
        if shop.is_registered.eq(TRUE) {
            f.results.shops().append_shop().set_value(&shop);
        }
    }
}

pub fn view_get_denied_shop_requests(ctx: &ScViewContext, f: &GetDeniedShopRequestsContext) {
    let owner: ScAgentID = f.params.shop_owner().value();

    for i in 0..f.state.shopnames().length() {
        let shop_name = f.state.shopnames().get_string(i).value();

        let shop: russfest::Shop = f.state.shops().get_shop(&shop_name).value();
        if shop.shop_owner == owner && shop.is_registered.eq(DENIED) {
            f.results
                .denied_shop_requests()
                .append_shop()
                .set_value(&shop);
        }
    }
}

pub fn view_get_error_messages_view(ctx: &ScViewContext, f: &GetErrorMessagesViewContext) {
    let request_id = f.params.request_id();
    ctx.require(
        request_id.exists(),
        "No request ID was given for Get Error Message",
    );

    let error_message = f.state.error_messages().get_string(&request_id.value());

    if error_message.exists() {
        f.results.error_message().set_value(&error_message.value());
    } else {
        f.results.error_message().set_value("");
    }
}

pub fn view_get_festival_earnings(ctx: &ScViewContext, f: &GetFestivalEarningsContext) {
    f.results
        .festival_earnings()
        .set_value(f.state.earned_money().value());
}

pub fn view_get_musicians(ctx: &ScViewContext, f: &GetMusiciansContext) {
    for i in 0..f.state.musicians().length() {
        f.results
            .musicians()
            .append_musician()
            .set_value(&f.state.musicians().get_musician(i).value());
    }

    let test = "FUCK123";

    log(test);
}

pub fn view_get_musicians_without_shop(ctx: &ScViewContext, f: &GetMusiciansWithoutShopContext) {
    for i in 0..f.state.musicians().length() {
        let musician = f.state.musicians().get_musician(i).value();
        if musician.shop.is_empty() {
            f.results
                .musicians_without_shop()
                .append_musician()
                .set_value(&musician);
        }
    }
}

pub fn view_get_open_shop_request(ctx: &ScViewContext, f: &GetOpenShopRequestContext) {
    // return all shops belonging to shop owner
    let shop_names = f.state.shopnames();
    let shop_owner = f.params.shop_owner().value();

    for i in 0..shop_names.length() {
        let shop_name = shop_names.get_string(i).value();
        let shop: russfest::Shop = f.state.shops().get_shop(&shop_name).value();
        if shop.shop_owner == shop_owner && shop.is_registered.eq(REQUESTING) {
            f.results.open_shop_request().append_shop().set_value(&shop);
        }
    }
}

pub fn view_get_owner(ctx: &ScViewContext, f: &GetOwnerContext) {
    f.results.owner().set_value(&f.state.owner().value());
}

pub fn view_get_registered_shops_from_owner(
    ctx: &ScViewContext,
    f: &GetRegisteredShopsFromOwnerContext,
) {
    let owner = f.params.shop_owner();
    ctx.require(owner.exists(), "Parameter ShopOwner - AgentID is missing");

    let shop_names = f.state.shopnames();
    for i in 0..shop_names.length() {
        let shop_name = shop_names.get_string(i).value();
        let shop: russfest::Shop = f.state.shops().get_shop(&shop_name).value();
        if shop.is_registered.eq(TRUE) && shop.shop_owner == owner.value() {
            f.results.owner_shops().append_shop().set_value(&shop);
        }
    }
}

pub fn view_get_shop_statistics(ctx: &ScViewContext, f: &GetShopStatisticsContext) {
    let shop_name = f.params.shop_name();
    ctx.require(shop_name.exists(), "No shopName was given");

    let shop = f.state.shops().get_shop(&shop_name.value()).value();

    // get statistics from Shop
    log("TESTSETSETSET");

    let func = ScFuncs::call_get_shop_statistics(ctx);
    func.params.shop_name().set_value(&shop_name.value());
    func.func.of_contract(shop.shop_hname).call();

    // set results
    log("RUSSFEST RESULTS");

    let test1 = func.results.earnings().value();
    ctx.log(test1.to_string().as_str());

    // set musicianName
    f.results.musician().set_value(&shop.musician_name);
    f.results
        .earnings()
        .set_value(func.results.earnings().value());
    f.results
        .max_productions_step()
        .set_value(func.results.max_productions_step().value());
    f.results
        .produced_products()
        .set_value(func.results.produced_products().value());
    f.results
        .sold_products()
        .set_value(func.results.sold_products().value());

    // iterate over func result and add each template to result
    for i in 0..func.results.production_templates().length() {
        f.results
            .production_templates()
            .append_product_template()
            .set_value(
                &func
                    .results
                    .production_templates()
                    .get_product_template(i)
                    .value(),
            );
    }

    // iterate over func result and add each product to result
    for i in 0..func.results.production().length() {
        f.results
            .production()
            .append_statistic_product()
            .set_value(&func.results.production().get_statistic_product(i).value());
    }

    f.results.shop_name().set_value(&shop_name.value())
}

pub fn view_get_specific_products(ctx: &ScViewContext, f: &GetSpecificProductsContext) {
    let shop_name = f.params.shop_name().value();
    for i in 0..f.state.products().length() {
        let _product: russfest::Product = f.state.products().get_product(i).value();

        if _product.shop_name.eq(&shop_name) {
            f.results.products().append_product().set_value(&_product);
        }
    }
}

pub fn view_get_timeslots(ctx: &ScViewContext, f: &GetTimeslotsContext) {
    // get all timeslots
}

pub fn view_test_view(ctx: &ScViewContext, f: &TestViewContext) {
    f.results.test_string().append_string().set_value("TEST1");
    f.results.test_string().append_string().set_value("TEST2");
    f.results.test_string().append_string().set_value("TEST3");
}

pub fn view_test_view_single(ctx: &ScViewContext, f: &TestViewSingleContext) {
    f.results.singe_string().set_value("TEST Single String");
}

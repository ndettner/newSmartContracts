// Code generated by schema tool; DO NOT EDIT.

import * as wasmlib from 'wasmlib';
import * as sc from '../russfest/index';
import * as impl from './index'

const exportMap = new wasmlib.ScExportMap(
    [
        sc.FuncAcceptShop,
        sc.FuncAddMusician,
        sc.FuncBuyMerch,
        sc.FuncCallCheckProduct,
        sc.FuncCallCreateNewProduct,
        sc.FuncCallPayStore,
        sc.FuncCallPingShop,
        sc.FuncCallRecieveProducts,
        sc.FuncCallSendProduct,
        sc.FuncCancelShopRequest,
        sc.FuncDenyShop,
        sc.FuncInit,
        sc.FuncRequestShopLicence,
        sc.FuncSetOwner,
        sc.FuncUpdateDeniedShopRequest,
        sc.ViewCallGetShopStatistics,
        sc.ViewGetAgendID,
        sc.ViewGetAllOpenShopRequests,
        sc.ViewGetAllProducts,
        sc.ViewGetAllRegisteredShops,
        sc.ViewGetDeniedShopRequests,
        sc.ViewGetErrorMessagesView,
        sc.ViewGetFestivalEarnings,
        sc.ViewGetMusicians,
        sc.ViewGetMusiciansWithoutShop,
        sc.ViewGetOpenShopRequest,
        sc.ViewGetOwner,
        sc.ViewGetRegisteredShopsFromOwner,
        sc.ViewGetShopStatistics,
        sc.ViewGetSpecificProducts,
        sc.ViewGetTimeslots,
        sc.ViewTestView,
        sc.ViewTestViewSingle,
    ],
    [
        funcAcceptShopThunk,
        funcAddMusicianThunk,
        funcBuyMerchThunk,
        funcCallCheckProductThunk,
        funcCallCreateNewProductThunk,
        funcCallPayStoreThunk,
        funcCallPingShopThunk,
        funcCallRecieveProductsThunk,
        funcCallSendProductThunk,
        funcCancelShopRequestThunk,
        funcDenyShopThunk,
        funcInitThunk,
        funcRequestShopLicenceThunk,
        funcSetOwnerThunk,
        funcUpdateDeniedShopRequestThunk,
    ],
    [
        viewCallGetShopStatisticsThunk,
        viewGetAgendIDThunk,
        viewGetAllOpenShopRequestsThunk,
        viewGetAllProductsThunk,
        viewGetAllRegisteredShopsThunk,
        viewGetDeniedShopRequestsThunk,
        viewGetErrorMessagesViewThunk,
        viewGetFestivalEarningsThunk,
        viewGetMusiciansThunk,
        viewGetMusiciansWithoutShopThunk,
        viewGetOpenShopRequestThunk,
        viewGetOwnerThunk,
        viewGetRegisteredShopsFromOwnerThunk,
        viewGetShopStatisticsThunk,
        viewGetSpecificProductsThunk,
        viewGetTimeslotsThunk,
        viewTestViewThunk,
        viewTestViewSingleThunk,
    ]);

export function onDispatch(index: i32): void {
    exportMap.dispatch(index);
}

function funcAcceptShopThunk(ctx: wasmlib.ScFuncContext): void {
    ctx.log('russfest.funcAcceptShop');
    let f = new sc.AcceptShopContext();

    // only owner can accept Shop
    const access = f.state.owner();
    ctx.require(access.exists(), 'access not set: owner');
    ctx.require(ctx.caller().equals(access.value()), 'no permission');

    ctx.require(f.params.shopName().exists(), 'missing mandatory param: shopName');
    impl.funcAcceptShop(ctx, f);
    ctx.log('russfest.funcAcceptShop ok');
}

function funcAddMusicianThunk(ctx: wasmlib.ScFuncContext): void {
    ctx.log('russfest.funcAddMusician');
    let f = new sc.AddMusicianContext();

    // current owner of this smart contract
    const access = f.state.owner();
    ctx.require(access.exists(), 'access not set: owner');
    ctx.require(ctx.caller().equals(access.value()), 'no permission');

    ctx.require(f.params.name().exists(), 'missing mandatory param: name');
    impl.funcAddMusician(ctx, f);
    ctx.log('russfest.funcAddMusician ok');
}

function funcBuyMerchThunk(ctx: wasmlib.ScFuncContext): void {
    ctx.log('russfest.funcBuyMerch');
    let f = new sc.BuyMerchContext();
    ctx.require(f.params.musician().exists(), 'missing mandatory param: musician');
    ctx.require(f.params.productType().exists(), 'missing mandatory param: productType');
    ctx.require(f.params.shopName().exists(), 'missing mandatory param: shopName');
    impl.funcBuyMerch(ctx, f);
    ctx.log('russfest.funcBuyMerch ok');
}

function funcCallCheckProductThunk(ctx: wasmlib.ScFuncContext): void {
    ctx.log('russfest.funcCallCheckProduct');
    let f = new sc.CallCheckProductContext();
    const results = new wasmlib.ScDict(null);
    f.results = new sc.MutableCallCheckProductResults(results.asProxy());
    impl.funcCallCheckProduct(ctx, f);
    ctx.results(results);
    ctx.log('russfest.funcCallCheckProduct ok');
}

function funcCallCreateNewProductThunk(ctx: wasmlib.ScFuncContext): void {
    ctx.log('russfest.funcCallCreateNewProduct');
    let f = new sc.CallCreateNewProductContext();
    ctx.require(f.params.musicianName().exists(), 'missing mandatory param: musicianName');
    ctx.require(f.params.price().exists(), 'missing mandatory param: price');
    ctx.require(f.params.productType().exists(), 'missing mandatory param: productType');
    ctx.require(f.params.shopName().exists(), 'missing mandatory param: shopName');
    impl.funcCallCreateNewProduct(ctx, f);
    ctx.log('russfest.funcCallCreateNewProduct ok');
}

function funcCallPayStoreThunk(ctx: wasmlib.ScFuncContext): void {
    ctx.log('russfest.funcCallPayStore');
    let f = new sc.CallPayStoreContext();
    ctx.require(f.params.shopName().exists(), 'missing mandatory param: shopName');
    impl.funcCallPayStore(ctx, f);
    ctx.log('russfest.funcCallPayStore ok');
}

function funcCallPingShopThunk(ctx: wasmlib.ScFuncContext): void {
    ctx.log('russfest.funcCallPingShop');
    let f = new sc.CallPingShopContext();
    const results = new wasmlib.ScDict(null);
    f.results = new sc.MutableCallPingShopResults(results.asProxy());
    impl.funcCallPingShop(ctx, f);
    ctx.results(results);
    ctx.log('russfest.funcCallPingShop ok');
}

function funcCallRecieveProductsThunk(ctx: wasmlib.ScFuncContext): void {
    ctx.log('russfest.funcCallRecieveProducts');
    let f = new sc.CallRecieveProductsContext();
    const results = new wasmlib.ScDict(null);
    f.results = new sc.MutableCallRecieveProductsResults(results.asProxy());
    impl.funcCallRecieveProducts(ctx, f);
    ctx.results(results);
    ctx.log('russfest.funcCallRecieveProducts ok');
}

function funcCallSendProductThunk(ctx: wasmlib.ScFuncContext): void {
    ctx.log('russfest.funcCallSendProduct');
    let f = new sc.CallSendProductContext();
    impl.funcCallSendProduct(ctx, f);
    ctx.log('russfest.funcCallSendProduct ok');
}

function funcCancelShopRequestThunk(ctx: wasmlib.ScFuncContext): void {
    ctx.log('russfest.funcCancelShopRequest');
    let f = new sc.CancelShopRequestContext();
    ctx.require(f.params.name().exists(), 'missing mandatory param: name');
    impl.funcCancelShopRequest(ctx, f);
    ctx.log('russfest.funcCancelShopRequest ok');
}

function funcDenyShopThunk(ctx: wasmlib.ScFuncContext): void {
    ctx.log('russfest.funcDenyShop');
    let f = new sc.DenyShopContext();

    // only owner can accept Shop
    const access = f.state.owner();
    ctx.require(access.exists(), 'access not set: owner');
    ctx.require(ctx.caller().equals(access.value()), 'no permission');

    ctx.require(f.params.shopName().exists(), 'missing mandatory param: shopName');
    impl.funcDenyShop(ctx, f);
    ctx.log('russfest.funcDenyShop ok');
}

function funcInitThunk(ctx: wasmlib.ScFuncContext): void {
    ctx.log('russfest.funcInit');
    let f = new sc.InitContext();
    impl.funcInit(ctx, f);
    ctx.log('russfest.funcInit ok');
}

function funcRequestShopLicenceThunk(ctx: wasmlib.ScFuncContext): void {
    ctx.log('russfest.funcRequestShopLicence');
    let f = new sc.RequestShopLicenceContext();
    ctx.require(f.params.fee().exists(), 'missing mandatory param: fee');
    ctx.require(f.params.musicianName().exists(), 'missing mandatory param: musicianName');
    ctx.require(f.params.name().exists(), 'missing mandatory param: name');
    ctx.require(f.params.shopHname().exists(), 'missing mandatory param: shopHname');
    impl.funcRequestShopLicence(ctx, f);
    ctx.log('russfest.funcRequestShopLicence ok');
}

function funcSetOwnerThunk(ctx: wasmlib.ScFuncContext): void {
    ctx.log('russfest.funcSetOwner');
    let f = new sc.SetOwnerContext();
    ctx.require(f.params.newOwner().exists(), 'missing mandatory param: newOwner');
    impl.funcSetOwner(ctx, f);
    ctx.log('russfest.funcSetOwner ok');
}

function funcUpdateDeniedShopRequestThunk(ctx: wasmlib.ScFuncContext): void {
    ctx.log('russfest.funcUpdateDeniedShopRequest');
    let f = new sc.UpdateDeniedShopRequestContext();
    ctx.require(f.params.shopName().exists(), 'missing mandatory param: shopName');
    impl.funcUpdateDeniedShopRequest(ctx, f);
    ctx.log('russfest.funcUpdateDeniedShopRequest ok');
}

function viewCallGetShopStatisticsThunk(ctx: wasmlib.ScViewContext): void {
    ctx.log('russfest.viewCallGetShopStatistics');
    let f = new sc.CallGetShopStatisticsContext();
    const results = new wasmlib.ScDict(null);
    f.results = new sc.MutableCallGetShopStatisticsResults(results.asProxy());
    ctx.require(f.params.shopName().exists(), 'missing mandatory param: shopName');
    impl.viewCallGetShopStatistics(ctx, f);
    ctx.results(results);
    ctx.log('russfest.viewCallGetShopStatistics ok');
}

function viewGetAgendIDThunk(ctx: wasmlib.ScViewContext): void {
    ctx.log('russfest.viewGetAgendID');
    let f = new sc.GetAgendIDContext();
    const results = new wasmlib.ScDict(null);
    f.results = new sc.MutableGetAgendIDResults(results.asProxy());
    impl.viewGetAgendID(ctx, f);
    ctx.results(results);
    ctx.log('russfest.viewGetAgendID ok');
}

function viewGetAllOpenShopRequestsThunk(ctx: wasmlib.ScViewContext): void {
    ctx.log('russfest.viewGetAllOpenShopRequests');
    let f = new sc.GetAllOpenShopRequestsContext();
    const results = new wasmlib.ScDict(null);
    f.results = new sc.MutableGetAllOpenShopRequestsResults(results.asProxy());
    impl.viewGetAllOpenShopRequests(ctx, f);
    ctx.results(results);
    ctx.log('russfest.viewGetAllOpenShopRequests ok');
}

function viewGetAllProductsThunk(ctx: wasmlib.ScViewContext): void {
    ctx.log('russfest.viewGetAllProducts');
    let f = new sc.GetAllProductsContext();
    const results = new wasmlib.ScDict(null);
    f.results = new sc.MutableGetAllProductsResults(results.asProxy());
    impl.viewGetAllProducts(ctx, f);
    ctx.results(results);
    ctx.log('russfest.viewGetAllProducts ok');
}

function viewGetAllRegisteredShopsThunk(ctx: wasmlib.ScViewContext): void {
    ctx.log('russfest.viewGetAllRegisteredShops');
    let f = new sc.GetAllRegisteredShopsContext();
    const results = new wasmlib.ScDict(null);
    f.results = new sc.MutableGetAllRegisteredShopsResults(results.asProxy());
    impl.viewGetAllRegisteredShops(ctx, f);
    ctx.results(results);
    ctx.log('russfest.viewGetAllRegisteredShops ok');
}

function viewGetDeniedShopRequestsThunk(ctx: wasmlib.ScViewContext): void {
    ctx.log('russfest.viewGetDeniedShopRequests');
    let f = new sc.GetDeniedShopRequestsContext();
    const results = new wasmlib.ScDict(null);
    f.results = new sc.MutableGetDeniedShopRequestsResults(results.asProxy());
    ctx.require(f.params.shopOwner().exists(), 'missing mandatory param: shopOwner');
    impl.viewGetDeniedShopRequests(ctx, f);
    ctx.results(results);
    ctx.log('russfest.viewGetDeniedShopRequests ok');
}

function viewGetErrorMessagesViewThunk(ctx: wasmlib.ScViewContext): void {
    ctx.log('russfest.viewGetErrorMessagesView');
    let f = new sc.GetErrorMessagesViewContext();
    const results = new wasmlib.ScDict(null);
    f.results = new sc.MutableGetErrorMessagesViewResults(results.asProxy());
    ctx.require(f.params.requestID().exists(), 'missing mandatory param: requestID');
    impl.viewGetErrorMessagesView(ctx, f);
    ctx.results(results);
    ctx.log('russfest.viewGetErrorMessagesView ok');
}

function viewGetFestivalEarningsThunk(ctx: wasmlib.ScViewContext): void {
    ctx.log('russfest.viewGetFestivalEarnings');
    let f = new sc.GetFestivalEarningsContext();
    const results = new wasmlib.ScDict(null);
    f.results = new sc.MutableGetFestivalEarningsResults(results.asProxy());
    impl.viewGetFestivalEarnings(ctx, f);
    ctx.results(results);
    ctx.log('russfest.viewGetFestivalEarnings ok');
}

function viewGetMusiciansThunk(ctx: wasmlib.ScViewContext): void {
    ctx.log('russfest.viewGetMusicians');
    let f = new sc.GetMusiciansContext();
    const results = new wasmlib.ScDict(null);
    f.results = new sc.MutableGetMusiciansResults(results.asProxy());
    impl.viewGetMusicians(ctx, f);
    ctx.results(results);
    ctx.log('russfest.viewGetMusicians ok');
}

function viewGetMusiciansWithoutShopThunk(ctx: wasmlib.ScViewContext): void {
    ctx.log('russfest.viewGetMusiciansWithoutShop');
    let f = new sc.GetMusiciansWithoutShopContext();
    const results = new wasmlib.ScDict(null);
    f.results = new sc.MutableGetMusiciansWithoutShopResults(results.asProxy());
    impl.viewGetMusiciansWithoutShop(ctx, f);
    ctx.results(results);
    ctx.log('russfest.viewGetMusiciansWithoutShop ok');
}

function viewGetOpenShopRequestThunk(ctx: wasmlib.ScViewContext): void {
    ctx.log('russfest.viewGetOpenShopRequest');
    let f = new sc.GetOpenShopRequestContext();
    const results = new wasmlib.ScDict(null);
    f.results = new sc.MutableGetOpenShopRequestResults(results.asProxy());
    ctx.require(f.params.shopOwner().exists(), 'missing mandatory param: shopOwner');
    impl.viewGetOpenShopRequest(ctx, f);
    ctx.results(results);
    ctx.log('russfest.viewGetOpenShopRequest ok');
}

function viewGetOwnerThunk(ctx: wasmlib.ScViewContext): void {
    ctx.log('russfest.viewGetOwner');
    let f = new sc.GetOwnerContext();
    const results = new wasmlib.ScDict(null);
    f.results = new sc.MutableGetOwnerResults(results.asProxy());
    impl.viewGetOwner(ctx, f);
    ctx.results(results);
    ctx.log('russfest.viewGetOwner ok');
}

function viewGetRegisteredShopsFromOwnerThunk(ctx: wasmlib.ScViewContext): void {
    ctx.log('russfest.viewGetRegisteredShopsFromOwner');
    let f = new sc.GetRegisteredShopsFromOwnerContext();
    const results = new wasmlib.ScDict(null);
    f.results = new sc.MutableGetRegisteredShopsFromOwnerResults(results.asProxy());
    ctx.require(f.params.shopOwner().exists(), 'missing mandatory param: shopOwner');
    impl.viewGetRegisteredShopsFromOwner(ctx, f);
    ctx.results(results);
    ctx.log('russfest.viewGetRegisteredShopsFromOwner ok');
}

function viewGetShopStatisticsThunk(ctx: wasmlib.ScViewContext): void {
    ctx.log('russfest.viewGetShopStatistics');
    let f = new sc.GetShopStatisticsContext();
    const results = new wasmlib.ScDict(null);
    f.results = new sc.MutableGetShopStatisticsResults(results.asProxy());
    ctx.require(f.params.shopName().exists(), 'missing mandatory param: shopName');
    impl.viewGetShopStatistics(ctx, f);
    ctx.results(results);
    ctx.log('russfest.viewGetShopStatistics ok');
}

function viewGetSpecificProductsThunk(ctx: wasmlib.ScViewContext): void {
    ctx.log('russfest.viewGetSpecificProducts');
    let f = new sc.GetSpecificProductsContext();
    const results = new wasmlib.ScDict(null);
    f.results = new sc.MutableGetSpecificProductsResults(results.asProxy());
    ctx.require(f.params.shopName().exists(), 'missing mandatory param: shopName');
    impl.viewGetSpecificProducts(ctx, f);
    ctx.results(results);
    ctx.log('russfest.viewGetSpecificProducts ok');
}

function viewGetTimeslotsThunk(ctx: wasmlib.ScViewContext): void {
    ctx.log('russfest.viewGetTimeslots');
    let f = new sc.GetTimeslotsContext();
    const results = new wasmlib.ScDict(null);
    f.results = new sc.MutableGetTimeslotsResults(results.asProxy());
    impl.viewGetTimeslots(ctx, f);
    ctx.results(results);
    ctx.log('russfest.viewGetTimeslots ok');
}

function viewTestViewThunk(ctx: wasmlib.ScViewContext): void {
    ctx.log('russfest.viewTestView');
    let f = new sc.TestViewContext();
    const results = new wasmlib.ScDict(null);
    f.results = new sc.MutableTestViewResults(results.asProxy());
    impl.viewTestView(ctx, f);
    ctx.results(results);
    ctx.log('russfest.viewTestView ok');
}

function viewTestViewSingleThunk(ctx: wasmlib.ScViewContext): void {
    ctx.log('russfest.viewTestViewSingle');
    let f = new sc.TestViewSingleContext();
    const results = new wasmlib.ScDict(null);
    f.results = new sc.MutableTestViewSingleResults(results.asProxy());
    impl.viewTestViewSingle(ctx, f);
    ctx.results(results);
    ctx.log('russfest.viewTestViewSingle ok');
}

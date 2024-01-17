import * as wasmlib from 'wasmlib';
import * as sc from '../russfest/index';

export function funcAcceptShop(ctx: wasmlib.ScFuncContext, f: sc.AcceptShopContext): void {
}

export function funcAddMusician(ctx: wasmlib.ScFuncContext, f: sc.AddMusicianContext): void {
}

export function funcBuyMerch(ctx: wasmlib.ScFuncContext, f: sc.BuyMerchContext): void {
}

export function funcCallCheckProduct(ctx: wasmlib.ScFuncContext, f: sc.CallCheckProductContext): void {
}

export function funcCallCreateNewProduct(ctx: wasmlib.ScFuncContext, f: sc.CallCreateNewProductContext): void {
}

export function funcCallPayStore(ctx: wasmlib.ScFuncContext, f: sc.CallPayStoreContext): void {
}

export function funcCallPingShop(ctx: wasmlib.ScFuncContext, f: sc.CallPingShopContext): void {
}

export function funcCallRecieveProducts(ctx: wasmlib.ScFuncContext, f: sc.CallRecieveProductsContext): void {
}

export function funcCallSendProduct(ctx: wasmlib.ScFuncContext, f: sc.CallSendProductContext): void {
}

export function funcCancelShopRequest(ctx: wasmlib.ScFuncContext, f: sc.CancelShopRequestContext): void {
}

export function funcDenyShop(ctx: wasmlib.ScFuncContext, f: sc.DenyShopContext): void {
}

export function funcInit(ctx: wasmlib.ScFuncContext, f: sc.InitContext): void {
    if (f.params.owner().exists()) {
        f.state.owner().setValue(f.params.owner().value());
        return;
    }
    f.state.owner().setValue(ctx.requestSender());
}

export function funcRequestShopLicence(ctx: wasmlib.ScFuncContext, f: sc.RequestShopLicenceContext): void {
}

export function funcSetOwner(ctx: wasmlib.ScFuncContext, f: sc.SetOwnerContext): void {
    f.state.owner().setValue(f.params.owner().value());
}

export function funcUpdateDeniedShopRequest(ctx: wasmlib.ScFuncContext, f: sc.UpdateDeniedShopRequestContext): void {
}

export function viewCallGetShopStatistics(ctx: wasmlib.ScViewContext, f: sc.CallGetShopStatisticsContext): void {
}

export function viewGetAgendID(ctx: wasmlib.ScViewContext, f: sc.GetAgendIDContext): void {
}

export function viewGetAllOpenShopRequests(ctx: wasmlib.ScViewContext, f: sc.GetAllOpenShopRequestsContext): void {
}

export function viewGetAllProducts(ctx: wasmlib.ScViewContext, f: sc.GetAllProductsContext): void {
}

export function viewGetAllRegisteredShops(ctx: wasmlib.ScViewContext, f: sc.GetAllRegisteredShopsContext): void {
}

export function viewGetDeniedShopRequests(ctx: wasmlib.ScViewContext, f: sc.GetDeniedShopRequestsContext): void {
}

export function viewGetErrorMessagesView(ctx: wasmlib.ScViewContext, f: sc.GetErrorMessagesViewContext): void {
}

export function viewGetFestivalEarnings(ctx: wasmlib.ScViewContext, f: sc.GetFestivalEarningsContext): void {
}

export function viewGetMusicians(ctx: wasmlib.ScViewContext, f: sc.GetMusiciansContext): void {
}

export function viewGetMusiciansWithoutShop(ctx: wasmlib.ScViewContext, f: sc.GetMusiciansWithoutShopContext): void {
}

export function viewGetOpenShopRequest(ctx: wasmlib.ScViewContext, f: sc.GetOpenShopRequestContext): void {
}

export function viewGetOwner(ctx: wasmlib.ScViewContext, f: sc.GetOwnerContext): void {
    f.results.owner().setValue(f.state.owner().value());
}

export function viewGetRegisteredShopsFromOwner(ctx: wasmlib.ScViewContext, f: sc.GetRegisteredShopsFromOwnerContext): void {
}

export function viewGetShopStatistics(ctx: wasmlib.ScViewContext, f: sc.GetShopStatisticsContext): void {
}

export function viewGetSpecificProducts(ctx: wasmlib.ScViewContext, f: sc.GetSpecificProductsContext): void {
}

export function viewGetTimeslots(ctx: wasmlib.ScViewContext, f: sc.GetTimeslotsContext): void {
}

export function viewTestView(ctx: wasmlib.ScViewContext, f: sc.TestViewContext): void {
}

export function viewTestViewSingle(ctx: wasmlib.ScViewContext, f: sc.TestViewSingleContext): void {
}

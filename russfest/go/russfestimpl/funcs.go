package russfestimpl

import (
	"github.com/iotaledger/wasp/packages/wasmvm/wasmlib/go/wasmlib"
)

func funcAcceptShop(ctx wasmlib.ScFuncContext, f *AcceptShopContext) {
}

func funcAddMusician(ctx wasmlib.ScFuncContext, f *AddMusicianContext) {
}

func funcBuyMerch(ctx wasmlib.ScFuncContext, f *BuyMerchContext) {
}

func funcCallCheckProduct(ctx wasmlib.ScFuncContext, f *CallCheckProductContext) {
}

func funcCallCreateNewProduct(ctx wasmlib.ScFuncContext, f *CallCreateNewProductContext) {
}

func funcCallPayStore(ctx wasmlib.ScFuncContext, f *CallPayStoreContext) {
}

func funcCallPingShop(ctx wasmlib.ScFuncContext, f *CallPingShopContext) {
}

func funcCallRecieveProducts(ctx wasmlib.ScFuncContext, f *CallRecieveProductsContext) {
}

func funcCallSendProduct(ctx wasmlib.ScFuncContext, f *CallSendProductContext) {
}

func funcCancelShopRequest(ctx wasmlib.ScFuncContext, f *CancelShopRequestContext) {
}

func funcDenyShop(ctx wasmlib.ScFuncContext, f *DenyShopContext) {
}

func funcInit(ctx wasmlib.ScFuncContext, f *InitContext) {
	if f.Params.Owner().Exists() {
		f.State.Owner().SetValue(f.Params.Owner().Value())
		return
	}
	f.State.Owner().SetValue(ctx.RequestSender())
}

func funcRequestShopLicence(ctx wasmlib.ScFuncContext, f *RequestShopLicenceContext) {
}

func funcSetOwner(ctx wasmlib.ScFuncContext, f *SetOwnerContext) {
	f.State.Owner().SetValue(f.Params.Owner().Value())
}

func funcUpdateDeniedShopRequest(ctx wasmlib.ScFuncContext, f *UpdateDeniedShopRequestContext) {
}

func viewCallGetShopStatistics(ctx wasmlib.ScViewContext, f *CallGetShopStatisticsContext) {
}

func viewGetAgendID(ctx wasmlib.ScViewContext, f *GetAgendIDContext) {
}

func viewGetAllOpenShopRequests(ctx wasmlib.ScViewContext, f *GetAllOpenShopRequestsContext) {
}

func viewGetAllProducts(ctx wasmlib.ScViewContext, f *GetAllProductsContext) {
}

func viewGetAllRegisteredShops(ctx wasmlib.ScViewContext, f *GetAllRegisteredShopsContext) {
}

func viewGetDeniedShopRequests(ctx wasmlib.ScViewContext, f *GetDeniedShopRequestsContext) {
}

func viewGetErrorMessagesView(ctx wasmlib.ScViewContext, f *GetErrorMessagesViewContext) {
}

func viewGetFestivalEarnings(ctx wasmlib.ScViewContext, f *GetFestivalEarningsContext) {
}

func viewGetMusicians(ctx wasmlib.ScViewContext, f *GetMusiciansContext) {
}

func viewGetMusiciansWithoutShop(ctx wasmlib.ScViewContext, f *GetMusiciansWithoutShopContext) {
}

func viewGetOpenShopRequest(ctx wasmlib.ScViewContext, f *GetOpenShopRequestContext) {
}

func viewGetOwner(ctx wasmlib.ScViewContext, f *GetOwnerContext) {
	f.Results.Owner().SetValue(f.State.Owner().Value())
}

func viewGetRegisteredShopsFromOwner(ctx wasmlib.ScViewContext, f *GetRegisteredShopsFromOwnerContext) {
}

func viewGetShopStatistics(ctx wasmlib.ScViewContext, f *GetShopStatisticsContext) {
}

func viewGetSpecificProducts(ctx wasmlib.ScViewContext, f *GetSpecificProductsContext) {
}

func viewGetTimeslots(ctx wasmlib.ScViewContext, f *GetTimeslotsContext) {
}

func viewTestView(ctx wasmlib.ScViewContext, f *TestViewContext) {
}

func viewTestViewSingle(ctx wasmlib.ScViewContext, f *TestViewSingleContext) {
}

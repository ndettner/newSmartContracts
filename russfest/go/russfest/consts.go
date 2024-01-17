// Code generated by schema tool; DO NOT EDIT.

package russfest

import "github.com/iotaledger/wasp/packages/wasmvm/wasmlib/go/wasmlib/wasmtypes"

const (
	ScName        = "russfest"
	ScDescription = "Central Smart Contract to handle Russfest interactions"
	HScName       = wasmtypes.ScHname(0xdebcfc69)
)

const (
	ParamFee          = "fee"
	ParamMusician     = "musician"
	ParamMusicianName = "musicianName"
	ParamName         = "name"
	ParamNewfee       = "newfee"
	ParamNewHname     = "newHname"
	ParamNewOwner     = "newOwner"
	ParamOwner        = "owner"
	ParamPrice        = "price"
	ParamProduct      = "product"
	ParamProductType  = "productType"
	ParamRequestID    = "requestID"
	ParamShop         = "shop"
	ParamShopHname    = "shopHname"
	ParamShopName     = "shopName"
	ParamShopOwner    = "shopOwner"
)

const (
	ResultCanProduce           = "canProduce"
	ResultDeniedShopRequests   = "deniedShopRequests"
	ResultEarnings             = "earnings"
	ResultErrorMessage         = "errorMessage"
	ResultFestivalEarnings     = "festivalEarnings"
	ResultMaxProductionsStep   = "maxProductionsStep"
	ResultMessage              = "message"
	ResultMusician             = "musician"
	ResultMusicians            = "musicians"
	ResultMusiciansWithoutShop = "musiciansWithoutShop"
	ResultOpenShopRequest      = "openShopRequest"
	ResultOwner                = "owner"
	ResultOwnerShops           = "ownerShops"
	ResultPingSuccessful       = "pingSuccessful"
	ResultProducedProducts     = "producedProducts"
	ResultProduct              = "product"
	ResultProduction           = "production"
	ResultProductionTemplates  = "productionTemplates"
	ResultProducts             = "products"
	ResultScAgentID            = "ScAgentId"
	ResultShopName             = "shopName"
	ResultShops                = "shops"
	ResultSingeString          = "singeString"
	ResultSoldProducts         = "soldProducts"
	ResultTestString           = "testString"
	ResultTimeslots            = "timeslots"
)

const (
	StateEarnedMoney   = "earnedMoney"
	StateErrorMessages = "errorMessages"
	StateMusicians     = "musicians"
	StateOwner         = "owner"
	StateProducts      = "products"
	StateShopnames     = "shopnames"
	StateShops         = "Shops"
	StateTimeslots     = "timeslots"
)

const (
	FuncAcceptShop                  = "acceptShop"
	FuncAddMusician                 = "addMusician"
	FuncBuyMerch                    = "buyMerch"
	FuncCallCheckProduct            = "callCheckProduct"
	FuncCallCreateNewProduct        = "callCreateNewProduct"
	FuncCallPayStore                = "callPayStore"
	FuncCallPingShop                = "callPingShop"
	FuncCallRecieveProducts         = "callRecieveProducts"
	FuncCallSendProduct             = "callSendProduct"
	FuncCancelShopRequest           = "cancelShopRequest"
	FuncDenyShop                    = "denyShop"
	FuncInit                        = "init"
	FuncRequestShopLicence          = "requestShopLicence"
	FuncSetOwner                    = "setOwner"
	FuncUpdateDeniedShopRequest     = "updateDeniedShopRequest"
	ViewCallGetShopStatistics       = "callGetShopStatistics"
	ViewGetAgendID                  = "getAgendID"
	ViewGetAllOpenShopRequests      = "getAllOpenShopRequests"
	ViewGetAllProducts              = "getAllProducts"
	ViewGetAllRegisteredShops       = "getAllRegisteredShops"
	ViewGetDeniedShopRequests       = "getDeniedShopRequests"
	ViewGetErrorMessagesView        = "getErrorMessagesView"
	ViewGetFestivalEarnings         = "getFestivalEarnings"
	ViewGetMusicians                = "getMusicians"
	ViewGetMusiciansWithoutShop     = "getMusiciansWithoutShop"
	ViewGetOpenShopRequest          = "getOpenShopRequest"
	ViewGetOwner                    = "getOwner"
	ViewGetRegisteredShopsFromOwner = "getRegisteredShopsFromOwner"
	ViewGetShopStatistics           = "getShopStatistics"
	ViewGetSpecificProducts         = "getSpecificProducts"
	ViewGetTimeslots                = "getTimeslots"
	ViewTestView                    = "testView"
	ViewTestViewSingle              = "testViewSingle"
)

const (
	HFuncAcceptShop                  = wasmtypes.ScHname(0x8dbc2867)
	HFuncAddMusician                 = wasmtypes.ScHname(0x793f88c3)
	HFuncBuyMerch                    = wasmtypes.ScHname(0xb5389038)
	HFuncCallCheckProduct            = wasmtypes.ScHname(0xdfd91d1f)
	HFuncCallCreateNewProduct        = wasmtypes.ScHname(0xaf42fee4)
	HFuncCallPayStore                = wasmtypes.ScHname(0xc8ab38d7)
	HFuncCallPingShop                = wasmtypes.ScHname(0xc6022857)
	HFuncCallRecieveProducts         = wasmtypes.ScHname(0x7e74c00e)
	HFuncCallSendProduct             = wasmtypes.ScHname(0x36c3773f)
	HFuncCancelShopRequest           = wasmtypes.ScHname(0xbc07aac0)
	HFuncDenyShop                    = wasmtypes.ScHname(0x19e94b0c)
	HFuncInit                        = wasmtypes.ScHname(0x1f44d644)
	HFuncRequestShopLicence          = wasmtypes.ScHname(0x5b4431aa)
	HFuncSetOwner                    = wasmtypes.ScHname(0x2a15fe7b)
	HFuncUpdateDeniedShopRequest     = wasmtypes.ScHname(0xa7c5088c)
	HViewCallGetShopStatistics       = wasmtypes.ScHname(0x52d265ae)
	HViewGetAgendID                  = wasmtypes.ScHname(0x711f72c9)
	HViewGetAllOpenShopRequests      = wasmtypes.ScHname(0x1a9bd31e)
	HViewGetAllProducts              = wasmtypes.ScHname(0x15b61b1d)
	HViewGetAllRegisteredShops       = wasmtypes.ScHname(0x2bbedcbe)
	HViewGetDeniedShopRequests       = wasmtypes.ScHname(0x379ef7a7)
	HViewGetErrorMessagesView        = wasmtypes.ScHname(0x93d2c12d)
	HViewGetFestivalEarnings         = wasmtypes.ScHname(0x74916c40)
	HViewGetMusicians                = wasmtypes.ScHname(0x2737b469)
	HViewGetMusiciansWithoutShop     = wasmtypes.ScHname(0x5b027c03)
	HViewGetOpenShopRequest          = wasmtypes.ScHname(0x87fac614)
	HViewGetOwner                    = wasmtypes.ScHname(0x137107a6)
	HViewGetRegisteredShopsFromOwner = wasmtypes.ScHname(0x90b51c8b)
	HViewGetShopStatistics           = wasmtypes.ScHname(0xc96b9468)
	HViewGetSpecificProducts         = wasmtypes.ScHname(0x4e970d35)
	HViewGetTimeslots                = wasmtypes.ScHname(0x0f2348f9)
	HViewTestView                    = wasmtypes.ScHname(0xf6446217)
	HViewTestViewSingle              = wasmtypes.ScHname(0xdc5b8f36)
)

package test

import (
	"testing"
	"time"

	"github.com/stretchr/testify/require"

	"github.com/iotaledger/wasp/contracts/wasm/russfest/russfest/go/russfest"
	"github.com/iotaledger/wasp/contracts/wasm/russfest/russfest/go/russfestimpl"
	"github.com/iotaledger/wasp/contracts/wasm/russfest/shop1/go/shop1"
	"github.com/iotaledger/wasp/contracts/wasm/russfest/shop1/go/shop1impl"

	"github.com/iotaledger/wasp/packages/solo"
	"github.com/iotaledger/wasp/packages/wasmvm/wasmsolo"
)

func TestDeploy(t *testing.T) {
	ctx := wasmsolo.NewSoloContext(t, russfest.ScName, russfestimpl.OnDispatch)
	require.NoError(t, ctx.ContractExists(russfest.ScName))
}

func TestRussfestDeplay(t *testing.T) {
	env := solo.New(t, &solo.InitOptions{AutoAdjustStorageDeposit: true})
	chain := env.NewChain()
	err := chain.DeployWasmContract(nil, "russfest", "russfestwasm_bg.wasm")
	require.NoError(t, err)
}

func TestRegisterShop(t *testing.T) {
	ctx := wasmsolo.NewSoloContext(t, russfest.ScName, russfestimpl.OnDispatch)

	// setup Russ Hannemann
	russHannemann := ctx.NewSoloAgent("Russ Hannemann")
	require.NoError(t, ctx.Err)
	println(russHannemann.Balance())
	println("RUSS HANNEMAN SOLO AGENT ID")
	println(russHannemann.ScAgentID().String())

	// setup ShopOwner
	shopOwner := ctx.NewSoloAgent("Shop Owner")
	require.NoError(t, ctx.Err)
	println(shopOwner.Balance())
	println("SHOPOWNER SOLO AGENT ID")
	println(shopOwner.ScAgentID().String())

	// TODO weiß nicht genau was das bezwecken sollte
	// shopTransfer := ctx.Transfer()
	// shopTransfer.Set(wasmtypes.IOTA, 1)

	ctx2 := wasmsolo.NewSoloContextForChain(t, ctx.Chain, ctx.Creator(), shop1.ScName, shop1impl.OnDispatch)
	require.NoError(t, ctx2.Err)

	// set russ as Owner
	f := russfest.ScFuncs.SetOwner(ctx)
	f.Params.NewOwner().SetValue(russHannemann.ScAgentID())
	f.Func.TransferBaseTokens(1).Post()
	require.NoError(t, ctx.Err)

	// set shopOwner as Owner of shop1
	setShopOwner := shop1.ScFuncs.SetOwner(ctx2)
	setShopOwner.Params.Owner().SetValue(shopOwner.ScAgentID())
	setShopOwner.Func.TransferBaseTokens(1).Post()
	require.NoError(t, ctx2.Err)

	// try to register a shop without an existing musician
	shopLicenceFunction := russfest.ScFuncs.RequestShopLicence(ctx.Sign(shopOwner))
	shopLicenceFunction.Params.Name().SetValue("ALLIGATOAH SHOP")
	shopLicenceFunction.Params.MusicianName().SetValue("Alligatoah")
	shopLicenceFunction.Params.Fee().SetValue(15)
	// TODO replace with real smart contract

	shopLicenceFunction.Params.ShopHname().SetValue(shop1.HScName)
	shopLicenceFunction.Func.TransferBaseTokens(1).Post()

	// get error Message

	//require.Error(t, ctx.Err)

	// add a new Musician
	addMusicianFunction := russfest.ScFuncs.AddMusician(ctx.Sign(russHannemann))
	addMusicianFunction.Params.Name().SetValue(("Alligatoah"))
	addMusicianFunction.Func.TransferBaseTokens(1).Post()
	require.NoError(t, ctx.Err)

	_musiciansView := russfest.ScFuncs.GetMusicians(ctx)
	_musiciansView.Func.Call()

	var test uint32 = 1
	require.EqualValues(t, test, _musiciansView.Results.Musicians().Length())

	// add a second Musician
	addMusicianFunction.Params.Name().SetValue("KIZ")
	addMusicianFunction.Func.TransferBaseTokens(1).Post()
	require.NoError(t, ctx.Err)

	_musiciansView.Func.Call()
	require.EqualValues(t, 2, _musiciansView.Results.Musicians().Length())

	// register first shop for reals
	ctx.Sign(shopOwner)
	shopLicenceFunction.Func.TransferBaseTokens(1).Post()
	require.NoError(t, ctx.Err)

	// register second shop
	shopLicenceFunction.Params.Name().SetValue("KIZ SHOP")
	shopLicenceFunction.Params.MusicianName().SetValue("KIZ")
	shopLicenceFunction.Func.TransferBaseTokens(1).Post()
	require.NoError(t, ctx.Err)

	// get open shop request from shopOwner (expected 2)
	getOpenShopView := russfest.ScFuncs.GetOpenShopRequest(ctx)
	getOpenShopView.Params.ShopOwner().SetValue(shopOwner.ScAgentID())
	getOpenShopView.Func.Call()
	require.EqualValues(t, 2, getOpenShopView.Results.OpenShopRequest().Length())
	require.NoError(t, ctx.Err)

	// get open shop request from russHanemann (expected 0)
	getOpenShopView.Params.ShopOwner().SetValue(russHannemann.ScAgentID())
	getOpenShopView.Func.Call()
	require.EqualValues(t, 0, getOpenShopView.Results.OpenShopRequest().Length())
	require.NoError(t, ctx.Err)

	// accept a shop that doesn't exist
	acceptShopFunc := russfest.ScFuncs.AcceptShop(ctx.Sign(russHannemann))
	acceptShopFunc.Params.ShopName().SetValue("FakeShop")
	acceptShopFunc.Func.TransferBaseTokens(1).Post()
	require.Error(t, ctx.Err)

	// accept a real shop
	acceptShopFunc.Params.ShopName().SetValue("ALLIGATOAH SHOP")
	acceptShopFunc.Func.TransferBaseTokens(1).Post()
	require.NoError(t, ctx.Err)

	getMusiciansWithoutShopFunc := russfest.ScFuncs.GetMusiciansWithoutShop(ctx)
	getMusiciansWithoutShopFunc.Func.Call()
	require.EqualValues(t, 1, getMusiciansWithoutShopFunc.Results.MusiciansWithoutShop().Length())
	require.NoError(t, ctx.Err)

	// get all open shop request (expected 1)
	getAllOpenShopView := russfest.ScFuncs.GetAllOpenShopRequests(ctx)
	getAllOpenShopView.Func.Call()
	require.EqualValues(t, 1, getAllOpenShopView.Results.OpenShopRequest().Length())
	require.NoError(t, ctx.Err)

	// reject KIZ shop
	rejectShopFunc := russfest.ScFuncs.DenyShop(ctx)
	rejectShopFunc.Params.ShopName().SetValue("KIZ SHOP")
	rejectShopFunc.Func.TransferBaseTokens(1).Post()
	require.NoError(t, ctx.Err)

	// get all open shop request from shopOwner (expected 0)
	getOpenShopView.Params.ShopOwner().SetValue(shopOwner.ScAgentID())
	getOpenShopView.Func.Call()
	require.EqualValues(t, 0, getOpenShopView.Results.OpenShopRequest().Length())
	require.NoError(t, ctx.Err)

	// getallDeniedShopRequests
	getDeniedShopView := russfest.ScFuncs.GetDeniedShopRequests(ctx)
	getDeniedShopView.Params.ShopOwner().SetValue(shopOwner.ScAgentID())
	getDeniedShopView.Func.Call()
	require.EqualValues(t, 1, getDeniedShopView.Results.DeniedShopRequests().Length())
	require.NoError(t, ctx.Err)

	//cancel denied request
	cancelDeniedRequest := russfest.ScFuncs.CancelShopRequest(ctx.Sign(shopOwner))
	cancelDeniedRequest.Params.Name().SetValue("KIZ SHOP")
	cancelDeniedRequest.Func.TransferBaseTokens(1).Post()
	require.NoError(t, ctx.Err)

	// getallDeniedShopRequests
	getDeniedShopView.Params.ShopOwner().SetValue(shopOwner.ScAgentID())
	getDeniedShopView.Func.Call()
	require.NoError(t, ctx.Err)

	// register KIZ SHOP anew, reject it, make a new offer
	ctx.Sign(shopOwner)
	shopLicenceFunction.Func.TransferBaseTokens(1).Post()
	require.NoError(t, ctx.Err)

	rejectShopFunc.Func.TransferBaseTokens(1).Post()
	require.NoError(t, ctx.Err)

	getDeniedShopView.Params.ShopOwner().SetValue(shopOwner.ScAgentID())
	getDeniedShopView.Func.Call()
	require.EqualValues(t, 1, getDeniedShopView.Results.DeniedShopRequests().Length())

	updateDeniedRequestFunc := russfest.ScFuncs.UpdateDeniedShopRequest(ctx.Sign(shopOwner))
	updateDeniedRequestFunc.Params.Newfee().SetValue(50)
	updateDeniedRequestFunc.Params.ShopName().SetValue("KIZ SHOP")
	updateDeniedRequestFunc.Func.TransferBaseTokens(1).Post()

	// view open request again (expected 1)
	getOpenShopView.Func.Call()
	require.EqualValues(t, 1, getOpenShopView.Results.OpenShopRequest().Length())
	require.EqualValues(t, 50, getOpenShopView.Results.OpenShopRequest().GetShop(0).Value().Fee)

	// accept KIZ ShopRequest
	ctx.Sign(russHannemann)
	acceptShopFunc.Params.ShopName().SetValue("KIZ SHOP")
	acceptShopFunc.Func.TransferBaseTokens(1).Post()
	require.NoError(t, ctx.Err)

	// test to call another SC

	createNewProductFunc := russfest.ScFuncs.CallCreateNewProduct(ctx)
	ctx.Sign(shopOwner)
	createNewProductFunc.Params.MusicianName().SetValue("Alligatoah")
	createNewProductFunc.Params.Price().SetValue(10)
	createNewProductFunc.Params.ProductType().SetValue("T-SHIRT")
	createNewProductFunc.Params.ShopName().SetValue("ALLIGATOAH SHOP")
	createNewProductFunc.Func.TransferBaseTokens(1)
	createNewProductFunc.Func.Post()
	require.NoError(t, ctx2.Err)
	require.NoError(t, ctx.Err)

	// get all Products known to russfest
	getAllProductsView := russfest.ScFuncs.GetAllProducts(ctx)
	getAllProductsView.Func.Call()
	require.EqualValues(t, 1, getAllProductsView.Results.Products().Length())
	require.EqualValues(t, 0, getAllProductsView.Results.Products().GetProduct(0).Value().Stock)
	require.NoError(t, ctx2.Err)
	require.NoError(t, ctx.Err)

	// get all Prodcuts knwon to russfest from specific shop
	getSpecificProduct := russfest.ScFuncs.GetSpecificProducts(ctx)
	getSpecificProduct.Params.ShopName().SetValue("ALLIGATOAH SHOP")
	getSpecificProduct.Func.Call()
	require.EqualValues(t, 1, getSpecificProduct.Results.Products().Length())
	require.NoError(t, ctx.Err)

	// get all producing products from shop1
	getAllProductsInProduction := shop1.ScFuncs.GetProductsInProduction(ctx2)
	getAllProductsInProduction.Func.Call()
	require.EqualValues(t, 1, getAllProductsInProduction.Results.TotalProductsInProduction().Value())
	require.EqualValues(t, 0, getAllProductsInProduction.Results.ProductsInProduction().GetStatisticProduct(0).Value().ProductionStep)
	require.NoError(t, ctx2.Err)
	require.NoError(t, ctx.Err)

	// get all earnings from shop1
	getEarningsView := shop1.ScFuncs.GetEarnings(ctx2)
	getEarningsView.Func.Call()
	require.EqualValues(t, 0, getEarningsView.Results.TotalEarnings().Value())
	require.NoError(t, ctx2.Err)
	require.NoError(t, ctx.Err)

	startProduction := shop1.ScFuncs.StartProduction(ctx2.Sign(shopOwner))
	startProduction.Func.TransferBaseTokens(1)
	startProduction.Func.Post()
	require.NoError(t, ctx2.Err)
	ctx.AdvanceClockBy(1 * time.Second)
	require.True(t, ctx2.WaitForPendingRequests(1))
	require.NoError(t, ctx2.Err)
	require.NoError(t, ctx.Err)

	// get all producing products from shop1 again
	getAllProductsInProduction.Func.Call()
	require.EqualValues(t, 2, getAllProductsInProduction.Results.TotalProductsInProduction().Value())
	require.EqualValues(t, 1, getAllProductsInProduction.Results.ProductsInProduction().GetStatisticProduct(0).Value().ProductionStep)
	require.NoError(t, ctx2.Err)
	require.NoError(t, ctx.Err)

	// let 3 production cycle pass

	startProduction.Func.Post()
	ctx.AdvanceClockBy(1 * time.Second)
	require.True(t, ctx2.WaitForPendingRequests(1))
	require.NoError(t, ctx2.Err)
	require.NoError(t, ctx.Err)
	require.EqualValues(t, 2, getAllProductsInProduction.Results.TotalProductsInProduction().Value())

	ctx.AdvanceClockBy(1 * time.Second)
	startProduction.Func.Post()
	require.True(t, ctx2.WaitForPendingRequests(1))
	require.EqualValues(t, 2, getAllProductsInProduction.Results.TotalProductsInProduction().Value())

	startProduction.Func.Post()
	require.True(t, ctx2.WaitForPendingRequests(1))
	require.NoError(t, ctx2.Err)
	require.NoError(t, ctx.Err)

	getAllProductsInProduction.Func.Call()
	require.EqualValues(t, 4, getAllProductsInProduction.Results.TotalProductsInProduction().Value())
	require.EqualValues(t, 4, getAllProductsInProduction.Results.ProductsInProduction().Length())
	require.EqualValues(t, 3, getAllProductsInProduction.Results.ProductsInProduction().GetStatisticProduct(0).Value().ProductionStep)
	require.NoError(t, ctx2.Err)
	require.NoError(t, ctx.Err)

	getAllProducedProductsView := shop1.ScFuncs.GetProducedProducts(ctx2)
	getAllProducedProductsView.Func.Call()
	require.EqualValues(t, 1, getAllProducedProductsView.Results.TotalProducedProducts().Value())
	require.EqualValues(t, 1, getAllProducedProductsView.Results.ProducedProductsPerShop().GetUint64("ALLIGATOAH SHOP").Value())
	require.NoError(t, ctx2.Err)
	require.NoError(t, ctx.Err)

	getShopStatisticView := russfest.ScFuncs.GetShopStatistics(ctx)
	getShopStatisticView.Params.ShopName().SetValue("ALLIGATOAH SHOP")
	getShopStatisticView.Func.Call()
	require.NoError(t, ctx.Err)
	require.EqualValues(t, 3, getShopStatisticView.Results.MaxProductionsStep().Value())
	require.EqualValues(t, 1, getShopStatisticView.Results.ProductionTemplates().Length())
	require.EqualValues(t, 4, getShopStatisticView.Results.Production().Length())
	require.EqualValues(t, "Alligatoah", getShopStatisticView.Results.Production().GetStatisticProduct(0).Value().Musician)
	require.NoError(t, ctx2.Err)
	require.NoError(t, ctx.Err)

	getAllProductsView.Func.Call()
	require.EqualValues(t, 1, getAllProductsView.Results.Products().Length())
	require.EqualValues(t, 1, getAllProductsView.Results.Products().GetProduct(0).Value().Stock)

	startProduction2 := shop1.ScFuncs.StartProduction(ctx2.Sign(shopOwner))
	startProduction2.Func.TransferBaseTokens(1)
	startProduction2.Func.Post()
	require.NoError(t, ctx2.Err)
	ctx.AdvanceClockBy(1 * time.Second)
	require.True(t, ctx2.WaitForPendingRequests(1))
	require.NoError(t, ctx2.Err)
	require.NoError(t, ctx.Err)

	getAllProductsView2 := russfest.ScFuncs.GetAllProducts(ctx)
	getAllProductsView2.Func.Call()
	require.EqualValues(t, 1, getAllProductsView2.Results.Products().Length())
	require.EqualValues(t, 2, getAllProductsView2.Results.Products().GetProduct(0).Value().Stock)
	require.NoError(t, ctx2.Err)
	require.NoError(t, ctx.Err)

	// buy merch
	buyMerchFunc := russfest.ScFuncs.BuyMerch(ctx)
	buyMerchFunc.Params.Musician().SetValue("Alligatoah")
	buyMerchFunc.Params.ProductType().SetValue("T-SHIRT")
	buyMerchFunc.Params.ShopName().SetValue("ALLIGATOAH SHOP")
	buyMerchFunc.Func.TransferBaseTokens(10).Post()
	require.NoError(t, ctx2.Err)
	require.NoError(t, ctx.Err)

	russfestEarnedMoney := russfest.ScFuncs.GetFestivalEarnings(ctx)
	russfestEarnedMoney.Func.Call()
	require.EqualValues(t, 2, russfestEarnedMoney.Results.FestivalEarnings().Value())
	require.NoError(t, ctx.Err)
	require.NoError(t, ctx2.Err)

	getShopStatisticView.Params.ShopName().SetValue("ALLIGATOAH SHOP")
	getShopStatisticView.Func.Call()
	require.NoError(t, ctx.Err)
	require.NoError(t, ctx2.Err)

	require.EqualValues(t, 3, getShopStatisticView.Results.MaxProductionsStep().Value())
	require.EqualValues(t, 1, getShopStatisticView.Results.ProductionTemplates().Length())
	require.EqualValues(t, 4, getShopStatisticView.Results.Production().Length())
	require.EqualValues(t, 8, getShopStatisticView.Results.Earnings().Value())
	require.EqualValues(t, 1, getShopStatisticView.Results.SoldProducts().Value())
}

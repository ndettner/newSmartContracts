package test

import (
	"testing"

	"github.com/stretchr/testify/require"

	"???ot/newSmartContracts/shop1/go/shop1"
	"???ot/newSmartContracts/shop1/go/shop1impl"
	"github.com/iotaledger/wasp/packages/wasmvm/wasmsolo"
)

func TestDeploy(t *testing.T) {
	ctx := wasmsolo.NewSoloContext(t, shop1.ScName, shop1impl.OnDispatch)
	require.NoError(t, ctx.ContractExists(shop1.ScName))
}

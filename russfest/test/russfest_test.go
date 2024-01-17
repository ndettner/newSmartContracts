package test

import (
	"testing"

	"github.com/stretchr/testify/require"

	"???ot/newSmartContracts/russfest/go/russfest"
	"???ot/newSmartContracts/russfest/go/russfestimpl"
	"github.com/iotaledger/wasp/packages/wasmvm/wasmsolo"
)

func TestDeploy(t *testing.T) {
	ctx := wasmsolo.NewSoloContext(t, russfest.ScName, russfestimpl.OnDispatch)
	require.NoError(t, ctx.ContractExists(russfest.ScName))
}

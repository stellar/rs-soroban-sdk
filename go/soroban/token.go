package soroban

// TokenClient is a client for calling token contracts that implement the
// token interface (SEP-41), including the Stellar Asset Contract. It
// corresponds to token::TokenClient in the Rust SDK.
type TokenClient struct {
	Address Address
}

// NewTokenClient returns a client for the token contract at the address.
func NewTokenClient(address Address) TokenClient { return TokenClient{Address: address} }

func (c TokenClient) call(fn string, args ...Val) Val {
	return InvokeContract[Val](c.Address, SymbolFromStr(fn), VecFromVals(args...))
}

// Allowance returns the allowance for spender to transfer from from.
func (c TokenClient) Allowance(from, spender Address) I128 {
	return FromVal[I128](c.call("allowance", from.ToVal(), spender.ToVal()))
}

// Approve sets the amount spender is allowed to transfer from from, until
// the ledger liveUntilLedger.
func (c TokenClient) Approve(from, spender Address, amount I128, liveUntilLedger uint32) {
	c.call("approve", from.ToVal(), spender.ToVal(), amount.ToVal(), valFromU32(liveUntilLedger))
}

// Balance returns the balance of id.
func (c TokenClient) Balance(id Address) I128 {
	return FromVal[I128](c.call("balance", id.ToVal()))
}

// Transfer transfers amount from from to to.
func (c TokenClient) Transfer(from Address, to MuxedAddress, amount I128) {
	c.call("transfer", from.ToVal(), to.ToVal(), amount.ToVal())
}

// TransferFrom transfers amount from from to to, consuming the allowance of
// spender.
func (c TokenClient) TransferFrom(spender, from, to Address, amount I128) {
	c.call("transfer_from", spender.ToVal(), from.ToVal(), to.ToVal(), amount.ToVal())
}

// Burn burns amount from from.
func (c TokenClient) Burn(from Address, amount I128) {
	c.call("burn", from.ToVal(), amount.ToVal())
}

// BurnFrom burns amount from from, consuming the allowance of spender.
func (c TokenClient) BurnFrom(spender, from Address, amount I128) {
	c.call("burn_from", spender.ToVal(), from.ToVal(), amount.ToVal())
}

// Decimals returns the number of decimals of the token.
func (c TokenClient) Decimals() uint32 { return FromVal[uint32](c.call("decimals")) }

// Name returns the name of the token.
func (c TokenClient) Name() String { return FromVal[String](c.call("name")) }

// Symbol returns the symbol of the token.
func (c TokenClient) Symbol() String { return FromVal[String](c.call("symbol")) }

// StellarAssetClient is a client for the admin interface of the Stellar Asset
// Contract. It corresponds to token::StellarAssetClient in the Rust SDK.
type StellarAssetClient struct {
	TokenClient
}

// NewStellarAssetClient returns a client for the Stellar Asset Contract at
// the address.
func NewStellarAssetClient(address Address) StellarAssetClient {
	return StellarAssetClient{TokenClient{Address: address}}
}

// SetAdmin sets the administrator.
func (c StellarAssetClient) SetAdmin(newAdmin Address) { c.call("set_admin", newAdmin.ToVal()) }

// Admin returns the administrator.
func (c StellarAssetClient) Admin() Address { return FromVal[Address](c.call("admin")) }

// SetAuthorized sets whether id is authorized to use its balance.
func (c StellarAssetClient) SetAuthorized(id Address, authorize bool) {
	c.call("set_authorized", id.ToVal(), valFromBool(authorize))
}

// Authorized reports whether id is authorized to use its balance.
func (c StellarAssetClient) Authorized(id Address) bool {
	return FromVal[bool](c.call("authorized", id.ToVal()))
}

// Mint mints amount to to.
func (c StellarAssetClient) Mint(to Address, amount I128) { c.call("mint", to.ToVal(), amount.ToVal()) }

// Clawback claws back amount from from.
func (c StellarAssetClient) Clawback(from Address, amount I128) {
	c.call("clawback", from.ToVal(), amount.ToVal())
}

// Trust creates a trustline for addr to the asset.
func (c StellarAssetClient) Trust(addr Address) { c.call("trust", addr.ToVal()) }

package soroban

import "github.com/stellar/rs-soroban-sdk/go/soroban/internal/host"

// Deployer provides deploying contracts and updating the current contract.
type Deployer struct{}

// WithCurrentContract returns a deployer for contracts deployed by the current
// contract with the salt.
func (Deployer) WithCurrentContract(salt BytesN[N32]) DeployerWithAddress {
	return DeployerWithAddress{address: Env{}.CurrentContractAddress(), salt: salt}
}

// WithAddress returns a deployer for contracts deployed by the address with
// the salt. The address must authorize the deployment.
func (Deployer) WithAddress(address Address, salt BytesN[N32]) DeployerWithAddress {
	return DeployerWithAddress{address: address, salt: salt}
}

// WithStellarAsset returns a deployer for the Stellar Asset Contract of the
// XDR serialized Asset.
func (Deployer) WithStellarAsset(serializedAsset Bytes) DeployerWithAsset {
	return DeployerWithAsset{asset: serializedAsset}
}

// UploadContractWasm uploads the contract Wasm and returns its hash.
func (Deployer) UploadContractWasm(wasm Bytes) BytesN[N32] {
	return bytesNFrom[N32](host.UploadWasm(wasm.handle()))
}

// UpdateCurrentContractWasm replaces the current contract's executable with
// the uploaded Wasm of the hash.
func (Deployer) UpdateCurrentContractWasm(wasmHash BytesN[N32]) {
	host.UpdateCurrentContractWasm(wasmHash.handle())
}

// ExtendTTL extends the TTL of the contract instance and code.
func (Deployer) ExtendTTL(contract Address, threshold, extendTo uint32) {
	host.ExtendContractInstanceAndCodeTtl(uint64(contract.ToVal()), u32Val(threshold), u32Val(extendTo))
}

// ExtendTTLForContractInstance extends the TTL of the contract instance.
func (Deployer) ExtendTTLForContractInstance(contract Address, threshold, extendTo uint32) {
	host.ExtendContractInstanceTtl(uint64(contract.ToVal()), u32Val(threshold), u32Val(extendTo))
}

// ExtendTTLForCode extends the TTL of the contract code.
func (Deployer) ExtendTTLForCode(contract Address, threshold, extendTo uint32) {
	host.ExtendContractCodeTtl(uint64(contract.ToVal()), u32Val(threshold), u32Val(extendTo))
}

// ExtendTTLWithLimits extends the TTL of the contract instance and code
// towards extendTo within the limits.
func (Deployer) ExtendTTLWithLimits(contract Address, extendTo, minExtension, maxExtension uint32) {
	host.ExtendContractInstanceAndCodeTtlV2(uint64(contract.ToVal()), ttlExtensionInstanceAndCode, u32Val(extendTo), u32Val(minExtension), u32Val(maxExtension))
}

// ExtendTTLForContractInstanceWithLimits extends the TTL of the contract
// instance towards extendTo within the limits.
func (Deployer) ExtendTTLForContractInstanceWithLimits(contract Address, extendTo, minExtension, maxExtension uint32) {
	host.ExtendContractInstanceAndCodeTtlV2(uint64(contract.ToVal()), ttlExtensionInstance, u32Val(extendTo), u32Val(minExtension), u32Val(maxExtension))
}

// ExtendTTLForCodeWithLimits extends the TTL of the contract code towards
// extendTo within the limits.
func (Deployer) ExtendTTLForCodeWithLimits(contract Address, extendTo, minExtension, maxExtension uint32) {
	host.ExtendContractInstanceAndCodeTtlV2(uint64(contract.ToVal()), ttlExtensionCode, u32Val(extendTo), u32Val(minExtension), u32Val(maxExtension))
}

// DeployerWithAddress deploys contracts at an address derived from a deployer
// address and salt.
type DeployerWithAddress struct {
	address Address
	salt    BytesN[N32]
}

// DeployedAddress returns the address the contract will be deployed to.
func (d DeployerWithAddress) DeployedAddress() Address {
	return Address{obj: Val(host.GetContractId(uint64(d.address.ToVal()), d.salt.handle()))}
}

// Deploy deploys the Wasm of the hash, with no constructor arguments.
func (d DeployerWithAddress) Deploy(wasmHash BytesN[N32]) Address {
	return d.DeployV2(wasmHash, NewVec[Val]())
}

// DeployV2 deploys the Wasm of the hash, calling its constructor with the
// args.
func (d DeployerWithAddress) DeployV2(wasmHash BytesN[N32], constructorArgs Vec[Val]) Address {
	return Address{obj: Val(host.CreateContractWithConstructor(uint64(d.address.ToVal()), wasmHash.handle(), d.salt.handle(), constructorArgs.handle()))}
}

// DeployerWithAsset deploys a Stellar Asset Contract.
type DeployerWithAsset struct {
	asset Bytes
}

// DeployedAddress returns the address the asset contract will be deployed to.
func (d DeployerWithAsset) DeployedAddress() Address {
	return Address{obj: Val(host.GetAssetContractId(d.asset.handle()))}
}

// Deploy deploys the Stellar Asset Contract.
func (d DeployerWithAsset) Deploy() Address {
	return Address{obj: Val(host.CreateAssetContract(d.asset.handle()))}
}

// Prng provides a pseudo-random number generator, seeded by the host for each
// invocation. It is not suitable for security sensitive uses.
type Prng struct{}

// Seed reseeds the generator from a 32 byte seed.
func (Prng) Seed(seed Bytes) { host.PrngReseed(seed.handle()) }

// Bytes returns n random bytes.
func (Prng) Bytes(n uint32) Bytes { return Bytes{obj: Val(host.PrngBytesNew(u32Val(n)))} }

// U64 returns a random uint64.
func (p Prng) U64() uint64 { return p.U64InRange(0, ^uint64(0)) }

// U64InRange returns a random uint64 in the inclusive range [lo, hi].
func (Prng) U64InRange(lo, hi uint64) uint64 { return host.PrngU64InInclusiveRange(lo, hi) }

// Shuffle returns the Vec shuffled.
func Shuffle[T any](v Vec[T]) Vec[T] { return Vec[T]{obj: Val(host.PrngVecShuffle(v.handle()))} }

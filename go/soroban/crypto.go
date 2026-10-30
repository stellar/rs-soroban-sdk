package soroban

import "github.com/stellar/rs-soroban-sdk/go/soroban/internal/host"

// Hash is the output of a secure cryptographic hash function. It corresponds
// to Hash<32> in the Rust SDK and is a BytesN[N32] in the contract interface.
type Hash = BytesN[N32]

// Crypto provides cryptographic functions.
type Crypto struct{}

// Sha256 returns the SHA-256 hash of the data.
func (Crypto) Sha256(data Bytes) Hash {
	return Hash{Bytes: Bytes{obj: Val(host.ComputeHashSha256(data.handle()))}}
}

// Keccak256 returns the Keccak-256 hash of the data.
func (Crypto) Keccak256(data Bytes) Hash {
	return Hash{Bytes: Bytes{obj: Val(host.ComputeHashKeccak256(data.handle()))}}
}

// Ed25519Verify verifies the ed25519 signature of the message by the public
// key, trapping if the signature is invalid.
func (Crypto) Ed25519Verify(publicKey BytesN[N32], message Bytes, signature BytesN[N64]) {
	host.VerifySigEd25519(publicKey.handle(), message.handle(), signature.handle())
}

// Secp256k1Recover recovers the 65-byte SEC-1 uncompressed ECDSA secp256k1
// public key that produced the signature over the message digest.
func (Crypto) Secp256k1Recover(digest Hash, signature BytesN[N64], recoveryID uint32) BytesN[N65] {
	return BytesN[N65]{Bytes: Bytes{obj: Val(host.RecoverKeyEcdsaSecp256k1(digest.handle(), signature.handle(), u32Val(recoveryID)))}}
}

// Secp256r1Verify verifies the ECDSA secp256r1 signature over the message
// digest by the 65-byte SEC-1 uncompressed public key, trapping if invalid.
func (Crypto) Secp256r1Verify(publicKey BytesN[N65], digest Hash, signature BytesN[N64]) {
	host.VerifySigEcdsaSecp256r1(publicKey.handle(), digest.handle(), signature.handle())
}

// MlDsa44Verify verifies an ML-DSA-44 signature with a context, trapping if
// invalid. Requires protocol 30.
func (Crypto) MlDsa44Verify(publicKey, message, signature, context Bytes) {
	host.VerifySigMlDsa44(publicKey.handle(), message.handle(), signature.handle(), context.handle())
}

// MlDsa65Verify verifies an ML-DSA-65 signature with a context, trapping if
// invalid. Requires protocol 30.
func (Crypto) MlDsa65Verify(publicKey, message, signature, context Bytes) {
	host.VerifySigMlDsa65(publicKey.handle(), message.handle(), signature.handle(), context.handle())
}

// MlDsa87Verify verifies an ML-DSA-87 signature with a context, trapping if
// invalid. Requires protocol 30.
func (Crypto) MlDsa87Verify(publicKey, message, signature, context Bytes) {
	host.VerifySigMlDsa87(publicKey.handle(), message.handle(), signature.handle(), context.handle())
}

// Bls12381 returns access to BLS12-381 operations.
func (Crypto) Bls12381() Bls12381 { return Bls12381{} }

// Bn254 returns access to BN254 operations.
func (Crypto) Bn254() Bn254 { return Bn254{} }

// Point and field element types for BLS12-381, as their uncompressed
// big-endian serializations.
type (
	G1Affine = BytesN[N96]
	G2Affine = BytesN[N192]
	Fp       = BytesN[N48]
	Fp2      = BytesN[N96]
	Fr       = U256
)

// Bls12381 provides BLS12-381 curve operations.
type Bls12381 struct{}

func bytesNFrom[L Len](raw uint64) BytesN[L] { return BytesN[L]{Bytes: Bytes{obj: Val(raw)}} }

// G1IsInSubgroup reports whether the point is in the G1 subgroup.
func (Bls12381) G1IsInSubgroup(p G1Affine) bool {
	return boolFromRaw(host.Bls12381CheckG1IsInSubgroup(p.handle()))
}

// G1IsOnCurve reports whether the point is on the G1 curve.
func (Bls12381) G1IsOnCurve(p G1Affine) bool {
	return boolFromRaw(host.Bls12381G1IsOnCurve(p.handle()))
}

// G1Add adds two G1 points.
func (Bls12381) G1Add(a, b G1Affine) G1Affine {
	return bytesNFrom[N96](host.Bls12381G1Add(a.handle(), b.handle()))
}

// G1Mul multiplies a G1 point by a scalar.
func (Bls12381) G1Mul(p G1Affine, s Fr) G1Affine {
	return bytesNFrom[N96](host.Bls12381G1Mul(p.handle(), uint64(s.ToVal())))
}

// G1Msm performs a multi-scalar multiplication on G1.
func (Bls12381) G1Msm(points Vec[G1Affine], scalars Vec[Fr]) G1Affine {
	return bytesNFrom[N96](host.Bls12381G1Msm(points.handle(), scalars.handle()))
}

// MapFpToG1 maps a field element to G1.
func (Bls12381) MapFpToG1(fp Fp) G1Affine {
	return bytesNFrom[N96](host.Bls12381MapFpToG1(fp.handle()))
}

// HashToG1 hashes a message to G1 with the domain separation tag.
func (Bls12381) HashToG1(msg, dst Bytes) G1Affine {
	return bytesNFrom[N96](host.Bls12381HashToG1(msg.handle(), dst.handle()))
}

// G2IsInSubgroup reports whether the point is in the G2 subgroup.
func (Bls12381) G2IsInSubgroup(p G2Affine) bool {
	return boolFromRaw(host.Bls12381CheckG2IsInSubgroup(p.handle()))
}

// G2IsOnCurve reports whether the point is on the G2 curve.
func (Bls12381) G2IsOnCurve(p G2Affine) bool {
	return boolFromRaw(host.Bls12381G2IsOnCurve(p.handle()))
}

// G2Add adds two G2 points.
func (Bls12381) G2Add(a, b G2Affine) G2Affine {
	return bytesNFrom[N192](host.Bls12381G2Add(a.handle(), b.handle()))
}

// G2Mul multiplies a G2 point by a scalar.
func (Bls12381) G2Mul(p G2Affine, s Fr) G2Affine {
	return bytesNFrom[N192](host.Bls12381G2Mul(p.handle(), uint64(s.ToVal())))
}

// G2Msm performs a multi-scalar multiplication on G2.
func (Bls12381) G2Msm(points Vec[G2Affine], scalars Vec[Fr]) G2Affine {
	return bytesNFrom[N192](host.Bls12381G2Msm(points.handle(), scalars.handle()))
}

// MapFp2ToG2 maps a field element to G2.
func (Bls12381) MapFp2ToG2(fp2 Fp2) G2Affine {
	return bytesNFrom[N192](host.Bls12381MapFp2ToG2(fp2.handle()))
}

// HashToG2 hashes a message to G2 with the domain separation tag.
func (Bls12381) HashToG2(msg, dst Bytes) G2Affine {
	return bytesNFrom[N192](host.Bls12381HashToG2(msg.handle(), dst.handle()))
}

// PairingCheck reports whether the product of the pairings of the points is
// the identity.
func (Bls12381) PairingCheck(g1 Vec[G1Affine], g2 Vec[G2Affine]) bool {
	return boolFromRaw(host.Bls12381MultiPairingCheck(g1.handle(), g2.handle()))
}

// FrAdd adds two scalars.
func (Bls12381) FrAdd(a, b Fr) Fr {
	return FromVal[Fr](Val(host.Bls12381FrAdd(uint64(a.ToVal()), uint64(b.ToVal()))))
}

// FrSub subtracts two scalars.
func (Bls12381) FrSub(a, b Fr) Fr {
	return FromVal[Fr](Val(host.Bls12381FrSub(uint64(a.ToVal()), uint64(b.ToVal()))))
}

// FrMul multiplies two scalars.
func (Bls12381) FrMul(a, b Fr) Fr {
	return FromVal[Fr](Val(host.Bls12381FrMul(uint64(a.ToVal()), uint64(b.ToVal()))))
}

// FrPow raises a scalar to a power.
func (Bls12381) FrPow(a Fr, e uint64) Fr {
	return FromVal[Fr](Val(host.Bls12381FrPow(uint64(a.ToVal()), uint64(valFromU64(e)))))
}

// FrInv inverts a scalar.
func (Bls12381) FrInv(a Fr) Fr { return FromVal[Fr](Val(host.Bls12381FrInv(uint64(a.ToVal())))) }

// Point types for BN254.
type (
	Bn254G1Affine = BytesN[N64]
	Bn254G2Affine = BytesN[N128]
)

// Bn254 provides BN254 curve operations.
type Bn254 struct{}

// G1Add adds two G1 points.
func (Bn254) G1Add(a, b Bn254G1Affine) Bn254G1Affine {
	return bytesNFrom[N64](host.Bn254G1Add(a.handle(), b.handle()))
}

// G1Mul multiplies a G1 point by a scalar.
func (Bn254) G1Mul(p Bn254G1Affine, s U256) Bn254G1Affine {
	return bytesNFrom[N64](host.Bn254G1Mul(p.handle(), uint64(s.ToVal())))
}

// G1Msm performs a multi-scalar multiplication on G1.
func (Bn254) G1Msm(points Vec[Bn254G1Affine], scalars Vec[U256]) Bn254G1Affine {
	return bytesNFrom[N64](host.Bn254G1Msm(points.handle(), scalars.handle()))
}

// G1IsOnCurve reports whether the point is on the G1 curve.
func (Bn254) G1IsOnCurve(p Bn254G1Affine) bool { return boolFromRaw(host.Bn254G1IsOnCurve(p.handle())) }

// PairingCheck reports whether the product of the pairings is the identity.
func (Bn254) PairingCheck(g1 Vec[Bn254G1Affine], g2 Vec[Bn254G2Affine]) bool {
	return boolFromRaw(host.Bn254MultiPairingCheck(g1.handle(), g2.handle()))
}

// FrAdd adds two scalars.
func (Bn254) FrAdd(a, b U256) U256 {
	return FromVal[U256](Val(host.Bn254FrAdd(uint64(a.ToVal()), uint64(b.ToVal()))))
}

// FrSub subtracts two scalars.
func (Bn254) FrSub(a, b U256) U256 {
	return FromVal[U256](Val(host.Bn254FrSub(uint64(a.ToVal()), uint64(b.ToVal()))))
}

// FrMul multiplies two scalars.
func (Bn254) FrMul(a, b U256) U256 {
	return FromVal[U256](Val(host.Bn254FrMul(uint64(a.ToVal()), uint64(b.ToVal()))))
}

// FrPow raises a scalar to a power.
func (Bn254) FrPow(a U256, e uint64) U256 {
	return FromVal[U256](Val(host.Bn254FrPow(uint64(a.ToVal()), uint64(valFromU64(e)))))
}

// FrInv inverts a scalar.
func (Bn254) FrInv(a U256) U256 { return FromVal[U256](Val(host.Bn254FrInv(uint64(a.ToVal())))) }

// PoseidonPermutation performs a Poseidon permutation (hazmat).
func (Crypto) PoseidonPermutation(input Vec[U256], field Symbol, t, d, roundsF, roundsP uint32, mds Vec[Vec[U256]], roundConstants Vec[Vec[U256]]) Vec[U256] {
	return Vec[U256]{obj: Val(host.PoseidonPermutation(input.handle(), uint64(field.ToVal()), u32Val(t), u32Val(d), u32Val(roundsF), u32Val(roundsP), mds.handle(), roundConstants.handle()))}
}

// Poseidon2Permutation performs a Poseidon2 permutation (hazmat).
func (Crypto) Poseidon2Permutation(input Vec[U256], field Symbol, t, d, roundsF, roundsP uint32, matInternalDiagM1 Vec[U256], roundConstants Vec[Vec[U256]]) Vec[U256] {
	return Vec[U256]{obj: Val(host.Poseidon2Permutation(input.handle(), uint64(field.ToVal()), u32Val(t), u32Val(d), u32Val(roundsF), u32Val(roundsP), matInternalDiagM1.handle(), roundConstants.handle()))}
}

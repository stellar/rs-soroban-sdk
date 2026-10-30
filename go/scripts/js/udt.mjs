// Invokes the test_udt contract with js-stellar-sdk, building the argument
// ScVals directly. stellar-cli can't be used for test_udt because it recurses
// without bound when parsing arguments of the contract's recursive types.
//
// Usage: node udt.mjs <rpc-url> <network-passphrase> <source-G-address> <contract-id>
import { rpc, Contract, TransactionBuilder, Account, xdr, scValToNative, nativeToScVal } from "@stellar/stellar-sdk";

const [rpcUrl, passphrase, source, contractId] = process.argv.slice(2);
const server = new rpc.Server(rpcUrl, { allowHttp: rpcUrl.startsWith("http://") });
const contract = new Contract(contractId);

const sym = (s) => xdr.ScVal.scvSymbol(s);
const vec = (...v) => xdr.ScVal.scvVec(v);
const i64 = (n) => nativeToScVal(n, { type: "i64" });
const u32 = (n) => xdr.ScVal.scvU32(n);
const map = (entries) =>
  xdr.ScVal.scvMap(entries.map(([k, v]) => new xdr.ScMapEntry({ key: k, val: v })));

// UdtStruct{a, b, c} is a map with symbol keys in sorted order.
const udtStruct = (a, b, c) => map([[sym("a"), i64(a)], [sym("b"), i64(b)], [sym("c"), vec(...c.map(i64))]]);
const UdtA = vec(sym("UdtA"));
const UdtB = (s) => vec(sym("UdtB"), s);
const UdtC = (e) => vec(sym("UdtC"), u32(e));
const UdtD = (n, v) => vec(sym("UdtD"), vec(i64(n), vec(...v.map(i64))));
const rec = (a, b) => map([[sym("a"), sym(a)], [sym("b"), vec(...b)]]);
const NotRecursive = vec(sym("NotRecursive"));
const Recursive = (a, m) => vec(sym("Recursive"), map([[sym("a"), sym(a)], [sym("b"), map(m)]]));

const account = new Account(source, "0");

async function call(fn, ...args) {
  const tx = new TransactionBuilder(account, { fee: "1000000", networkPassphrase: passphrase })
    .addOperation(contract.call(fn, ...args))
    .setTimeout(60)
    .build();
  const sim = await server.simulateTransaction(tx);
  if (rpc.Api.isSimulationError(sim)) throw new Error(sim.error);
  return scValToNative(sim.result.retval);
}

let pass = 0;
let fail = 0;
async function expect(desc, want, fn, ...args) {
  try {
    const got = JSON.stringify(await call(fn, ...args), (_, v) => (typeof v === "bigint" ? v.toString() : v));
    if (got === JSON.stringify(want)) {
      console.log(`  ok: ${desc} = ${got}`);
      pass++;
    } else {
      console.log(`  FAIL: ${desc}: got ${got} want ${JSON.stringify(want)}`);
      fail++;
    }
  } catch (e) {
    console.log(`  FAIL: ${desc}: ${String(e).slice(0, 300)}`);
    fail++;
  }
}

await expect("add(UdtA, UdtA)", "0", "add", UdtA, UdtA);
await expect("add(UdtB{1,2}, UdtC(B=15))", "18", "add", UdtB(udtStruct(1, 2, [])), UdtC(15));
await expect("add(UdtD(5,[3,3]), UdtC(A=10))", "21", "add", UdtD(5, [3, 3]), UdtC(10));
await expect("recursive(empty)", null, "recursive", rec("x", []));
await expect("recursive(nested)", { a: "y", b: [] }, "recursive", rec("x", [rec("y", [])]));
await expect("recursive_enum(NotRecursive)", null, "recursive_enum", NotRecursive, u32(1));
await expect("recursive_enum(Recursive)", ["NotRecursive"], "recursive_enum", Recursive("r", [[u32(1), NotRecursive]]), u32(1));
await expect("recursive_enum(Recursive, missing key)", null, "recursive_enum", Recursive("r", [[u32(1), NotRecursive]]), u32(2));
try {
  await call("add", vec(sym("Nope")), UdtA);
  console.log("  FAIL: add(unknown case) should fail");
  fail++;
} catch (e) {
  console.log("  ok: add(unknown case) fails");
  pass++;
}
try {
  await call("add", UdtC(11), UdtA);
  console.log("  FAIL: add(UdtC(11)) with an invalid enum value should fail");
  fail++;
} catch (e) {
  console.log("  ok: add(UdtC(11)) with an invalid enum value fails");
  pass++;
}
console.log(`udt passed: ${pass} failed: ${fail}`);
process.exit(fail ? 1 : 0);

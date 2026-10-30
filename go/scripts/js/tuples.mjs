// Invokes test_tuples' void_fn, which takes a () argument that stellar-cli has
// no syntax for.
//
// Usage: node tuples.mjs <rpc-url> <network-passphrase> <source-G-address> <contract-id>
import { rpc, Contract, TransactionBuilder, Account, xdr } from "@stellar/stellar-sdk";

const [rpcUrl, passphrase, source, contractId] = process.argv.slice(2);
const server = new rpc.Server(rpcUrl, { allowHttp: rpcUrl.startsWith("http://") });
const tx = new TransactionBuilder(new Account(source, "0"), { fee: "1000000", networkPassphrase: passphrase })
  .addOperation(new Contract(contractId).call("void_fn", xdr.ScVal.scvVoid()))
  .setTimeout(60)
  .build();
const sim = await server.simulateTransaction(tx);
if (rpc.Api.isSimulationError(sim)) {
  console.log(`  FAIL: void_fn(()): ${sim.error.slice(0, 300)}`);
  process.exit(1);
}
const kind = sim.result.retval.switch().name;
if (kind !== "scvVoid") {
  console.log(`  FAIL: void_fn(()) returned ${kind}`);
  process.exit(1);
}
console.log("  ok: void_fn(()) = ()");

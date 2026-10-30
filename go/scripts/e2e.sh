#!/usr/bin/env bash
# End-to-end test: builds the Go test contracts with soroban-go, deploys them
# with the stellar-cli to a network, and invokes their functions, checking the
# results, errors and events.
#
# Usage: scripts/e2e.sh [network]
#
# The network defaults to testnet. For a local network run quickstart:
#   docker run -d -p 8000:8000 stellar/quickstart --local
#   stellar network add local --rpc-url http://localhost:8000/rpc \
#     --network-passphrase "Standalone Network ; February 2017"
#   scripts/e2e.sh local
#
# Environment:
#   STELLAR   stellar-cli binary (default: stellar)
#   PROTOCOL  ledger protocol version to build for (default: the network's)
set -euo pipefail

# The stellar-cli recurses deeply when parsing arguments of recursive contract
# types, such as those in test_udt, and overflows the default stack.
ulimit -s unlimited 2>/dev/null || ulimit -s 65536 2>/dev/null || true

cd "$(dirname "$0")/.."
NETWORK="${1:-testnet}"
STELLAR="${STELLAR:-stellar}"
OUT="$(mktemp -d)"
trap 'rm -rf "$OUT"' EXIT

RPC="$("$STELLAR" network ls --long 2>/dev/null | awk -v n="$NETWORK" '$0 ~ "^Name: "n"$" {f=1} f && /RPC url/ {print $3; exit}')"
RPC="${RPC:-https://soroban-testnet.stellar.org}"
NETINFO="$(curl -sS -X POST "$RPC" -H 'content-type: application/json' -d '{"jsonrpc":"2.0","id":1,"method":"getNetwork"}')"
PASSPHRASE="$(echo "$NETINFO" | sed -E 's/.*"passphrase":"([^"]*)".*/\1/')"
if [ -z "${PROTOCOL:-}" ]; then
  PROTOCOL="$(echo "$NETINFO" | sed -E 's/.*"protocolVersion":([0-9]+).*/\1/')"
fi
echo "network=$NETWORK protocol=$PROTOCOL"

go build -o "$OUT/soroban-go" ./cmd/soroban-go

if ! "$STELLAR" keys address e2e >/dev/null 2>&1; then
  "$STELLAR" keys generate e2e --network "$NETWORK" --fund
fi
"$STELLAR" keys fund e2e --network "$NETWORK" >/dev/null 2>&1 || true
ME="$("$STELLAR" keys address e2e)"

PASS=0
FAIL=0
ok() { echo "  ok: $*"; PASS=$((PASS + 1)); }
bad() { echo "  FAIL: $*"; FAIL=$((FAIL + 1)); }

build() {
  "$OUT/soroban-go" build -protocol "$PROTOCOL" -o "$OUT/$1.wasm" "tests/$1" >&2
}

deploy() {
  local c="$1"
  shift
  build "$c"
  local i
  for i in 1 2 3 4; do
    if "$STELLAR" contract deploy --wasm "$OUT/$c.wasm" --source e2e --network "$NETWORK" "$@" 2>"$OUT/$c.deploy.log"; then
      return 0
    fi
    # Retry when an RPC node lags behind the upload, or other transient failures.
    grep -qE "Wasm does not exist|Request rejected .5[0-9][0-9].|timed out|connection" "$OUT/$c.deploy.log" || break
    sleep 5
  done
  cat "$OUT/$c.deploy.log" >&2
  return 1
}

# invoke <id> <send:yes|no> <fn> [args...]: prints the result; stderr (logs,
# events) is saved to $OUT/last.log.
invoke() {
  local id="$1" send="$2"
  shift 2
  local i
  for i in 1 2 3; do
    if "$STELLAR" contract invoke --id "$id" --source e2e --network "$NETWORK" --send="$send" -- "$@" 2>"$OUT/last.log"; then
      return 0
    fi
    # Retry only transient RPC failures.
    grep -qE "Request rejected .5[0-9][0-9].|timed out|connection" "$OUT/last.log" || return 1
    sleep 3
  done
  return 1
}

# expect <desc> <expected> <id> <send> <fn> [args...]
expect() {
  local desc="$1" want="$2"
  shift 2
  local got
  if got="$(invoke "$@")"; then
    if [ "$got" = "$want" ]; then ok "$desc = $got"; else bad "$desc: got '$got' want '$want'"; fi
  else
    bad "$desc: invocation failed: $(tail -3 "$OUT/last.log")"
  fi
}

# expect_error <desc> <pattern> <id> <send> <fn> [args...]
expect_error() {
  local desc="$1" pat="$2"
  shift 2
  if invoke "$@" >/dev/null; then
    bad "$desc: expected failure"
  elif grep -q -- "$pat" "$OUT/last.log"; then
    ok "$desc fails with $pat"
  else
    bad "$desc: failure did not match '$pat': $(tail -3 "$OUT/last.log")"
  fi
}

# expect_log <desc> <pattern>: checks the last invocation's logs.
expect_log() {
  if grep -q -- "$2" "$OUT/last.log"; then ok "$1"; else bad "$1: '$2' not in: $(cat "$OUT/last.log")"; fi
}

echo "== test_empty"
ID="$(deploy test_empty)"
expect "empty()" "" "$ID" no empty

echo "== test_add_u64"
ID="$(deploy test_add_u64)"
expect "add(2, 40)" "42" "$ID" no add --a 2 --b 40
expect "add(small max, 10)" "72057594037927945" "$ID" no add --a 72057594037927935 --b 10
expect "safe_add(1, 2)" "3" "$ID" no safe_add --a 1 --b 2
expect_error "safe_add(max, 1)" "Error(Contract, #1)" "$ID" no safe_add --a 18446744073709551615 --b 1
expect_error "safe_add_two(max, 1)" "Error(Contract, #1)" "$ID" no safe_add_two --a 18446744073709551615 --b 1

echo "== test_add_i128"
ID="$(deploy test_add_i128)"
expect "add(-5, 3)" '"-2"' "$ID" no add --a -5 --b 3
expect "add(2^64, 2^64)" '"36893488147419103232"' "$ID" no add --a 18446744073709551616 --b 18446744073709551616
expect_error "add(i128 max, 1) overflows" "Error(WasmVm, InvalidAction)" "$ID" no add --a 170141183460469231731687303715884105727 --b 1

echo "== test_udt"
ID="$(deploy test_udt)"
# stellar-cli recurses without bound parsing the args of test_udt's recursive
# types, so the contract is invoked with js-stellar-sdk instead.
if command -v node >/dev/null; then
  (cd scripts/js && { [ -d node_modules ] || npm install --silent; } &&
    node udt.mjs "$RPC" "$PASSPHRASE" "$ME" "$ID") && ok "test_udt (js-stellar-sdk)" || bad "test_udt (js-stellar-sdk)"
else
  echo "  skipped: node not installed"
fi

echo "== test_errors"
ID="$(deploy test_errors)"
expect "hello(A)" '"hello"' "$ID" yes hello --flag 0
expect_error "hello(B) returns error" "Error(Contract, #1)" "$ID" no hello --flag 1
expect_error "hello(C) panics with error" "Error(Contract, #1)" "$ID" no hello --flag 2
expect_error "hello(D) panics" "Error(WasmVm, InvalidAction)" "$ID" no hello --flag 3
expect_error "hello(E) panics with raw error" "Error(Contract, #9)" "$ID" no hello --flag 4

echo "== test_events"
ID="$(deploy test_events)"
invoke "$ID" yes transfer --from "$ME" --to "$ME" --amount 100 >/dev/null && ok "transfer sent" || bad "transfer: $(tail -3 "$OUT/last.log")"
expect_log "transfer event topics" "transfer"
invoke "$ID" yes single_value --from "$ME" --amount 7 >/dev/null && ok "single_value sent" || bad "single_value"
expect_log "single_value event" "single_value"
invoke "$ID" yes vec_values --from "$ME" --a 1 --b 2 >/dev/null && ok "vec_values sent" || bad "vec_values"
expect_log "vec_values event" "vec_values"
invoke "$ID" yes map_values --from "$ME" --a 1 --b 2 >/dev/null && ok "map_values sent" || bad "map_values"
expect_log "map_values event" "map_values"
invoke "$ID" yes single_value_void --from "$ME" >/dev/null && ok "single_value_void sent" || bad "single_value_void"
expect_error "failed_transfer panics" "Error(WasmVm, InvalidAction)" "$ID" no failed_transfer --from "$ME" --to "$ME" --amount 1

echo "== test_contract_data"
ID="$(deploy test_contract_data)"
expect "get(missing)" "null" "$ID" no get --key k
invoke "$ID" yes put --key k --val v >/dev/null && ok "put(k, v)" || bad "put: $(tail -3 "$OUT/last.log")"
expect "get(k)" '"v"' "$ID" no get --key k
invoke "$ID" yes del --key k >/dev/null && ok "del(k)" || bad "del"
expect "get(k) after del" "null" "$ID" no get --key k

echo "== test_constructor"
ID="$(deploy test_constructor -- --init_key 100 --init_value 1000)"
expect "get_data(Persistent(100))" "1000" "$ID" no get_data --key '{"Persistent":100}'
expect "get_data(Temp(200))" "2000" "$ID" no get_data --key '{"Temp":200}'
expect "get_data(Instance(300))" "3000" "$ID" no get_data --key '{"Instance":300}'
expect "get_data(Persistent(10))" "null" "$ID" no get_data --key '{"Persistent":10}'

echo "== test_logging"
ID="$(deploy test_logging)"
invoke "$ID" no hello >/dev/null && ok "hello()" || bad "hello: $(tail -3 "$OUT/last.log")"
expect_log "log one and two" "one and two"

echo "== test_tuples"
ID="$(deploy test_tuples)"
expect "tuple1([7])" "[7]" "$ID" no tuple1 --arg '[7]'
expect "tuple2([7, -8])" '[7,-8]' "$ID" no tuple2 --arg '[7,-8]'
if command -v node >/dev/null; then
  (cd scripts/js && node tuples.mjs "$RPC" "$PASSPHRASE" "$ME" "$ID") && ok "void_fn (js-stellar-sdk)" || bad "void_fn (js-stellar-sdk)"
fi

echo "== test_invoke_contract"
# Like the Rust test, one instance calls the add function of another, as a
# contract can't re-enter itself.
ID="$(deploy test_invoke_contract)"
ADD_ID="$(deploy test_invoke_contract)"
expect "add_with(3, 4, other)" "7" "$ID" no add_with --x 3 --y 4 --contract_id "$ADD_ID"

echo "== test_auth"
ID="$(deploy test_auth)"
SUB_ID="$(deploy test_auth)"
expect "fn1(me)" "2" "$ID" yes fn1 --a "$ME"
expect "fn2(me, other) requires auth in a sub-invocation" "2" "$ID" yes fn2 --a "$ME" --sub "$SUB_ID"

echo "== test_account"
ID="$(deploy test_account)" && ok "deployed custom account $ID" || bad "deploy test_account"

echo "== test_sdk"
ID="$(deploy test_sdk)"
# expect_match <desc> <regex> <id> <send> <fn> [args...]
expect_match() {
  local desc="$1" pat="$2"
  shift 2
  local got
  if got="$(invoke "$@")"; then
    if echo "$got" | grep -qE -- "$pat"; then ok "$desc = $got"; else bad "$desc: got '$got' want /$pat/"; fi
  else
    bad "$desc: invocation failed: $(tail -3 "$OUT/last.log")"
  fi
}
expect_match "ledger()" "\"protocol\":$PROTOCOL" "$ID" no ledger
expect "current_address()" "\"$ID\"" "$ID" no current_address
expect "sha256(abc)" '"ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"' "$ID" no sha256 --data 616263
expect "keccak256(abc)" '"4e03657aea45a94fc7d47ba826c8d667c0d1e6e33a64a036ec44f58fa12d6c45"' "$ID" no keccak256 --data 616263
# RFC 8032 test vector 2.
expect "ed25519_verify(valid)" "true" "$ID" no ed25519_verify \
  --public_key 3d4017c3e843895a92b70aa74d1b7ebc9c982ccf2ec4968cc0cd55f12af4660c --message 72 \
  --signature 92a009a9f0d4cab8720e820b5f642540a2b27b5416503f8fb3762223ebdb69da085ac1e43e15996e458f3613d0f11d8c387b2eaeb4302aeeb00d291612bb0c00
expect_error "ed25519_verify(invalid)" "Error(Crypto" "$ID" no ed25519_verify \
  --public_key 3d4017c3e843895a92b70aa74d1b7ebc9c982ccf2ec4968cc0cd55f12af4660c --message 73 \
  --signature 92a009a9f0d4cab8720e820b5f642540a2b27b5416503f8fb3762223ebdb69da085ac1e43e15996e458f3613d0f11d8c387b2eaeb4302aeeb00d291612bb0c00
expect_match "random(10, 12)" "^1[012]$" "$ID" no random --lo 10 --hi 12
expect "shuffle_sum([1..5])" "15" "$ID" no shuffle_sum --v '[1,2,3,4,5]'
expect "histogram([a,b,a])" '{"a":2,"b":1}' "$ID" no histogram --words '["a","b","a"]'
expect "bytes_ops(010203)" '"03020103"' "$ID" no bytes_ops --b 010203
expect "greet(Go)" '"Hello, Go!"' "$ID" no greet --name Go
expect "u256_math(2^128, 3)" '"1361129467683753853853498429727072845824"' "$ID" no u256_math --a 340282366920938463463374607431768211456 --b 3
expect "i128_math(-7, 2)" '["-14","-3","-1"]' "$ID" no i128_math --a -7 --b 2
expect "increment()" "1" "$ID" yes increment
expect_log "increment event" "counter"
expect "increment() again" "2" "$ID" yes increment
expect_error "get_temp(missing)" "Error(Contract, #1)" "$ID" no get_temp --key t
invoke "$ID" yes put_temp --key t --val 99 >/dev/null && ok "put_temp" || bad "put_temp: $(tail -3 "$OUT/last.log")"
expect "get_temp(t)" "99" "$ID" no get_temp --key t
expect_match "max_ttl()" "^[1-9][0-9]*$" "$ID" no max_ttl
expect "checked(50)" "100" "$ID" no checked --x 50
expect_error "checked(101)" "Error(Contract, #2)" "$ID" no checked --x 101
expect "address_string(me)" "\"$ME\"" "$ID" no address_string --a "$ME"
expect "address_from_string(me)" "\"$ME\"" "$ID" no address_from_string --s "$ME"
expect "muxed(me)" "[\"$ME\",null]" "$ID" no muxed --m "$ME"
build test_empty
HASH="$("$STELLAR" contract upload --wasm "$OUT/test_empty.wasm" --source e2e --network "$NETWORK" 2>/dev/null)"
SALT="$(printf '%064x' "$(date +%s)")"
expect_match "deploy(test_empty)" '^"C[A-Z0-9]{55}"$' "$ID" yes deploy --wasm_hash "$HASH" --salt "$SALT" --args '[]'
"$STELLAR" contract asset deploy --asset native --source e2e --network "$NETWORK" >/dev/null 2>&1 || true
XLM="$("$STELLAR" contract id asset --asset native --network "$NETWORK")"
expect_match "token_info(native, me)" '^\["native",7,"[0-9]+"\]$' "$ID" no token_info --token "$XLM" --id "$ME"
expect "pay(native, me -> contract, 5)" '"5"' "$ID" yes pay --token "$XLM" --from "$ME" --to "$ID" --amount 5

echo
echo "passed: $PASS failed: $FAIL"
[ "$FAIL" -eq 0 ]

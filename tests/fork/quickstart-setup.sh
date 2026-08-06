#!/usr/bin/env bash
#
# Submit transactions to a local stellar/quickstart network and write a fixture
# describing them, for the fork test in src/lib.rs (module `quickstart`) to fork
# from.
#
# Start quickstart with the ledger meta store and the history archive both
# running, then run this script:
#
#   docker run --rm -p 8000:8000 stellar/quickstart:testing --local --enable rpc,galexie
#   tests/fork/quickstart-setup.sh /tmp/quickstart-fixture.json
#   TEST_FORK_QUICKSTART_FIXTURE=/tmp/quickstart-fixture.json \
#     cargo test -p test_fork --lib -- --ignored --nocapture quickstart
#
# The transfers are of the native asset's Stellar Asset Contract, to an address
# that no other transaction touches, so the balance of that address changes
# only by the transfers submitted here and the expected balance at each point
# is known.

set -euo pipefail

FIXTURE_PATH="${1:?usage: quickstart-setup.sh <fixture-path>}"

HOST="${QUICKSTART_HOST:-http://localhost:8000}"
export STELLAR_RPC_URL="$HOST/rpc"
export STELLAR_NETWORK_PASSPHRASE="${QUICKSTART_NETWORK_PASSPHRASE:-Standalone Network ; February 2017}"

# Checkpoints are every 8 ledgers on quickstart's local network, matching
# Network::local()'s archive_checkpoint_ledger_count.
CHECKPOINT_FREQUENCY=8

# Amounts transferred, one transaction each. The balance the test expects
# before transfer N is the sum of the amounts before it.
AMOUNTS=(10 20 30)

rpc() {
  curl -sf "$STELLAR_RPC_URL" -H 'Content-Type: application/json' \
    -d "{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"$1\",\"params\":${2:-null}}"
}

# Create an account, retrying friendbot until it is up, because it takes a
# moment longer to come up than the rpc does.
account() {
  stellar keys generate "$1" --overwrite
  until curl -sf -o /dev/null "$HOST/friendbot?addr=$(stellar keys address "$1")"; do sleep 1; done
}

echo "Waiting for rpc to be healthy ..."
until [ "$(rpc getHealth | jq -r '.result.status // empty')" = "healthy" ]; do sleep 2; done

echo "Deploying the native asset contract ..."
account deployer
SAC=$(stellar contract id asset --asset native)
# Deploying fails if a previous run of this script against the same network
# already deployed it, which is only a problem if it is still not deployed.
stellar contract asset deploy --asset native --source-account deployer > /dev/null \
  || stellar contract info interface --id "$SAC" > /dev/null
echo "Native asset contract: $SAC"

# The address the transfers are sent to, and whose balance the test checks. The
# contract ID of the asset contract of a randomly named asset, which is never
# deployed and so is an address that no other transaction has touched: its
# balance starts at zero and only the transfers below change it.
TARGET=$(stellar contract id asset \
  --asset "T$(od -An -tx1 -N4 /dev/urandom | tr -d ' \n' | tr 'a-f' 'A-F'):$(stellar keys address deployer)")
echo "Target address: $TARGET"

transfers=()
for i in "${!AMOUNTS[@]}"; do
  amount="${AMOUNTS[$i]}"

  # A separate source account per transfer, so the transactions do not depend
  # on each other's sequence numbers and can land in the same ledger.
  account "sender$i"
  sender=$(stellar keys address "sender$i")

  signed=$(
    stellar contract invoke --quiet --build-only --source-account "sender$i" --id "$SAC" \
      -- transfer --from "$sender" --to "$TARGET" --amount "$amount" \
      | stellar tx simulate --quiet --source-account "sender$i" \
      | stellar tx sign --quiet --sign-with-key "sender$i"
  )
  hash=$(printf '%s' "$signed" | stellar tx hash)
  printf '%s' "$signed" | stellar tx send > /dev/null

  echo "Waiting for transaction $hash ..."
  result=$(
    until r=$(rpc getTransaction "{\"hash\":\"$hash\"}") \
      && [ "$(jq -r '.result.status' <<<"$r")" != "NOT_FOUND" ]; do sleep 1; done
    echo "$r"
  )
  status=$(jq -r '.result.status' <<<"$result")
  [ "$status" = "SUCCESS" ] || { echo "transaction $hash failed: $status"; exit 1; }

  ledger=$(jq -r '.result.ledger' <<<"$result")
  echo "Transferred $amount in ledger $ledger, transaction $hash"
  transfers+=("{\"ledger\":$ledger,\"tx\":\"$hash\",\"amount\":$amount}")
done

# A ledger far enough past the last transfer that the search for the balance
# entry falls out of the ledger meta store, back past a checkpoint, and into
# the history archive.
last_ledger=$(jq -r '.ledger' <<<"${transfers[-1]}")
archive_ledger=$(( (last_ledger / CHECKPOINT_FREQUENCY + 2) * CHECKPOINT_FREQUENCY ))
archive_checkpoint=$(( archive_ledger - 1 ))

echo "Waiting for ledger $archive_ledger to close ..."
until [ "$(rpc getLatestLedger | jq -r '.result.sequence')" -ge "$archive_ledger" ]; do sleep 1; done

echo "Waiting for ledger $archive_ledger in the ledger meta store ..."
# Galexie writes one ledger per file, 64000 files per partition, so on a fresh
# local network every ledger is in the first partition.
batch=$(printf '%08X--%d.xdr.zst' "$((0xFFFFFFFF - archive_ledger))" "$archive_ledger")
until curl -sf -o /dev/null "$HOST/ledger-meta/FFFFFFFF--0-63999/$batch"; do sleep 1; done

echo "Waiting for checkpoint $archive_checkpoint in the history archive ..."
until [ "$(curl -sf "$HOST/archive/.well-known/stellar-history.json" | jq -r '.currentLedger')" \
  -ge "$archive_checkpoint" ]; do sleep 1; done

jq -n \
  --arg sac "$SAC" \
  --arg target "$TARGET" \
  --argjson transfers "$(printf '%s\n' "${transfers[@]}" | jq -s .)" \
  --argjson archive_ledger "$archive_ledger" \
  '{sac: $sac, target: $target, transfers: $transfers, archive_ledger: $archive_ledger}' \
  > "$FIXTURE_PATH"

echo "Wrote $FIXTURE_PATH:"
cat "$FIXTURE_PATH"

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

# Ledgers to wait between transfers. Spreads the transfers over more than 64
# ledgers and several checkpoints, so that the transfers are not all reachable
# from the same place: the most recent is still in the ledger meta store, while
# the earlier ones have to be read out of the history archive, and out of
# different checkpoints of it rather than only the most recent one.
LEDGERS_BETWEEN_TRANSFERS=40

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

  if [ "$i" -gt 0 ]; then
    next=$(( $(jq -r '.ledger' <<<"${transfers[-1]}") + LEDGERS_BETWEEN_TRANSFERS ))
    echo "Waiting for ledger $next to close before the next transfer ..."
    until [ "$(rpc getLatestLedger | jq -r '.result.sequence')" -ge "$next" ]; do sleep 1; done
  fi

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

  # A ledger far enough past this transfer that a search for the balance entry
  # falls out of the ledger meta store, back past a checkpoint, and into the
  # history archive. One per transfer, so that each lands in a different
  # checkpoint: the test reads the earlier ones out of checkpoints that are no
  # longer the archive's most recent.
  archive_ledger=$(( (ledger / CHECKPOINT_FREQUENCY + 2) * CHECKPOINT_FREQUENCY ))

  echo "Transferred $amount in ledger $ledger, transaction $hash"
  transfers+=(
    "{\"ledger\":$ledger,\"tx\":\"$hash\",\"amount\":$amount,\"archive_ledger\":$archive_ledger}"
  )
done

# The last transfer's archive ledger is the furthest ahead, so waiting for it
# waits for everything the test reads.
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
  '{sac: $sac, target: $target, transfers: $transfers}' \
  > "$FIXTURE_PATH"

echo "Wrote $FIXTURE_PATH:"
cat "$FIXTURE_PATH"

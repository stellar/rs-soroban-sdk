/*
Verify the native XLM balance of GBLKX4UPDM7CC4UUG2FXBLOCXOTQ6ARHOQYVL4RD6A4AQVB6TPTLIUYN
at each (ledger, tx) point tested in src/lib.rs mod mainnet.

Reconstructs running balance from SAC contract events (transfers + fees)
using the known starting balance at ledger 61292151.

Source: BigQuery Hubble dataset crypto-stellar.crypto_stellar

Usage:
  bq query --use_legacy_sql=false --format=csv --max_rows=200 \
    "$(cat verify_balances.sql)" > verify_balances.csv
*/

WITH
balance_changes AS (
  SELECT e.ledger_sequence, e.transaction_hash,
    JSON_VALUE(e.topics_decoded, "$[0].symbol") as event_type,
    CASE
      WHEN JSON_VALUE(e.topics_decoded, "$[0].symbol") = "fee"
        THEN -CAST(JSON_VALUE(e.data_decoded, "$.i128") AS INT64)
      WHEN JSON_VALUE(e.topics_decoded, "$[1].address") = "GBLKX4UPDM7CC4UUG2FXBLOCXOTQ6ARHOQYVL4RD6A4AQVB6TPTLIUYN"
        THEN -CAST(JSON_VALUE(e.data_decoded, "$.i128") AS INT64)
      WHEN JSON_VALUE(e.topics_decoded, "$[2].address") = "GBLKX4UPDM7CC4UUG2FXBLOCXOTQ6ARHOQYVL4RD6A4AQVB6TPTLIUYN"
        THEN CAST(JSON_VALUE(e.data_decoded, "$.i128") AS INT64)
    END as amount_change
  FROM `crypto-stellar.crypto_stellar.history_contract_events` e
  WHERE e.ledger_sequence BETWEEN 61292152 AND 61292232
    AND e.contract_id = "CAS3J7GYLGXMF6TDJBBYYSE3HQ6BBSMLNUQ34T6TZMYMW2EVH34XOWMA"
    AND e.type_string = "ContractEventTypeContract"
    AND JSON_VALUE(e.topics_decoded, "$[0].symbol") IN ("transfer", "fee")
    AND (
      JSON_VALUE(e.topics_decoded, "$[1].address") = "GBLKX4UPDM7CC4UUG2FXBLOCXOTQ6ARHOQYVL4RD6A4AQVB6TPTLIUYN"
      OR JSON_VALUE(e.topics_decoded, "$[2].address") = "GBLKX4UPDM7CC4UUG2FXBLOCXOTQ6ARHOQYVL4RD6A4AQVB6TPTLIUYN"
    )
),
tx_summary AS (
  SELECT
    ledger_sequence,
    transaction_hash,
    SUM(amount_change) as net_change,
    STRING_AGG(DISTINCT event_type ORDER BY event_type) as event_types
  FROM balance_changes
  GROUP BY ledger_sequence, transaction_hash
),
ordered AS (
  SELECT ts.ledger_sequence, ts.transaction_hash, ts.net_change, ts.event_types, t.id as tx_id
  FROM tx_summary ts
  JOIN `crypto-stellar.crypto_stellar.history_transactions` t
    ON ts.transaction_hash = t.transaction_hash AND ts.ledger_sequence = t.ledger_sequence
),
running AS (
  SELECT
    ledger_sequence,
    transaction_hash,
    event_types,
    tx_id,
    28640019212 + COALESCE(SUM(net_change) OVER (
      ORDER BY tx_id ROWS BETWEEN UNBOUNDED PRECEDING AND 1 PRECEDING
    ), 0) as balance_before_tx
  FROM ordered
)
SELECT ledger_sequence, transaction_hash, event_types, balance_before_tx
FROM running
ORDER BY tx_id

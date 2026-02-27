/*
Verify the native XLM balance of GBLKX4UPDM7CC4UUG2FXBLOCXOTQ6ARHOQYVL4RD6A4AQVB6TPTLIUYN
using the accounts table which appends a row each time the account is modified.

Each row represents the end-of-ledger balance after all transactions in that
ledger have been applied. This is compared against the test's balance_before_tx
for the first transaction in the next active ledger.

Source: BigQuery Hubble dataset crypto-stellar.crypto_stellar.accounts

Usage:
  bq query --use_legacy_sql=false --format=csv --max_rows=200 \
    "$(cat verify_balances_accounts.sql)" > verify_balances_accounts.csv
*/

WITH
account_balances AS (
  SELECT
    ledger_sequence,
    CAST(ROUND(balance * 10000000) AS INT64) as balance_stroops
  FROM `crypto-stellar.crypto_stellar.accounts`
  WHERE account_id = "GBLKX4UPDM7CC4UUG2FXBLOCXOTQ6ARHOQYVL4RD6A4AQVB6TPTLIUYN"
    AND ledger_sequence BETWEEN 61292151 AND 61292232
  ORDER BY ledger_sequence
),
with_next AS (
  SELECT
    ledger_sequence,
    balance_stroops,
    LEAD(ledger_sequence) OVER (ORDER BY ledger_sequence) as next_active_ledger,
    LEAD(balance_stroops) OVER (ORDER BY ledger_sequence) as next_balance_stroops
  FROM account_balances
)
SELECT
  ledger_sequence,
  balance_stroops as end_of_ledger_balance
FROM with_next
ORDER BY ledger_sequence

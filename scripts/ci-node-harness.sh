#!/usr/bin/env bash
set -euo pipefail

node --check tests/harness/pi-cold-routing/bounds.mjs
node --check tests/harness/pi-cold-routing/runner.mjs
node --check tests/harness/codex-cold-routing/driver.mjs
node --check tests/harness/byte-ledger/ledger.mjs
node --check scripts/byte-ledger.mjs
node --check scripts/recommendation-pilot.mjs
node --check tests/harness/roster-recommendation-pilot/pilot.mjs
# gate.test.mjs imports gate.ts below, checking and exercising its TypeScript.
# Newer Node releases parse --check inputs as JavaScript even with strip-types.
node --test \
  tests/harness/pi-cold-routing/*.test.mjs \
  tests/harness/codex-cold-routing/*.test.mjs \
  tests/harness/byte-ledger/*.test.mjs \
  tests/harness/roster-recommendation-pilot/*.test.mjs

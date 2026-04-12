# ADR-002: Security Audit and Hardening

**Status:** Accepted
**Date:** 2025-07-15

## Context

A comprehensive security audit of convergio-billing identified several issues:

1. **Bug: Wrong column/table references** — `handle_cost_summary` referenced `recorded_at` (non-existent column, should be `created_at`) and `plans` table (not part of billing schema). Both queries silently failed, always returning 0.
2. **Bug: Timestamps not parsed from DB** — `usage_for_period`, `list_invoices`, and `list_settlements` used `chrono::Utc::now()` instead of parsing the actual `created_at` value from the database.
3. **Secret exposure** — Pool and DB error `.to_string()` leaked internal details to API responses.
4. **No input validation** — Monetary fields (`cost_usd`, `quantity`, `price_per_unit`, `daily_limit_usd`, `monthly_limit_usd`, `amount_usd`) accepted negative, NaN, and Infinity values. `entity_id` and `org_id` had no length constraints.

## Decision

### Fixes Applied

- **cost_summary endpoint**: Fixed to use `created_at` column and query `billing_budgets` instead of non-existent `plans` table.
- **Timestamp parsing**: All DB reads now parse RFC 3339 timestamps with fallback to `Utc::now()`.
- **Error sanitization**: All error responses now return opaque messages (`"service unavailable"`, `"failed to set budget"`) instead of raw error strings.
- **Input validation on all monetary functions**:
  - `record_usage()`: rejects negative/NaN/Infinity `quantity` and `cost_usd`
  - `set_rate()`: rejects negative/NaN/Infinity `price_per_unit`
  - `set_budget()`: rejects negative/NaN/Infinity `daily_limit_usd` and `monthly_limit_usd`
  - `record_settlement()`: rejects negative/NaN/Infinity `amount_usd`
  - `handle_alerts` route: validates limits before DB write, rejects empty/overlength `entity_id`
  - `handle_usage` route: rejects empty/overlength `org_id`

### Security Checklist Result

| Check | Result |
|-------|--------|
| SQL injection | PASS — all queries use parameterized `?N` placeholders |
| Path traversal | N/A — no file operations |
| Command injection | N/A — no shell commands |
| SSRF | N/A — no outbound HTTP |
| Secret exposure | FIXED — error messages sanitized |
| Race conditions | ACCEPTABLE — SQLite serializes writes |
| Unsafe blocks | PASS — none present |
| Input validation | FIXED — all monetary inputs validated |
| Auth/AuthZ | N/A — handled at gateway layer |
| Integer overflow | FIXED — NaN/Infinity rejected at input |

## Consequences

- **Positive**: All billing calculations now reject invalid inputs at the boundary, preventing NaN/Infinity propagation through the system.
- **Positive**: `cost_summary` endpoint now returns correct data instead of always-zero values.
- **Positive**: Internal error details no longer leak to API consumers.
- **Negative**: Callers passing NaN/negative values will now get errors instead of silent corruption — this is the correct behavior.
- **Tests**: 6 new validation tests added (27 total, up from 21).

# Fork Account Pool Contract

This fork keeps account-pool ownership outside Codex. Codex consumes the selected account through the app-server auth surface and does not store, rank, rotate, lease, or inspect a pool of accounts.

## Design

The account pool owns account inventory, token storage, rotation policy, cooldowns, quota decisions, and secret handling. When the pool selects an account for Codex, it calls `account/login/start` with `type: "chatgptAuthTokens"` and supplies the active `accessToken`, `chatgptAccountId`, and optional `chatgptPlanType`.

When Codex receives `401 Unauthorized` while using externally managed ChatGPT auth, it sends `account/chatgptAuthTokens/refresh` to the client. The request includes `previousAccountId` when the current auth snapshot has one. The pool should use that value as a continuity hint. Returning a different account is allowed only when the pool intentionally wants to switch accounts; Codex rejects the refresh itself when a forced workspace is configured and the returned account does not match it.

`account/read` with `reloadAuth: true` is only a fallback for integrations that update persistent auth storage outside the app-server process. It should not be used as the primary pool-switching mechanism, and the TUI should not set it by default because status reads must not switch accounts unexpectedly.

## Invariants

- Keep pool state out of `codex-core`.
- Prefer `chatgptAuthTokens` over writing `auth.json` for normal account switches.
- Use `reloadAuth` only after an external process intentionally changed persistent auth storage.
- Treat `previousAccountId` on refresh as the account continuity hint.
- Never log raw access tokens or pool secrets.
- Do not silently switch to an unrelated account; any account change during refresh must be a deliberate pool decision.

## When to Add More API

Add private `accountPool/*` methods only if Codex itself must reserve, release, mark, or select accounts. Until then, an external pool plus the existing auth-token bridge is the smallest boundary that keeps upstream merges manageable.

## Fork Maintenance Flow

Use this flow when updating the fork from `openai/codex`:

1. Keep `upstream` fetch-only. Do not enable a push URL for `upstream`; all fork work goes to `origin`.
2. Create or refresh a backup branch/tag before pulling upstream changes into the fork.
3. Bring upstream changes into a short-lived `codex/*` sync branch, resolve conflicts there, and keep pool-specific changes limited to the auth bridge and fork docs.
4. Preserve the external pool boundary: the pool selects accounts and calls `account/login/start` with `chatgptAuthTokens`; Codex should not grow pool selection, lease, cooldown, or quota logic unless this doc is deliberately superseded.
5. Run the focused local checks for auth-boundary changes:
   - `just write-app-server-schema` when app-server protocol shapes change.
   - `just fmt` from `codex-rs` after Rust changes.
   - `cargo test -p codex-app-server-protocol`.
   - `cargo test -p codex-app-server suite::v2::account`.
6. Push only to `origin`, let the fork `Fork checks` workflow pass, then fast-forward `origin/main`.
7. After `origin/main` points at the sync commit and `Fork checks` is green, delete the short-lived sync branch. Keep backup branches/tags.

GitHub branch protection for this fork should require only the `Fork checks` status check. The upstream-heavy workflows are guarded so they remain available for `openai/codex` but skip in `uxio8/codex`.

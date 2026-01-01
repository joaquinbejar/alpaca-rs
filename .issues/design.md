# Alpaca-RS Issue Implementation Agent

You are an autonomous development agent responsible for implementing issues in the alpaca-rs Rust library. Your goal is to systematically work through the issues defined in `.issues/` following the roadmap priorities.

## Repository Context

- **Project**: alpaca-rs - Rust client library for Alpaca Markets API
- **Language**: Rust
- **API Reference**: Available in `doc/api/docs.alpaca.markets/docs/`
- **Issue Files**: Located in `.issues/` directory with format `XXX-issue-name.md` (local specifications)
- **GitHub Issues**: Created from issue files, referenced by GitHub number (e.g., `#12`)
- **Roadmap**: Defined in `.issues/000-roadmap.md`

### Workspace Crates

The repository is a Cargo workspace with the following crates:

| Crate | Description | README |
|-------|-------------|--------|
| `alpaca-base` | Core types, models, and shared utilities | `alpaca-base/README.md` |
| `alpaca-http` | HTTP client for REST API endpoints | `alpaca-http/README.md` |
| `alpaca-websocket` | WebSocket streaming client | `alpaca-websocket/README.md` |

**Each crate MUST have its own `README.md`** that is kept up-to-date with every change to that crate.

## Initial Setup (Run Once)

Before implementing any issues, create all GitHub issues and labels from the `.issues/` directory.

### Step 0.1: Create Labels

```bash
# Priority labels
gh label create "priority:high" --color "d73a4a" --description "High priority - core functionality"
gh label create "priority:medium" --color "fbca04" --description "Medium priority - important features"
gh label create "priority:low" --color "0e8a16" --description "Low priority - specialized features"

# Phase labels
gh label create "phase:1" --color "1d76db" --description "Phase 1: Core Trading"
gh label create "phase:2" --color "1d76db" --description "Phase 2: Options & Enhanced Market Data"
gh label create "phase:3" --color "1d76db" --description "Phase 3: Broker API"
gh label create "phase:4" --color "1d76db" --description "Phase 4: Crypto & Portfolio"
gh label create "phase:5" --color "1d76db" --description "Phase 5: Additional Features"
gh label create "phase:6" --color "1d76db" --description "Phase 6: Specialized Features"

# Type labels
gh label create "type:trading" --color "5319e7" --description "Trading API functionality"
gh label create "type:market-data" --color "5319e7" --description "Market data functionality"
gh label create "type:broker" --color "5319e7" --description "Broker API functionality"
gh label create "type:websocket" --color "5319e7" --description "WebSocket streaming"
gh label create "type:crypto" --color "5319e7" --description "Cryptocurrency functionality"
gh label create "type:infrastructure" --color "5319e7" --description "Testing, errors, rate limiting"
gh label create "type:documentation" --color "5319e7" --description "Documentation improvements"
gh label create "type:auth" --color "5319e7" --description "Authentication & authorization"
```

### Step 0.2: Create GitHub Issues

For each issue file in `.issues/` (excluding `000-roadmap.md`), create a GitHub issue:

```bash
# Example for issue 001
gh issue create \
  --title "Options Trading API" \
  --body "$(cat .issues/001-options-trading-api.md)" \
  --label "priority:high,phase:2,type:trading"
```

**Issue to Label Mapping** (based on roadmap):

| Issue | Title | Labels |
|-------|-------|--------|
| 001 | Options Trading API | `priority:high`, `phase:2`, `type:trading` |
| 002 | Broker API - Accounts | `priority:high`, `phase:3`, `type:broker` |
| 003 | Broker API - Funding | `priority:high`, `phase:3`, `type:broker` |
| 004 | Enhanced Crypto Trading | `priority:high`, `phase:4`, `type:crypto` |
| 005 | Enhanced Stock Market Data | `priority:high`, `phase:2`, `type:market-data` |
| 006 | News API Enhancement | `priority:medium`, `phase:5`, `type:market-data` |
| 007 | Enhanced WebSocket Streaming | `priority:high`, `phase:2`, `type:websocket` |
| 008 | OAuth Support | `priority:medium`, `phase:5`, `type:auth` |
| 009 | Broker API Events (SSE) | `priority:medium`, `phase:3`, `type:broker` |
| 010 | Enhanced Account Activities | `priority:medium`, `phase:5`, `type:trading` |
| 011 | Portfolio Management | `priority:medium`, `phase:4`, `type:trading` |
| 012 | Advanced Order Types | `priority:high`, `phase:1`, `type:trading` |
| 013 | Enhanced Assets API | `priority:medium`, `phase:5`, `type:market-data` |
| 014 | Calendar & Clock Enhancement | `priority:low`, `phase:6`, `type:market-data` |
| 015 | Comprehensive Error Handling | `priority:high`, `phase:1`, `type:infrastructure` |
| 016 | Rate Limiting | `priority:medium`, `phase:4`, `type:infrastructure` |
| 017 | Testing Infrastructure | `priority:high`, `phase:1`, `type:infrastructure` |
| 018 | Comprehensive Documentation | `priority:high`, `phase:1`, `type:documentation` |
| 019 | FIX Protocol Support | `priority:low`, `phase:6`, `type:trading` |
| 020 | Statements & Confirmations | `priority:low`, `phase:6`, `type:broker` |
| 021 | Margin & Short Selling | `priority:medium`, `phase:5`, `type:trading` |
| 022 | Fractional Trading | `priority:medium`, `phase:5`, `type:trading` |
| 023 | Paper Trading Support | `priority:medium`, `phase:5`, `type:trading` |
| 024 | Local Currency Trading | `priority:low`, `phase:6`, `type:trading` |
| 025 | IRA Account Support | `priority:low`, `phase:6`, `type:broker` |

**Script to create all issues:**

```bash
#!/bin/bash
# create_github_issues.sh

# Array of issues with their labels
declare -A ISSUES=(
  ["001-options-trading-api.md"]="Options Trading API|priority:high,phase:2,type:trading"
  ["002-broker-api-accounts.md"]="Broker API - Accounts|priority:high,phase:3,type:broker"
  ["003-broker-api-funding.md"]="Broker API - Funding|priority:high,phase:3,type:broker"
  ["004-crypto-trading-enhanced.md"]="Enhanced Crypto Trading|priority:high,phase:4,type:crypto"
  ["005-market-data-stocks-enhanced.md"]="Enhanced Stock Market Data|priority:high,phase:2,type:market-data"
  ["006-news-api.md"]="News API Enhancement|priority:medium,phase:5,type:market-data"
  ["007-websocket-streams-enhanced.md"]="Enhanced WebSocket Streaming|priority:high,phase:2,type:websocket"
  ["008-oauth-support.md"]="OAuth Support|priority:medium,phase:5,type:auth"
  ["009-broker-api-events-sse.md"]="Broker API Events (SSE)|priority:medium,phase:3,type:broker"
  ["010-account-activities-enhanced.md"]="Enhanced Account Activities|priority:medium,phase:5,type:trading"
  ["011-portfolio-management.md"]="Portfolio Management|priority:medium,phase:4,type:trading"
  ["012-orders-advanced.md"]="Advanced Order Types|priority:high,phase:1,type:trading"
  ["013-assets-enhanced.md"]="Enhanced Assets API|priority:medium,phase:5,type:market-data"
  ["014-calendar-clock.md"]="Calendar & Clock Enhancement|priority:low,phase:6,type:market-data"
  ["015-error-handling.md"]="Comprehensive Error Handling|priority:high,phase:1,type:infrastructure"
  ["016-rate-limiting.md"]="Rate Limiting|priority:medium,phase:4,type:infrastructure"
  ["017-testing-infrastructure.md"]="Testing Infrastructure|priority:high,phase:1,type:infrastructure"
  ["018-documentation.md"]="Comprehensive Documentation|priority:high,phase:1,type:documentation"
  ["019-fix-protocol.md"]="FIX Protocol Support|priority:low,phase:6,type:trading"
  ["020-statements-confirms.md"]="Statements & Confirmations|priority:low,phase:6,type:broker"
  ["021-margin-short-selling.md"]="Margin & Short Selling|priority:medium,phase:5,type:trading"
  ["022-fractional-trading.md"]="Fractional Trading|priority:medium,phase:5,type:trading"
  ["023-paper-trading.md"]="Paper Trading Support|priority:medium,phase:5,type:trading"
  ["024-local-currency-trading.md"]="Local Currency Trading|priority:low,phase:6,type:trading"
  ["025-ira-accounts.md"]="IRA Account Support|priority:low,phase:6,type:broker"
)

for file in "${!ISSUES[@]}"; do
  IFS='|' read -r title labels <<< "${ISSUES[$file]}"
  if [[ -f ".issues/$file" ]]; then
    echo "Creating issue: $title"
    gh issue create \
      --title "$title" \
      --body "$(cat .issues/$file)" \
      --label "$labels"
    sleep 1  # Avoid rate limiting
  fi
done
```

### Step 0.3: Verify Issues Created

```bash
# List all created issues
gh issue list --limit 30

# Verify labels exist
gh label list
```

**⚠️ IMPORTANT**: Run the initial setup only once. After issues are created in GitHub, the implementation workflow references them by their GitHub issue number (e.g., `#12` for Advanced Order Types).

---

**⚠️ CRITICAL: This is a strictly sequential workflow. Complete each issue fully (including merge and release) before starting the next one. Never have multiple open PRs.**

> [!IMPORTANT]
> **Every single change, no matter how small, MUST be performed in a dedicated feature branch.**
> Never commit directly to the `main` branch.

```
┌─────────────────────────────────────────────────────────────────┐
│  1. SELECT ISSUE (by phase priority from roadmap)               │
│                           ↓                                     │
│  2. ANALYZE (read issue, identify affected crates)              │
│                           ↓                                     │
│  3. IMPLEMENT (code changes + version bump + README update)     │
│                           ↓                                     │
│  4. VALIDATE (make lint-fix pre-push)                           │
│                           ↓                                     │
│  5. CREATE PR (gh pr create)                                    │
│                           ↓                                     │
│  6. MERGE (gh pr merge) ← Must complete before next issue       │
│                           ↓                                     │
│  7. RELEASE (gh release create per crate)                       │
│                           ↓                                     │
│  8. CLEANUP (delete issue file, commit)                         │
└─────────────────────────────────────────────────────────────────┘
```

## Detailed Steps

### Step 1: Select Next Issue

**Before starting, verify no open PRs exist:**
```bash
gh pr list --state open
```
If any PRs are open, complete them first (merge + release + cleanup).

**Find the next issue to implement:**
```bash
# List open issues by phase priority
gh issue list --label "phase:1" --state open  # Start with Phase 1
gh issue list --label "phase:2" --state open  # Then Phase 2, etc.
```

Follow the priority order defined in `.issues/000-roadmap.md`:
- Phase 1 (Core Trading) → Phase 2 (Options) → Phase 3 (Broker) → etc.
- Within each phase, implement in the order listed

**⚠️ IMPORTANT**: Note the GitHub issue number (e.g., `#12`) from `gh issue list`. This number is used for:
- Branch naming: `issue-12-description`
- PR closing: `Closes #12`

```bash
# Read issue details
gh issue view <number>
# Or read the local file for full specification:
cat .issues/XXX-issue-name.md
```

### Step 2: Analyze the Issue

1. Read the issue file completely:
   ```bash
   cat .issues/XXX-issue-name.md
   ```

2. Identify:
    - Affected crate(s) in the workspace
    - Required API endpoints (reference `doc/api/`)
    - Dependencies on other issues
    - Breaking vs non-breaking changes

3. If the issue depends on unimplemented issues, skip to the next one.

### Step 3: Implement Changes

#### 3.1 Create Feature Branch

```bash
git checkout main
git pull origin main
git checkout -b issue-<github-number>-<short-description>
```

Branch naming: `issue-<github-number>-<short-kebab-description>`

Examples:
- `issue-12-advanced-order-types`
- `issue-15-error-handling`
- `issue-1-options-trading`

#### 3.2 Implement the Code

- Follow existing code patterns in the repository
- Use the API documentation in `doc/api/` as reference

**Testing Requirements**:

1. **Unit Tests (MANDATORY)**:
    - Create unit tests for all new functionality
    - Place tests in the same file using `#[cfg(test)]` module or in `tests/` directory
    - Test edge cases, error conditions, and happy paths
    - Use mocking when external services are involved

   ```rust
   #[cfg(test)]
   mod tests {
       use super::*;
       
       #[test]
       fn test_new_feature_happy_path() {
           // Test implementation
       }
       
       #[test]
       fn test_new_feature_error_handling() {
           // Test error cases
       }
   }
   ```

2. **Integration Tests (when necessary)**:
    - Create integration tests when the feature interacts with external APIs
    - Place in `tests/` directory at crate root
    - Use `#[ignore]` attribute for tests requiring real API credentials
    - Document required environment variables

   ```rust
   // tests/integration_test.rs
   #[test]
   #[ignore] // Run with: cargo test -- --ignored
   fn test_real_api_integration() {
       // Requires ALPACA_API_KEY and ALPACA_API_SECRET in .env
   }
   ```

**When to create integration tests**:
- New API endpoint implementations
- WebSocket streaming functionality
- Authentication flows
- Any feature that depends on external service behavior

#### 3.3 Documentation Requirements

**All code must be fully documented. Never use `#![allow(missing_docs)]`.**

1. **Module-level documentation**:
   ```rust
   //! This module provides advanced order types for the Alpaca Trading API.
   //!
   //! # Overview
   //!
   //! Supports bracket orders, OCO, and OTO order types.
   //!
   //! # Examples
   //!
   //! ```rust
   //! use alpaca_trading::orders::BracketOrder;
   //!
   //! let order = BracketOrder::new("AAPL", 100, 150.0, 145.0, 160.0);
   //! ```
   ```

2. **Struct/Enum documentation**:
   ```rust
   /// Represents a bracket order with entry, stop-loss, and take-profit levels.
   ///
   /// A bracket order automatically creates three linked orders:
   /// - Entry order (limit or market)
   /// - Stop-loss order (triggered if price drops)
   /// - Take-profit order (triggered if price rises)
   ///
   /// # Fields
   ///
   /// * `symbol` - The stock ticker symbol
   /// * `qty` - Number of shares to trade
   /// * `entry_price` - Limit price for entry (None for market order)
   /// * `stop_loss` - Stop-loss trigger price
   /// * `take_profit` - Take-profit limit price
   #[derive(Debug, Clone)]
   pub struct BracketOrder {
       /// The stock ticker symbol (e.g., "AAPL")
       pub symbol: String,
       /// Number of shares to trade
       pub qty: u32,
       // ... etc
   }
   ```

3. **Function/Method documentation**:
   ```rust
   /// Creates a new bracket order with the specified parameters.
   ///
   /// # Arguments
   ///
   /// * `symbol` - The stock ticker symbol
   /// * `qty` - Number of shares to trade
   /// * `stop_loss` - Stop-loss trigger price
   /// * `take_profit` - Take-profit limit price
   ///
   /// # Returns
   ///
   /// A new `BracketOrder` instance ready to be submitted.
   ///
   /// # Errors
   ///
   /// Returns `OrderError::InvalidPrice` if stop_loss >= take_profit.
   ///
   /// # Examples
   ///
   /// ```rust
   /// use alpaca_trading::orders::BracketOrder;
   ///
   /// let order = BracketOrder::new("AAPL", 100, 145.0, 160.0)?;
   /// assert_eq!(order.symbol, "AAPL");
   /// ```
   pub fn new(symbol: &str, qty: u32, stop_loss: f64, take_profit: f64) -> Result<Self, OrderError> {
       // ...
   }
   ```

4. **Inline comments** for complex logic:
   ```rust
   // Calculate the adjusted stop price based on the trailing percentage.
   // We use the highest price seen since order creation as the reference.
   let adjusted_stop = highest_price * (1.0 - trailing_percent / 100.0);
   ```

**Documentation checklist**:
- [ ] All public modules have `//!` module docs with overview and examples
- [ ] All public structs/enums have `///` docs explaining purpose and usage
- [ ] All public fields have `///` docs
- [ ] All public functions have `///` docs with Arguments, Returns, Errors, and Examples
- [ ] Complex algorithms have inline `//` comments explaining the logic
- [ ] No `#![allow(missing_docs)]` anywhere in the codebase

#### 3.3 Create Usage Examples

For each new feature, create example(s) in the `examples/` directory:

```bash
# Example file naming convention
examples/<crate>_<feature>_example.rs
# or for complex features
examples/<feature>/main.rs
```

**Example Requirements**:
- Demonstrate real-world usage of the new functionality
- Use **real data and services** when possible (Alpaca paper trading API)
- Include clear comments explaining each step
- Handle errors gracefully with informative messages

**Configuration via Environment Variables**:

Examples must load configuration from `.env` file:

```rust
use dotenvy::dotenv;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load .env file
    dotenv().ok();
    
    let api_key = env::var("ALPACA_API_KEY")?;
    let api_secret = env::var("ALPACA_API_SECRET")?;
    let base_url = env::var("ALPACA_BASE_URL")
        .unwrap_or_else(|_| "https://paper-api.alpaca.markets".to_string());
    
    // ... example code
    Ok(())
}
```

**Required `.env` variables** (document in example comments):
```env
ALPACA_API_KEY=your_api_key
ALPACA_API_SECRET=your_api_secret
ALPACA_BASE_URL=https://paper-api.alpaca.markets  # or live API
```

Ensure `.env` is in `.gitignore` and provide `.env.example` template.

#### 3.3 Update Crate Version

For each modified crate, update `Cargo.toml`:

```toml
[package]
version = "X.Y.Z"  # Increment appropriately
```

**Version Increment Rules (SemVer)**:
- **MAJOR** (X): Breaking API changes
- **MINOR** (Y): New features, backward compatible
- **PATCH** (Z): Bug fixes, internal improvements

#### 3.4 Update Crate README

**Every crate has its own README.md that MUST be updated when that crate changes:**

- `alpaca-base/README.md` - Core types, models, and shared utilities
- `alpaca-http/README.md` - HTTP client for REST API endpoints
- `alpaca-websocket/README.md` - WebSocket streaming client

**README Structure** (maintain this format for each crate):

```markdown
# <crate-name>

<Brief one-line description>

## Overview

<Detailed description of what this crate provides>

## Features

- Feature 1: <description>
- Feature 2: <description>

## Installation

Add to your `Cargo.toml`:

\`\`\`toml
[dependencies]
<crate-name> = "<current-version>"
\`\`\`

## Usage

\`\`\`rust
// Basic usage example
\`\`\`

## API Reference

### Main Types

- `TypeName` - <brief description>

### Main Functions

- `function_name()` - <brief description>

## Examples

See the `examples/` directory:
- `example_name.rs` - <description>

## Changelog

### v<version> (latest)
- <Change 1>
- <Change 2>

## License

<License info>
```

**What to update in README when modifying a crate**:
- [ ] Version number in Installation section
- [ ] New features in Features section
- [ ] New types/functions in API Reference section
- [ ] New examples in Examples section
- [ ] Changelog with summary of changes
- [ ] Usage examples if API changed

### Step 4: Validate Changes

**MANDATORY**: Run validation before any PR:

```bash
make lint-fix pre-push
```

This must complete successfully. If it fails:
1. Fix all reported issues
2. Re-run until all checks pass
3. Never proceed with failing checks

### Step 5: Create Pull Request

```bash
# Stage and commit changes
git add .
git commit -m "<type>(<scope>): <description>

<body>

Closes #<github-issue-number>"

# Push and create PR
git push -u origin HEAD
gh pr create \
  --title "<type>(<scope>): <description>" \
  --body "## Summary
<Brief description of changes>

## Changes
- <Change 1>
- <Change 2>

## Testing
- Unit tests: <describe unit tests added>
- Integration tests: <describe integration tests or N/A>
- Manual testing: <how feature was manually verified>

## Checklist
- [x] Version bumped in affected crate(s)
- [x] README updated for affected crate(s) (alpaca-base/http/websocket)
- [x] All public items documented (no \`#![allow(missing_docs)]\`)
- [x] Unit tests added
- [x] Integration tests added (if applicable)
- [x] Examples created in \`examples/\`
- [x] \`make lint-fix pre-push\` passes

Closes #<github-issue-number>" \
  --base main
```

**⚠️ IMPORTANT**: Use the GitHub issue number (from `gh issue list`), not the file number. The `Closes #XX` keyword in the PR body will automatically close the issue when the PR is merged.

#### Commit Message Format (Conventional Commits)

```
<type>(<scope>): <description>

[optional body]

[optional footer]
```

**Types**:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation only
- `refactor`: Code refactoring
- `test`: Adding tests
- `chore`: Maintenance tasks

**Scope**: The crate name or module affected

### Step 6: Merge Pull Request

After PR is created and CI passes:

```bash
gh pr merge --squash --delete-branch
```

Then update local main:

```bash
git checkout main
git pull origin main
```

### Step 7: Create Release

For each crate that was versioned, create a separate release:

**Available crates:**
- `alpaca-base`
- `alpaca-http`
- `alpaca-websocket`

```bash
gh release create <crate-name>-v<version> \
  --title "<crate-name> v<version>" \
  --notes "## What's New

<Summary of changes from the issue>

## Changes
- <Change 1>
- <Change 2>

See [Issue XXX](.issues/XXX-issue-name.md) for details."
```

**Release Tag Format**: `<crate-name>-v<major>.<minor>.<patch>`

Examples:
- `alpaca-base-v0.2.0`
- `alpaca-http-v0.3.0`
- `alpaca-websocket-v0.1.5`

### Step 8: Cleanup

The GitHub issue is automatically closed when the PR is merged (via `Closes #XX` keyword).

Remove the local issue file from `.issues/`:

```bash
git checkout main
rm .issues/XXX-issue-name.md
git add .issues/XXX-issue-name.md
git commit -m "chore: remove resolved issue file XXX"
git push origin main
```

## Rules and Constraints

### MUST DO
- ✅ Complete ONE issue fully before starting the next
- ✅ Always run `make lint-fix pre-push` before creating PR
- ✅ Always bump version of modified crates
- ✅ Always update README.md of modified crates (`alpaca-base`, `alpaca-http`, `alpaca-websocket`)
- ✅ Always create unit tests for new functionality
- ✅ Create integration tests when interacting with external APIs
- ✅ Document all public items (modules, structs, enums, functions, fields)
- ✅ Always create examples in `examples/` for new features
- ✅ Always use `.env` for configuration in examples (never hardcode credentials)
- ✅ Always use `gh` CLI for GitHub operations
- ✅ Always delete issue file after successful merge and release
- ✅ Follow existing code style and patterns

### MUST NOT
- ❌ Never skip the validation step
- ❌ Never skip writing unit tests for new functionality
- ❌ Never create PR with failing checks
- ❌ Never merge without CI passing
- ❌ Never implement issues out of dependency order
- ❌ Never modify multiple unrelated issues in one PR
- ❌ Never hardcode API keys or secrets (always use `.env`)
- ❌ Never create a new PR before the previous one is merged
- ❌ Never use `#![allow(missing_docs)]` — all public items must be documented

### ERROR HANDLING

If `make lint-fix pre-push` fails:
1. Read the error output carefully
2. Fix the issues in the code
3. Re-run until successful
4. Only then proceed to PR creation

If PR creation fails:
1. Check `gh auth status`
2. Ensure you have push permissions
3. Verify branch is pushed

If merge fails:
1. Check for merge conflicts
2. Resolve conflicts locally
3. Push updates
4. Retry merge

## Example Session

```bash
# 0. Verify no open PRs (MANDATORY before starting)
gh pr list --state open
# Must return empty. If not, complete pending PR first.

# 1. Find the GitHub issue number for the next issue to implement
gh issue list --label "phase:1" --state open
# Example output: #12  Advanced Order Types  priority:high,phase:1,type:trading

# 2. Read the issue details
gh issue view 12
# Or read the local file for full details:
cat .issues/012-orders-advanced.md

# 3. Create branch (use GitHub issue number)
git checkout -b issue-12-advanced-order-types

# 4. Implement changes...
# (code implementation with full documentation)
# - Document all modules with //! docs
# - Document all public structs/enums with /// docs
# - Document all public functions with /// docs including Examples
# - Add inline comments for complex logic
# - NEVER use #![allow(missing_docs)]

# 5. Create unit tests
# Add tests in src/orders.rs or tests/orders_test.rs
# Test all new order types, edge cases, and error handling

# 6. Create integration tests (if needed)
# Add tests/integration/orders_advanced.rs
# Mark with #[ignore] if requires real API

# 7. Create examples
# Create examples/trading_bracket_order.rs
# Create examples/trading_oco_order.rs
# Ensure they use .env for ALPACA_API_KEY, ALPACA_API_SECRET, etc.

# 8. Update version in affected crate(s)
# Edit alpaca-http/Cargo.toml: version = "0.3.0"
# (Also alpaca-base/Cargo.toml if types were added there)

# 9. Update README(s) for modified crate(s)
# Edit alpaca-http/README.md:
#   - Update version in Installation section
#   - Add new order types to Features section
#   - Add BracketOrder, OcoOrder to API Reference
#   - Add new examples to Examples section
#   - Add changelog entry for v0.3.0

# 10. Validate
make lint-fix pre-push

# 11. Commit and PR
git add .
git commit -m "feat(trading): implement advanced order types

Add support for bracket, OCO, and OTO orders.
Includes trailing stop and stop limit orders.
All public items fully documented.
Added unit tests and integration tests.
Added examples for bracket and OCO orders.

Closes #12"

git push -u origin HEAD
gh pr create --title "feat(trading): implement advanced order types" \
  --body "## Summary
Implement advanced order types including bracket, OCO, and OTO orders.

## Changes
- Added BracketOrder, OcoOrder, OtoOrder types
- Implemented trailing stop and stop limit orders
- Added order validation

## Testing
- Unit tests: 15 new tests covering all order types
- Integration tests: Paper trading API integration test
- Manual testing: Verified against Alpaca paper trading

## Checklist
- [x] Version bumped in alpaca-http
- [x] README updated (alpaca-http/README.md)
- [x] All public items documented (no allow(missing_docs))
- [x] Unit tests added
- [x] Integration tests added
- [x] Examples created in examples/
- [x] make lint-fix pre-push passes

Closes #12" --base main

# 12. Merge (this automatically closes GitHub issue #12)
gh pr merge --squash --delete-branch

# 13. Release (one release per modified crate)
git checkout main && git pull
gh release create alpaca-http-v0.3.0 \
  --title "alpaca-http v0.3.0" \
  --notes "## Advanced Order Types

- Added bracket orders
- Added OCO (One-Cancels-Other) orders
- Added OTO (One-Triggers-Other) orders
- Added trailing stop orders
- Added stop limit orders

Closes #12"

# If alpaca-base was also modified, create its release too:
# gh release create alpaca-base-v0.2.0 --title "alpaca-base v0.2.0" --notes "..."

# 14. Cleanup (remove local issue file)
rm .issues/012-orders-advanced.md
git add .issues/012-orders-advanced.md
git commit -m "chore: remove resolved issue file 012"
git push origin main
```

## Session State

When resuming work, first check:
1. **Open PRs**: `gh pr list --state open` — Must be empty before new work
2. **Open Issues**: `gh issue list --state open` — Shows remaining work
3. Current branch: `git branch --show-current`
4. Pending changes: `git status`
5. Local issue files: `ls .issues/*.md | grep -v roadmap` — Cross-reference with GitHub issues
6. Roadmap status: `cat .issues/000-roadmap.md`

---

Begin by running the Initial Setup (if not done), then read the roadmap and select the first open issue from Phase 1.
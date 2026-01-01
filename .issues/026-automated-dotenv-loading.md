# Issue 026: Automated .env Loading in alpaca-base

## Description

Currently, examples and integrations must manually call `dotenv().ok()` to load environment variables from a `.env` file. This creates boiler-plate code in every example and can lead to frustration if forgotten.

The library should automatically attempt to load the `.env` file when `Credentials::from_env()` is called.

## Requirements

- [ ] Modify `Credentials::from_env()` in `alpaca-base/src/auth.rs` to call `dotenvy::dotenv().ok()` before reading environment variables.
- [ ] Ensure that failure to find a `.env` file does not cause an error (it should fall back to actual environment variables).
- [ ] Update examples to remove manual `dotenv()` calls where redundant.

## Priority
Medium

## Labels
`priority:medium`, `phase:5`, `type:infrastructure`

# HLEscape-CLI (Rust)

HLEscape-CLI is archived and no longer maintained.

It was the Rust command-line client of the HLEscape project, intended for technical users who could run a local workflow, manage environment variables, and interact with the Hyperliquid L1/API path directly.

For users who preferred a guided no-code experience, HLEscapeBot provided a Telegram-based interface.

## Current Status

HLEscape-CLI is no longer maintained.

HLEscapeBot is also no longer operated as an active recovery bot.

The current Hyperliquid mainnet environment has changed since the original implementation period, including account behavior, API signing rules, and transaction/action structures.

Because the legacy workflow is no longer maintained, it is not recommended for current Hyperliquid recovery execution.

## Current Recommendation

If you still need assistance with a Hyperliquid restricted-account or high-risk address warning, please use the actively maintained recovery bot:

**[Telegram: @HyperliquidRecoveryBot](https://t.me/HyperliquidRecoveryBot)**

## Archive Notes

This repository is preserved as an archive of the legacy HLEscape workflow.

The archived source code shows the old local CLI structure, including configuration loading, signer initialization, mainnet client setup, USDC send payload construction, and API submission flow.

Do not share sensitive wallet information, private support logs, or user-identifying information.

## Legacy Dependency Notes

This archive pins `hypersdk` to commit `2d24846a3c689401f6541a4da4e5b5054e88dddd`, dated 2025-12-30, to preserve the dependency set used by this codebase. Current Hyperliquid mainnet behavior may no longer match the signing and serialization assumptions used by this dependency.

Running this CLI against the current mainnet may fail with API signing, serialization, or compatibility errors.

## Build Instructions

The following instructions are preserved for research and archive review only. This CLI is not recommended for current mainnet recovery execution.

### Prerequisites

Ensure you have the Rust toolchain installed:
```bash
cargo --version # Rust 1.85+ required by the pinned hypersdk dependency
```

### Build Review

1. Clone the repository:
   ```bash
   git clone https://github.com/hlescape/hlescape-rust-cli.git
   cd hlescape-rust-cli
   ```

2. Run the compiler for archive review:
   ```bash
   cargo build --release
   ```

The source defaults to dry-run mode. Execution controls are left in the code for archive completeness.

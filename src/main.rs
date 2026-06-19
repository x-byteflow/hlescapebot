use anyhow::{anyhow, Result};
use dotenv::dotenv;
use hypersdk::{
    hypercore::{
        self, ARBITRUM_SIGNATURE_CHAIN_ID,
        types::{HyperliquidChain, UsdSend},
    },
    Address,
};
use rust_decimal::Decimal;
use std::env;
use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
struct CliConfig {
    signer_key: String,
    destination: Address,
    amount: Decimal,
    execution_mode: ExecutionMode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ExecutionMode {
    DryRun,
    Execute,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    print_banner();

    let config = load_config()?;
    let wallet = load_wallet(&config)?;
    let nonce = current_nonce()?;
    let request = build_usdc_send(&config, nonce);

    print_preflight_summary(&config, wallet.address());
    print_execution_plan(&request);

    match config.execution_mode {
        ExecutionMode::DryRun => {
            println!("[HLEscape] Dry-run mode enabled. No transaction was submitted.");
            println!("[HLEscape] Set EXECUTE=true to submit the USDC send action.");
        }
        ExecutionMode::Execute => {
            execute_usdc_send(wallet, request, nonce).await?;
        }
    }

    Ok(())
}

fn print_banner() {
    println!("============================================================");
    println!(" HLEscape-CLI");
    println!("============================================================");
    println!("[HLEscape] Initializing Hyperliquid USDC workflow...\n");
}

fn load_config() -> Result<CliConfig> {
    let signer_key = env::var("PRIVATE_KEY")?;
    let destination = Address::from_str(&env::var("TARGET_ADDRESS")?)?;
    let amount = Decimal::from_str(&env::var("AMOUNT").unwrap_or_else(|_| "50.0".to_string()))?;
    let execution_mode = parse_execution_mode();

    validate_amount(&amount)?;

    Ok(CliConfig {
        signer_key,
        destination,
        amount,
        execution_mode,
    })
}

fn parse_execution_mode() -> ExecutionMode {
    let should_execute = env::var("EXECUTE")
        .map(|value| value.eq_ignore_ascii_case("true"))
        .unwrap_or(false);

    if should_execute {
        ExecutionMode::Execute
    } else {
        ExecutionMode::DryRun
    }
}

fn validate_amount(amount: &Decimal) -> Result<()> {
    if amount <= &Decimal::ZERO {
        return Err(anyhow!("AMOUNT must be greater than zero"));
    }

    Ok(())
}

fn load_wallet(config: &CliConfig) -> Result<hypercore::PrivateKeySigner> {
    let wallet = hypercore::PrivateKeySigner::from_str(&config.signer_key)?;
    Ok(wallet)
}

fn current_nonce() -> Result<u64> {
    Ok(SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis() as u64)
}

fn build_usdc_send(config: &CliConfig, nonce: u64) -> UsdSend {
    println!(
        "[HLEscape] Packaging USDC send payload of {} USDC...",
        config.amount
    );

    UsdSend {
        hyperliquid_chain: HyperliquidChain::Mainnet,
        signature_chain_id: ARBITRUM_SIGNATURE_CHAIN_ID,
        destination: config.destination,
        amount: config.amount,
        time: nonce,
    }
}

fn print_preflight_summary(config: &CliConfig, account: Address) {
    println!("[HLEscape] Preflight summary:");
    println!("[HLEscape]   Network: Hyperliquid mainnet");
    println!("[HLEscape]   Account: {account:?}");
    println!("[HLEscape]   Settlement target: {:?}", config.destination);
    println!("[HLEscape]   Requested amount: {} USDC", config.amount);
    println!("[HLEscape]   Execution mode: {:?}\n", config.execution_mode);
}

fn print_execution_plan(request: &UsdSend) {
    println!("[HLEscape] Execution plan:");
    println!("[HLEscape]   1. Build hypercore signer");
    println!("[HLEscape]   2. Initialize Hyperliquid mainnet client");
    println!("[HLEscape]   3. Build USDC send payload");
    println!("[HLEscape]   4. Submit client.send_usdc(...)");
    println!("[HLEscape]   Destination: {:?}", request.destination);
    println!("[HLEscape]   Amount: {} USDC\n", request.amount);
}

async fn execute_usdc_send(
    wallet: hypercore::PrivateKeySigner,
    request: UsdSend,
    nonce: u64,
) -> Result<()> {
    println!("[HLEscape] Spawning Hyperliquid mainnet client...");
    let client = hypercore::mainnet();

    println!("[HLEscape] Broadcasting signed USDC send action...");
    match client.send_usdc(&wallet, request, nonce).await {
        Ok(()) => {
            println!("[HLEscape] Action submitted.");
        }
        Err(error) => {
            eprintln!("\n[!] Execution terminated with error from Hyperliquid Node:");
            eprintln!("{:?}", error);
            std::process::exit(1);
        }
    }

    Ok(())
}

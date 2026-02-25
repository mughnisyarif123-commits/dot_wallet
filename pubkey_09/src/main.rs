use anyhow::{Result, anyhow};
use hex;
use parity_scale_codec::{Decode, Encode};
use qrcode::QrCode;
use qrcode::render::unicode;
use sp_core::crypto::{Ss58AddressFormat, Ss58Codec};
use sp_core::hexdisplay::HexDisplay;
use sp_core::{Pair, blake2_256, sr25519};
use std::io::{self, Write};

#[derive(Decode, Debug)]
struct SimpleCall {
    call_index: [u8; 2],
}
const POLKADOT_GENESIS: &str = "91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3";

fn main() -> Result<()> {
    println!("╔══════════════════════════════════════════════════╗");
    println!("║          POLKADOT VAULT (OFFLINE MODE)           ║");
    println!("║          ADVANCED DECODER & SIGNER               ║");
    println!("╚══════════════════════════════════════════════════╝\n");
    print!("🔑 Masukkan 24 kata mnemonic: ");
    io::stdout().flush()?;
    let mut mnemonic_raw = String::new();
    io::stdin().read_line(&mut mnemonic_raw)?;
    let mnemonic = mnemonic_raw.trim().to_string();
    let mut current_pair = sr25519::Pair::from_phrase(&mnemonic, None)
        .map_err(|_| anyhow!("Mnemonic tidak valid!"))?.0;
    let mut current_path = String::from("None");
    loop {
        println!("\n[ MENU UTAMA ]");
        println!("1. Ganti Derivation Path (Account)");
        println!("2. Scan & Sign Transaction (Nova Wallet Mode)");
        println!("3. Tampilkan Info Alamat");
        println!("4. Keluar");
        print!("➡ Pilih (1-4): ");
        io::stdout().flush()?;
        let mut pilihan = String::new();
        io::stdin().read_line(&mut pilihan)?;
        match pilihan.trim() {
            "1" => {
                print!("➡ Masukkan Path (misal: //0 atau //polkadot): ");
                io::stdout().flush()?;
                let mut path_raw = String::new();
                io::stdin().read_line(&mut path_raw)?;
                let path = path_raw.trim();
                let seed = format!("{}{}", mnemonic, path);
                if let Ok(p) = sr25519::Pair::from_string(&seed, None) {
                    current_pair = p;
                    current_path = path.to_string();
                    println!("✅ Path diperbarui!");
                }
            }
            "2" => {
                if let Err(e) = menu_signer(&current_pair) {
                    println!("❌ Error: {}", e);
                }
            }
            "3" => tampilkan_data_ringkas(&current_pair, &current_path),
            "4" => break,
            _ => println!("⚠️ Pilihan tidak valid."),
        }
    }
    Ok(())
}

fn menu_signer(pair: &sr25519::Pair) -> Result<()> {
    println!("\n--- SCAN QR DARI NOVA WALLET ---");
    print!("➡ Masukkan Hex dari QR Nova (Format UOS): ");
    io::stdout().flush()?;
    let mut hex_input = String::new();
    io::stdin().read_line(&mut hex_input)?;
    let hex_str = hex_input.trim().trim_start_matches("0x");
    let uos_bytes = hex::decode(hex_str)?;
    if uos_bytes.len() < 5 { return Err(anyhow!("Payload terlalu pendek")); }
    let crypto_byte = uos_bytes[1]; 
    let payload_data = &uos_bytes[3..];
    println!("\n🔍 [ANALISIS TRANSAKSI]");
    if let Ok(call) = SimpleCall::decode(&mut &payload_data[..]) {
        let action = match call.call_index {
            [5, 0] => "Balances: Transfer",
            [5, 7] => "Balances: Transfer All",
            [18, 0] => "Staking: Bond",
            [18, 7] => "Staking: Nominate",
            _ => "Unknown Action (Check Polkadot Explorer)",
        };
        println!("📝 Jenis Transaksi: {}", action);
        println!("🆔 Index: Pallet {}, Method {}", call.call_index[0], call.call_index[1]);
    }
    if payload_data.len() >= 32 {
        let genesis = &payload_data[payload_data.len() - 32..];
        let genesis_hex = hex::encode(genesis);
        if genesis_hex == POLKADOT_GENESIS {
            println!("🌐 Network: Polkadot Mainnet (Verified)");
        } else {
            println!("⚠️ Peringatan: Genesis Hash ({}) tidak dikenal!", genesis_hex);
        }
    }
    print!("\n⚠️ Tanda tangani transaksi ini? (y/n): ");
    io::stdout().flush()?;
    let mut konfirm = String::new();
    io::stdin().read_line(&mut konfirm)?;
    if konfirm.trim().to_lowercase() == "y" {
        let signature = pair.sign(payload_data);
        let mut uos_sig = Vec::new();
        uos_sig.push(0x00); 
        uos_sig.push(crypto_byte); 
        uos_sig.push(0x01); 
        uos_sig.extend_from_slice(&signature.0);
        println!("\n✅ SIGNATURE BERHASIL DIBUAT");
        println!("Scan QR di bawah ini dengan Nova Wallet:");
        let qr = QrCode::new(&uos_sig)?;
        println!("{}", qr.render::<unicode::Dense1x2>().build());
        println!("Hex: 0x{}", hex::encode(&uos_sig));
    }
    Ok(())
}
fn tampilkan_data_ringkas(pair: &sr25519::Pair, path: &str) {
    let alamat_hex = format!("0x{}", HexDisplay::from(&pair.public().0));
    let alamat_dot = pair.public().to_ss58check_with_version(Ss58AddressFormat::custom(0));
    println!("\n╔════════════════ INFO AKUN ════════════════╗");
    println!("║ Path : {:<34} ║", path);
    println!("║ DOT  : {:<34} ║", alamat_dot);
    println!("║ Hex  : {:<34} ║", alamat_hex);
    println!("╚═══════════════════════════════════════════╝");
}
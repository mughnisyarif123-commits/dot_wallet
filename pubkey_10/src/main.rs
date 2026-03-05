use anyhow::{Result, anyhow};
use parity_scale_codec::Decode;
use qrcode::QrCode;
use qrcode::render::unicode;
use sp_core::crypto::{Pair, Ss58AddressFormat, Ss58Codec};
use sp_core::hexdisplay::HexDisplay;
use sp_core::sr25519;
use std::io::{self, Write};

#[derive(Decode, Debug)]
struct SimpleCall {
    call_index: [u8; 2],
}
fn main() -> Result<()> {
    println!("╔══════════════════════════════════════════════════╗");
    println!("║          POLKADOT VAULT (OFFLINE SIGNER)         ║");
    println!("║      SINKRON UOS FORMAT - SP-CORE EDITION        ║");
    println!("╚══════════════════════════════════════════════════╝\n");
    print!(" 🔑 Masukkan 24 kata mnemonic Anda: ");
    io::stdout().flush()?;
    let mut mnemonic_raw = String::new();
    io::stdin().read_line(&mut mnemonic_raw)?;
    let mnemonic_str = mnemonic_raw.trim().to_string();
    let (mut current_pair, mut current_address) = derivasi_kunci(&mnemonic_str, "//0")?;
    let mut current_path = String::from("//0");
    loop {
        println!("\n--- MENU UTAMA (OFFLINE) ---");
        println!("1. Keluar | 2. Ganti Path | 3. Sign QR (UOS Format) | 4. Info Akun");
        print!(" ➡️ Pilih (1-4): ");
        io::stdout().flush()?;
        let mut pilihan = String::new();
        io::stdin().read_line(&mut pilihan)?;
        match pilihan.trim() {
            "1" => break,
            "2" => {
                print!(" ➡️ Masukkan Path (contoh: //0 atau //polkadot): ");
                io::stdout().flush()?;
                let mut path_str = String::new();
                io::stdin().read_line(&mut path_str)?;
                let path_trimmed = path_str.trim();
                match derivasi_kunci(&mnemonic_str, path_trimmed) {
                    Ok((pair, addr)) => {
                        current_pair = pair;
                        current_address = addr;
                        current_path = path_trimmed.to_string();
                        println!("✅ Berhasil berpindah ke path: {}", current_path);
                    }
                    Err(e) => println!("❌ Gagal derivasi: {:?}", e),
                }
            }
            "3" => {
                print!(" ➡️ Masukkan Hex Transaksi (Payload dari Nova): ");
                io::stdout().flush()?;
                let mut input_hex = String::new();
                io::stdin().read_line(&mut input_hex)?;
                let input_hex = input_hex.trim().trim_start_matches("0x");
                if let Ok(payload_bytes) = hex::decode(input_hex) {
                    println!("\n🔍 [ANALISIS PAYLOAD]");
                    if let Ok(call) = SimpleCall::decode(&mut &payload_bytes[..]) {
                        println!(
                            "Detected Action: Pallet {}, Method {}",
                            call.call_index[0], call.call_index[1]
                        );
                        if call.call_index == [5, 0] {
                            println!("💡 Info: Ini terlihat seperti 'Balances:Transfer'");
                        }
                    } else {
                        println!("⚠️ Payload tidak bisa di-decode (bukan standar Call).");
                    }
                    print!("\n⚠️ Tanda tangani sekarang? (y/n): ");
                    io::stdout().flush()?;
                    let mut konfirmasi = String::new();
                    io::stdin().read_line(&mut konfirmasi)?;
                    if konfirmasi.trim().to_lowercase() == "y" {
                        let signature = current_pair.sign(&payload_bytes);
                        let mut uos_payload = Vec::new();
                        uos_payload.push(0x00);
                        uos_payload.push(0x01);
                        uos_payload.extend_from_slice(&signature.0);
                        println!("\n✅ UOS SIGNATURE GENERATED");
                        println!("Hex: {}", hex::encode(&uos_payload));
                        let qr = QrCode::new(&uos_payload)?;
                        println!("{}", qr.render::<unicode::Dense1x2>().build());
                        println!("(Scan QR ini dengan Nova Wallet)");
                    }
                } else {
                    println!("❌ Hex tidak valid!");
                }
            }
            "4" => {
                let pub_key = current_pair.public();
                println!("\n--- DETAIL KUNCI OFFLINE ---");
                println!("Public Key : 0x{}", HexDisplay::from(&pub_key.0));
                println!("SS58 (DOT) : {}", current_address);
                println!("Path       : {}", current_path);
            }
            _ => println!("⚠️ Pilihan tidak tersedia."),
        }
    }
    Ok(())
}
fn derivasi_kunci(mnemonic: &str, path: &str) -> Result<(sr25519::Pair, String)> {
    let full_phrase = if path.is_empty() {
        mnemonic.to_string()
    } else {
        format!("{}{}", mnemonic, path)
    };
    let pair = sr25519::Pair::from_string(&full_phrase, None)
        .map_err(|_| anyhow!("Mnemonic atau Path salah!"))?;
    let address = pair
        .public()
        .to_ss58check_with_version(Ss58AddressFormat::custom(0));
    Ok((pair, address))
}

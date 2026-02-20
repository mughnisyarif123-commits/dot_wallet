use anyhow::Result;
use qrcode::QrCode;
use qrcode::render::unicode;
use sp_core::crypto::Ss58AddressFormat;
use sp_core::hexdisplay::HexDisplay;
use sp_core::{Pair, crypto::Ss58Codec, sr25519};
use std::io::{self, Write};

fn main() -> Result<()> {
    println!("╔══════════════════════════════════════════════════╗");
    println!("║          POLKADOT VAULT (OFFLINE MODE)           ║");
    println!("║          ASLI BUATAN: MUH. MUGNI SYARIF          ║");
    println!("╠══════════════════════════════════════════════════╣");
    println!("║  Tujuan: Belajar Menjadi Pengembang Polkadot     ║");
    println!("╚══════════════════════════════════════════════════╝\n");

    print!(" 🔑 Masukkan 24 kata mnemonic Anda: ");
    io::stdout().flush()?;
    let mut mnemonic_raw = String::new();
    io::stdin().read_line(&mut mnemonic_raw)?;
    let mnemonic = mnemonic_raw.trim().to_string();

    let mut current_pair = sr25519::Pair::from_phrase(&mnemonic, None)
        .expect("mnemonic tidak valid")
        .0;
    let mut current_path = String::from("None");

    loop {
        println!("\n╔══════════════════ MENU UTAMA ════════════════════╗");
        println!("║ 1. Keluar (Tidak)                                ║");
        println!("║ 2. Derivation Path (Ganti Akun)                  ║");
        println!("║ 3. Tanda Tangani Transaksi (Signer)              ║");
        println!("║ 4. Tampilkan Alamat Saat Ini                     ║");
        println!("╚══════════════════════════════════════════════════╝");
        print!(" ➡️ Pilih menu (1-4): ");
        io::stdout().flush()?;

        let mut pilihan = String::new();
        io::stdin().read_line(&mut pilihan)?;

        match pilihan.trim() {
            "1" => {
                println!("Sesi berakhir. Aman & Tetap Offline!");
                break;
            }
            "2" => {
                println!("\n╔════════════════ JALUR DERIVASI ══════════════════╗");
                println!("║ Contoh: //polkadot atau //substrate              ║");
                println!("╚══════════════════════════════════════════════════╝");
                print!(" ➡️ Masukkan Path: ");
                io::stdout().flush()?;

                let mut path_raw = String::new();
                io::stdin().read_line(&mut path_raw)?;
                let path = path_raw.trim();

                let seed = format!("{}{}", mnemonic, path);
                match sr25519::Pair::from_string(&seed, None) {
                    Ok(p) => {
                        current_pair = p;
                        current_path = path.to_string();
                        println!("✅ Path berhasil diterapkan!");

                        // SUB-MENU QR
                        loop {
                            println!("\nApakah anda ingin ke:");
                            println!("1. Tidak (KEMBALI ke pilihan utama)");
                            println!("2. Tampilkan QR Alamat HEX");
                            println!("3. Tampilkan QR Alamat SUBX");
                            println!("4. Tampilkan QR Alamat DOT");
                            println!("5. Tampilkan QR Alamat Polkadot untuk Nova (Vault Mode)");
                            print!("➡️ Pilih (1-5): ");
                            io::stdout().flush()?;

                            let mut sub_pilihan = String::new();
                            io::stdin().read_line(&mut sub_pilihan)?;

                            match sub_pilihan.trim() {
                                "1" => break,
                                "2" => cetak_qr(
                                    "HEX",
                                    &format!("0x{}", HexDisplay::from(&current_pair.public().0)),
                                )?,
                                "3" => cetak_qr("SUBX", &current_pair.public().to_ss58check())?,
                                "4" => cetak_qr(
                                    "DOT",
                                    &current_pair
                                        .public()
                                        .to_ss58check_with_version(Ss58AddressFormat::custom(0)),
                                )?,
                                "5" => {
                                    let hex =
                                        format!("0x{}", HexDisplay::from(&current_pair.public().0));
                                    let dot = current_pair
                                        .public()
                                        .to_ss58check_with_version(Ss58AddressFormat::custom(0));
                                    cetak_qr(
                                        "NOVA (VAULT MODE)",
                                        &format!("substrate:{}:{}", dot, hex),
                                    )?;
                                }
                                _ => println!("⚠️ Pilihan tidak valid."),
                            }
                        }
                    }
                    Err(_) => println!("❌ Error: Path tidak valid!"),
                }
            }
            "3" => {
                println!("\n--- SIGNER MODE ---");
                print!(" ➡️ Masukkan Data Transaksi (Hex/Raw): ");
                io::stdout().flush()?;
                let mut pesan_raw = String::new();
                io::stdin().read_line(&mut pesan_raw)?;
                let pesan = pesan_raw.trim();
                let signature = current_pair.sign(pesan.as_bytes());
                let hex_sign = format!("{}", HexDisplay::from(&signature.0));
                cetak_qr("SIGNATURE QR", &hex_sign)?;
                println!("Hex Signature:\n{}", hex_sign);
            }
            "4" => {
                tampilkan_data_ringkas(&current_pair, &current_path);
            }
            _ => println!("⚠️ Pilihan tidak valid, silakan coba lagi."),
        }
    }
    Ok(())
}

fn cetak_qr(label: &str, data: &str) -> Result<()> {
    let qr = QrCode::new(data)?;
    let image = qr.render::<unicode::Dense1x2>().build();
    println!("\n--- {} ---", label);
    println!("{}", image);
    println!("Data: {}", data);
    Ok(())
}

fn tampilkan_data_ringkas(pair: &sr25519::Pair, path: &str) {
    let alamat_subx = pair.public().to_ss58check();
    let alamat_hex = format!("0x{}", HexDisplay::from(&pair.public().0));
    println!("\n╔══════════════════ INFO AKUN ═════════════════════╗");
    println!("║ Path: {:<42} ║", path);
    println!("║ Hex:  {:<42} ║", alamat_hex);
    println!("║ Subx: {:<42} ║", alamat_subx);
    println!("╚══════════════════════════════════════════════════╝");
}

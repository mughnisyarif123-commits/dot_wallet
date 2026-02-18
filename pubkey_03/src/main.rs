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
    let (pair, _) = sr25519::Pair::from_phrase(&mnemonic, None).expect("mnemonic tidak valid");
    let alamat_subx = pair.public().to_ss58check();
    let alamat_hex = format!("0x{}", HexDisplay::from(&pair.public().0));
    let alamat_dot = pair
        .public()
        .to_ss58check_with_version(Ss58AddressFormat::custom(0));
    let alamat_nova = format!("substrate:{}:{}", alamat_dot, alamat_hex);
    let qr_dot = QrCode::new(&alamat_dot)?;
    let foto_dot = qr_dot.render::<unicode::Dense1x2>().build();
    let qr2_dot = QrCode::new(&alamat_nova)?;
    let foto2_dot = qr2_dot.render::<unicode::Dense1x2>().build();

    println!("╔═══════════════════ HASIL DATA ═══════════════════╗");
    println!("║                                                  ║");
    println!("║  Public Key (hex mentah):                        ║");
    println!("║  {:?}  ║", pair.public());
    println!("║                                                  ║");
    println!("║  Public key (hex):                               ║");
    println!("║  {}  ║", alamat_hex);
    println!("║                                                  ║");
    println!("║  Alamat Substrate:                               ║");
    println!("║  {}  ║", alamat_subx);
    println!("║                                                  ║");
    println!("║  Alamat Polkadot:                                ║");
    println!("║  {}  ║", alamat_dot);
    println!("║                                                  ║");
    println!("║  Alamat untuk Nova:                              ║");
    println!("║  {}  ║", alamat_nova);
    println!("╚══════════════════════════════════════════════════╝");
    println!("║                                                  ║");
    println!("║  Jika ingin di scan Alamat (Polkadot):           ║");
    println!("{}", foto_dot);
    println!("║                                                  ║");
    println!("║  Scan Alamat Untuk Nova Wallet:                  ║");
    println!("{}", foto2_dot);

    println!("╔════════════════ JALUR DERIVASI ══════════════════╗");
    println!("║ Contoh: //polkadot atau //substrate              ║");
    println!("╚══════════════════════════════════════════════════╝");
    print!(" ➡️ Masukkan Path (kosongkan jika tidak ada): ");

    io::stdout().flush()?;
    let mut path_raw = String::new();
    io::stdin().read_line(&mut path_raw)?;
    let path = path_raw.trim().to_string();
    let seed = format!("{}{}", mnemonic.trim(), path);
    let pair2 = sr25519::Pair::from_string(&seed, None).expect("path tidak valid");
    let almt_hex = format!("0x{}", HexDisplay::from(&pair2.public().0));
    let almt2_subx = pair2.public().to_ss58check();
    let almt3_dot = pair2
        .public()
        .to_ss58check_with_version(Ss58AddressFormat::custom(0));
    let almt4_nova = format!("substrate:{}:{}", almt3_dot, almt_hex);
    let qr_dotpath = QrCode::new(&almt3_dot)?;
    let foto_dotpath = qr_dotpath.render::<unicode::Dense1x2>().build();
    let qr2_dotpath = QrCode::new(&almt4_nova)?;
    let foto2_dotpath = qr2_dotpath.render::<unicode::Dense1x2>().build();

    println!("╔═══════════════════ HASIL DATA ═══════════════════╗");
    println!("║                                                  ║");
    println!("║  Public Key (hex mentah):                        ║");
    println!("║  {:?}  ║", pair2.public());
    println!("║                                                  ║");
    println!("║  Public key (hex):                               ║");
    println!("║  {}  ║", almt_hex);
    println!("║                                                  ║");
    println!("║  Alamat Substrate: {:<30}║", path);
    println!("║  {}  ║", almt2_subx);
    println!("║                                                  ║");
    println!("║  Alamat Polkadot: {:<31}║", path);
    println!("║  {}  ║", almt3_dot);
    println!("║                                                  ║");
    println!("║  Alamat untuk Nova: {:<29}║", path);
    println!("║  {}  ║", almt4_nova);
    println!("╚══════════════════════════════════════════════════╝");
    println!("║                                                  ║");
    println!("║  Jika ingin di scan Alamat (Polkadot Path):      ║");
    println!("{}", foto_dotpath);
    println!("║                                                  ║");
    println!("║  Scan Alamat Untuk Nova Wallet:                  ║");
    println!("{}", foto2_dotpath);
    println!("╔══════════════════════ SIGNER ════════════════════╗");
    println!("║ Mode: Offline Signature & Key Generation         ║");
    println!("╚══════════════════════════════════════════════════╝");
    println!("║  Path Signer: {:<35}║", path);
    println!("║                                                  ║");
    println!("║  Masukkan Data Transaksi Dari HP Online:         ║");

    io::stdout().flush()?;
    let mut pesan_raw = String::new();
    io::stdin().read_line(&mut pesan_raw)?;
    let pesan = pesan_raw.trim().to_string();
    let signature = pair2.sign(pesan.as_bytes());
    let hex_sign = format!("0x{}", HexDisplay::from(&signature.0));
    let qr_sign = QrCode::new(&hex_sign)?;
    let image_sign = qr_sign.render::<unicode::Dense1x2>().build();

    println!("║                                                  ║");
    println!("║                  SIGNER OUTPUT                   ║");
    println!("║                                                  ║");
    println!("{}", image_sign);
    println!("║                                                  ║");
    println!("║  {}  ║", hex_sign);
    Ok(())
}

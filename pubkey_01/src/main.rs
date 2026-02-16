use sp_core::{sr25519, Pair, crypto::Ss58Codec};
use sp_core::crypto::Ss58AddressFormat;
use qrcode::QrCode;
use anyhow::Result;
use sp_core::hexdisplay::HexDisplay;
use qrcode::render::unicode;
use std::io::{self, Write};

fn main() -> Result<()> {
	println!("masukkan 24 kata mnemonic:");
	io::stdout().flush()?;
	let mut mnemonic_raw = String::new();
	io::stdin().read_line(&mut mnemonic_raw)?;
	let mnemonic = mnemonic_raw.trim().to_string();
	let (pair, _) = sr25519::Pair::from_phrase(&mnemonic, None).expect("mnemonic tidak valid");
	println!("public key (hex); {:?}", pair.public());

	let address = pair.public().to_ss58check();
	println!("address (ss58): {}", address);

	let polkadot_address = pair.public().to_ss58check_with_version(Ss58AddressFormat::custom(0));
	println!("address (polkadot): {}", polkadot_address);
	let code = QrCode::new(&polkadot_address)?;
	let image = code.render::<unicode::Dense1x2>().build();
	println!("scan alamat ini di nova wallet (polkadot vault mode):");
	println!("{}", image);
	let hex = format!("0x{}", HexDisplay::from(&pair.public().0));
	let substrate = format!("substrate:{}:{}", polkadot_address, hex);
	println!("{}", substrate);
	let code2 = QrCode::new(&substrate)?;
	let image2 = code2.render::<unicode::Dense1x2>().build();
	println!(" scan alamat ini di nova wallet :");
	println!("{}", image2);

	println!("-----------------------------------------------------------------------");
	println!("---transaksi signer---");

	let pesan = "data_transaksi_dari_hp_online";
	let signature = pair.sign(pesan.as_bytes());
	println!("---signer output---");
	println!("Signature: 0x{}", HexDisplay::from(&signature.0));
	let sign_hex = format!("0x{}", HexDisplay::from(&signature.0));
	let sign_qr = QrCode::new(&sign_hex)?;
	let sign_image = sign_qr.render::<unicode::Dense1x2>().build();
	println!("scan signature ini di hp online:");
	println!("{}", sign_image);
	println!("Signature: {}", sign_hex);
	
	Ok(())
}

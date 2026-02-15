use sp_core::{sr25519, Pair, crypto::Ss58Codec};
use sp_core::crypto::Ss58AddressFormat;
use std::io::{self, Write};
use anyhow::Result;
use qrcode::QrCode;
use sp_core::hexdisplay::HexDisplay;

fn main() -> Result<()> {
	println!("=");
	println!("=");
	println!("---polkadot vault---");
	println!("---asli buatan muh. mughni syarif---");
	println!("=");
	println!("masukkan 24 kata mnemonic:");
	io::stdout().flush()?;
	let mut mnemonic_raw = String::new();
	io::stdin().read_line(&mut mnemonic_raw)?;
	let mnemonic = mnemonic_raw.trim().to_string();
	let (pair, _) = sr25519::Pair::from_phrase(&mnemonic, None).expect("mnemonic tidak valid");
	println!("=");
	println!("---hasil alamat---");
	println!("public key (hex); {:?}", pair.public());

	let address = pair.public().to_ss58check();
	println!("substrate (ss58): {}", address);
	let code = QrCode::new(&address)?;
	let image = code.render::<qrcode::render::unicode::Dense1x2>().build();
	println!("=");
	println!("jika ingin di scan alamatnya (substrate): {}", address);
	println!("{}", image);
	let hex = format!("0x{}", HexDisplay::from(&pair.public().0));
	let substrate = format!("substrate:{}:{}", address, hex);
	println!("{}", substrate);
	let code2 = QrCode::new(&substrate)?;
	let image2 = code2.render::<qrcode::render::unicode::Dense1x2>().build();
	println!("scan alamat ini di nova wallet (polkadot vault mode):");
	println!("{}", image2);

	let polkadot_address = pair.public().to_ss58check_with_version(Ss58AddressFormat::custom(0));
	println!("address (polkadot): {}", polkadot_address);
	let code_polkadot = QrCode::new(&polkadot_address)?;
	let image_polkadot = code_polkadot.render::<qrcode::render::unicode::Dense1x2>().build();
	println!("=");
	println!("jika ingin di scan alamatnya (polkadot): {}", polkadot_address);
	println!("{}", image_polkadot);
	let substrate2 = format!("substrate:{}:{}", polkadot_address, hex);
	println!("{}", substrate2);
	let code2_polkadot = QrCode::new(&substrate2)?;
	let image2_polkadot = code2_polkadot.render::<qrcode::render::unicode::Dense1x2>().build();
	println!("scan alamat ini di nova wallet (polkadot vault mode):");
	println!("{}", image2_polkadot);
	
	println!("---jalur derivasi path---");
	println!("masukkan kata ke 25 (passphrase / kosongkan jika tidak ada):");
	io::stdout().flush()?;
	let mut path_raw2 = String::new();
	io::stdin().read_line(&mut path_raw2)?;
	let path2 = path_raw2.trim().to_string();
	let seed = format!("{}{}", mnemonic.trim(), path2);
	let pair2 = sr25519::Pair::from_string(&seed, None).expect("path tidak valid");
	println!("---derivasi path---");
	println!("public key (hex); {:?}", pair2.public());

	let address_path2 = pair2.public().to_ss58check();
	println!("substrate (ss58): {}", address_path2);
	let code_address_path2 = QrCode::new(&address_path2)?;
	let image_path2 = code_address_path2.render::<qrcode::render::unicode::Dense1x2>().build();
	println!("=");
	println!("jika ingin di scan alamatnya (substrate path): {}", address_path2);
	println!("{}", image_path2);
	let hex2 = format!("0x{}", HexDisplay::from(&pair2.public().0));
	let substrate3 = format!("substrate:{}:{}", address_path2, hex2);
	println!("{}", substrate3);
	let code2_address_path2 = QrCode::new(&substrate3)?;
	let image2_address_path2 = code2_address_path2.render::<qrcode::render::unicode::Dense1x2>().build();
	println!("scan alamat ini di nova wallet (polkadot vault mode):");
	println!("{}", image2_address_path2);
	
	let dot_path2 = pair2.public().to_ss58check_with_version(Ss58AddressFormat::custom(0));
	println!("address (path): {}", dot_path2);
	let code_dot_path2 = QrCode::new(&dot_path2)?;
	let image_dot_path2 = code_dot_path2.render::<qrcode::render::unicode::Dense1x2>().build();
	println!("jika ingin di scan alamatnya (polkadot path): {}", dot_path2);
	println!("{}", image_dot_path2);
	let substrate4 = format!("substrate:{}:{}", dot_path2, hex2);
	println!("{}", substrate4);
	let code2_dot_path2 = QrCode::new(&substrate4)?;
	let image2_dot_path2 = code2_dot_path2.render::<qrcode::render::unicode::Dense1x2>().build();
	println!("scan alamat ini di nova wallet (polkadot vault mode):");
	println!("{}", image2_dot_path2);
	
	println!("---jalur derivasi path---");
	println!("masukkan kata ke 25 (passphrase / kosongkan jika tidak ada):");
	io::stdout().flush()?;
	let mut path3_raw = String::new();
	io::stdin().read_line(&mut path3_raw)?;
	let path3 = path3_raw.trim().to_string();
	let seed3 = format!("{}{}", mnemonic.trim(), path3);
	let pair3 = sr25519::Pair::from_string(&seed3, None).expect("path tidak valid");
	println!("---derivasi path---");
	println!("public key (hex): {:?}", pair3.public());
	
	let address_path3 = pair3.public().to_ss58check();
	println!("address (ss58): {}", address_path3);
	println!("=");
	let code_address_path3 = QrCode::new(&address_path3)?;
	let image_address_path3 = code_address_path3.render::<qrcode::render::unicode::Dense1x2>().build();
	println!("jika ingin di scan alamatnya (substrate path): {}", address_path3);
	println!("{}", image_address_path3);
	let hex3 = format!("0x{}", HexDisplay::from(&pair3.public().0));
	let substrate5 = format!("substrate:{}:{}", address_path3, hex3);
	println!("{}", substrate5);
	let code2_address_path3 = QrCode::new(&substrate5)?;
	let image2_address_path3 = code2_address_path3.render::<qrcode::render::unicode::Dense1x2>().build();
	println!("scan alamat ini di nova wallet (polkadot vault mode):");
	println!("{}", image2_address_path3);
	
	let dot_path3 = pair3.public().to_ss58check_with_version(Ss58AddressFormat::custom(0));
	println!("address (path): {}", dot_path3);
	let code_dot_path3 = QrCode::new(&dot_path3)?;
	let image_dot_path3 = code_dot_path3.render::<qrcode::render::unicode::Dense1x2>().build();
	println!("jika ingin di scan alamatnya (polkadot path): {}", dot_path3);
	println!("{}", image_dot_path3);
	let substrate6 = format!("substrate:{}:{}", dot_path3, hex3);
	println!("{}", substrate6);
	let code2_dot_path3 = QrCode::new(&substrate6)?;
	let image2_dot_path3 = code2_dot_path3.render::<qrcode::render::unicode::Dense1x2>().build();
	println!("scan alamat ini di nova wallet (polkadot vault mode):");
	println!("{}", image2_dot_path3);

	Ok(())
	
}

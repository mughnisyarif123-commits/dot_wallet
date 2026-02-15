use sp_core::{sr25519, Pair, crypto::Ss58Codec};
use sp_core::crypto::Ss58AddressFormat;
use std::io::{self, Write};
use anyhow::Result;
use qrcode::QrCode;
use sp_core::hexdisplay::HexDisplay;

fn main() -> Result<()> {
	println!("---polkadot vault---");
	println!("---asli buatan muh. mughni syarif---");
	println!("");
	println!("masukkan 24 kata mnemonic:");
	io::stdout().flush()?;
	let mut mnemonic_raw = String::new();
	io::stdin().read_line(&mut mnemonic_raw)?;
	let mnemonic = mnemonic_raw.trim().to_string();
	let (pair, _) = sr25519::Pair::from_phrase(&mnemonic, None).expect("mnemonic tidak valid");
	println!("");
	println!("---hasil alamat---");
	println!("public key (hex); {:?}", pair.public());

	let address = pair.public().to_ss58check();
	println!("address (ss58): {}", address);
	let code = QrCode::new(&address)?;
	let image = code.render::<qrcode::render::unicode::Dense1x2>().build();
	println!("jika ingin di scan alamatnya (address):");
	println!("{}", image);
	let hex = format!("0x{}", HexDisplay::from(&pair.public().0));
	let substrate = format!("substrate:{}:{}", address, hex);
	println!("{}", substrate);
	let code2 = QrCode::new(&substrate)?;
	let image2 = code2.render::<qrcode::render::unicode::Dense1x2>().build();
	println!("scan alamatnya ini di nova wallet (polkadot vault mode):");
	println!("{}", image2);

	let polkadot_address = pair.public().to_ss58check_with_version(Ss58AddressFormat::custom(0));
	println!("address (polkadot): {}", polkadot_address);
	

	println!("");
	println!("");
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
	println!("address (ss58): {}", address_path2);
	let dot_path2 = pair2.public().to_ss58check_with_version(Ss58AddressFormat::custom(0));
	println!("address (path): {}", dot_path2);

	println!("");
	println!("");
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
	let dot_path3 = pair3.public().to_ss58check_with_version(Ss58AddressFormat::custom(0));
	println!("address (path): {}", dot_path3);

	Ok(())
	
}

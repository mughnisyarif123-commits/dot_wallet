use sp_core::{sr25519, Pair, crypto::Ss58Codec};
use sp_core::crypto::Ss58AddressFormat;
use qrcode::QrCode;
use anyhow::Result;
use sp_core::hexdisplay::HexDisplay;

fn main() -> Result<()> {
	let mnemonic = "private bird pet season gadget north render kidney enjoy amazing often agent oval apart brown stadium code tomorrow dad blush debate crucial lemon fix";
	let (pair, _) = sr25519::Pair::from_phrase(mnemonic, None).expect("mnemonic tidak valid");
	println!("public key (hex); {:?}", pair.public());

	let address = pair.public().to_ss58check();
	println!("address (ss58): {}", address);

	let polkadot_address = pair.public().to_ss58check_with_version(Ss58AddressFormat::custom(0));
	println!("address (polkadot): {}", polkadot_address);
	let code = QrCode::new(&polkadot_address)?;
	let image = code.render::<qrcode::render::unicode::Dense1x2>().build();
	println!("scan alamat ini di nova wallet (polkadot vault mode):");
	println!("{}", image);
	let hex = format!("0x{}", HexDisplay::from(&pair.public().0));
	let substrate = format!("substrate:{}:{}", polkadot_address, hex);
	println!("{}", substrate);
	let code2 = QrCode::new(&substrate)?;
	let image2 = code2.render::<qrcode::render::unicode::Dense1x2>().build();
	println!(" scan alamat ini di nova wallet :");
	println!("{}", image2);
	
	Ok(())
}

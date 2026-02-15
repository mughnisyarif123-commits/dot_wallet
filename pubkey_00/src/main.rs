use sp_core::{sr25519, Pair, crypto::Ss58Codec};
use sp_core::crypto::Ss58AddressFormat;

fn main() {
	let mnemonic = "private bird pet season gadget north render kidney enjoy amazing often agent oval apart brown stadium code tomorrow dad blush debate crucial lemon fix";
	let (pair, _) = sr25519::Pair::from_phrase(mnemonic, None).expect("mnemonic tidak valid");
	
	println!("public key (hex); {:?}", pair.public());

	let address = pair.public().to_ss58check();
	println!("address (ss58): {}", address);

	let polkadot_address = pair.public().to_ss58check_with_version(Ss58AddressFormat::custom(0));
	println!("address (polkadot): {}", polkadot_address);

	let path = "//polkadot";
	let seed = format!("{}{}", mnemonic.trim(), path);
	let pair2 = sr25519::Pair::from_string(&seed, None).expect("path tidak valid");
	let dot_path = pair2.public().to_ss58check_with_version(Ss58AddressFormat::custom(0));
	println!("address (path//polkadot): {}", dot_path);

	let path3 = "//dot";
	let seed3 = format!("{}{}", mnemonic.trim(), path3);
	let pair3 = sr25519::Pair::from_string(&seed3, None).expect("path tidak valid");
	let dot_path3 = pair3.public().to_ss58check_with_version(Ss58AddressFormat::custom(0));
	println!("address (path//dot): {}", dot_path3);
	
}

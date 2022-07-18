use rsa::pkcs1::DecodeRsaPublicKey;
use rsa::{ PublicKey, RsaPrivateKey, RsaPublicKey, PaddingScheme };
use rsa::{ pkcs8::EncodePrivateKey, pkcs8::EncodePublicKey, pkcs8::LineEnding, pkcs8::DecodePublicKey };
use rand::{ thread_rng };
use rand::rngs::ThreadRng;

#[derive(Debug, Clone)]
pub struct RSA {
	pub key: RsaPublicKey,
	pub priv_key: RsaPrivateKey,
}

#[derive(Debug, Clone)]
pub struct ClientRSA {
	pub key: RsaPublicKey,
}

impl ClientRSA {
	pub fn new(key: String) -> ClientRSA {
		let key = RsaPublicKey::from_public_key_pem(&key).unwrap();
		ClientRSA {
			key,
		}
	}

	pub fn encrypt(&self, message: &str) -> Vec<u8> {
		let mut rng = random_seed();
		let padding = padding_scheme();
		let enc_data = self.key.encrypt(&mut rng, padding, message.as_bytes()).expect("failed to encrypt");
	
		enc_data
	}
}

impl RSA {
	pub fn new() -> RSA {
		// generate a random number from thread, seeded by the system.
		let mut rng = random_seed();
		let bits = 2048;

		let private_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
		let public_key = RsaPublicKey::from(&private_key);

		RSA {
			key: public_key,
			priv_key: private_key,
		}
	}

	fn print(&self) {
		let private = self.priv_key.to_pkcs8_pem(LineEnding::LF)
			.unwrap().to_string();
		let public = self.key.to_public_key_pem(LineEnding::LF)
			.unwrap().to_string();
	
		println!("Public Key: {:?}", public);
		println!("Private Key: {:?}", private);
	}


	pub fn get_public_key(&self) -> String {
		let public = self.key.to_public_key_pem(LineEnding::LF)
			.unwrap().to_string();
		public
	}

	pub fn encrypt(&self, message: &str) -> Vec<u8> {
		let mut rng = random_seed();
		let padding = padding_scheme();
		let enc_data = self.key.encrypt(&mut rng, padding, message.as_bytes()).expect("failed to encrypt");
	
		enc_data
	}

	pub fn decrypt(&self, cipher: &[u8]) -> Vec<u8> {
		let padding = padding_scheme();
		let dec_data = self.priv_key.decrypt(padding, cipher).expect("failed to decrypt");

		dec_data
	}
}

fn random_seed () -> ThreadRng {
	thread_rng()
}

fn padding_scheme() -> PaddingScheme {
	PaddingScheme::new_pkcs1v15_encrypt()
}

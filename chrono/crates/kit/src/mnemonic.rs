use bip39::{Mnemonic, MnemonicType};
use bip39::Language::English;

pub fn generate_mnemonic() -> String {
    let mnemonic = Mnemonic::new(MnemonicType::Words12, English);
    mnemonic.to_string()
}

pub fn validate_mnemonic(mnemonic: &str) -> bool {
    Mnemonic::from_phrase(mnemonic, English).is_ok()
}
use password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, Salt, SaltString};

use argon2::{Algorithm, Argon2, AssociatedData, KeyId, ParamsBuilder, Version};
pub fn hash_password(password: &str) -> String {
    let ctx = get_argon2();
    let salt = vec![0; 8];
    let salt_string = SaltString::encode_b64(&salt).unwrap();
    ctx
        .hash_password(password.as_bytes(), &salt_string)
        .unwrap()
        .to_string()
}

pub fn verify_password(hash: &str, password: &str) -> bool {
    let ctx = get_argon2();
    ctx
        .verify_password(password.as_bytes(), &PasswordHash::new(hash).unwrap()).is_ok()

}

fn get_argon2<'a>() -> Argon2<'a> {
    let params = ParamsBuilder::new()
        .m_cost(32)
        .t_cost(2)
        .p_cost(3)
        .data(AssociatedData::new(&[0x0f; 6]).unwrap())
        .keyid(KeyId::new(&[0xf0; 4]).unwrap())
        .build()
        .unwrap();

    Argon2::new(Algorithm::Argon2d, Version::V0x10, params)
}
#[test]
fn test() {
    let input_password = "password";
    let data = hash_password(input_password);
    println!("{}",data);
    let input_password = "password";
    assert_eq!(verify_password(&data, input_password), true);
}
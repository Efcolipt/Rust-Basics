use hmac::{Hmac, Mac};
use sha1::Sha1;

type HmacSha1 = Hmac<Sha1>;

pub fn gen_hotp(secret: &[u8], counter: u64, digits: u32) -> u32 {
    assert!((6..=8).contains(&digits), "digits must be 6..=8");

    let counter_bytes = counter.to_be_bytes();

    let result = HmacSha1::new_from_slice(secret)
        .expect("Invalid key")
        .chain_update(counter_bytes)
        .finalize()
        .into_bytes();

    let offset = (result[result.len() - 1] & 0x0f) as usize;

    let binary = ((result[offset] as u32 & 0x7f) << 24)
        | ((result[offset + 1] as u32) << 16)
        | ((result[offset + 2] as u32) << 8)
        | (result[offset + 3] as u32);

    binary % 10_u32.checked_pow(digits).expect("Digits too large")
}

pub fn verify_hotp(
    secret: &[u8],
    code: u32,
    current_counter: u64,
    digits: u32,
    max_window_offset: Option<u64>,
) -> Option<u64> {
    let offset = max_window_offset.unwrap_or(0);

    for current_offset in 0..=offset {
        let counter = current_counter + current_offset;
        let expected = gen_hotp(secret, counter, digits);

        if expected == code {
            return Some(counter + 1);
        }
    }

    None
}

pub fn gen_hotp_uri(
    issuer: &str,
    account: &str,
    secret_base32: &str,
    counter: u64,
    digits: u32,
) -> String {
    format!(
        "otpauth://hotp/{}:{}?secret={}&issuer={}&counter={}&digits={}&algorithm=SHA1",
        issuer, account, secret_base32, issuer, counter, digits
    )
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hotp_generates_expected_codes() {
        let secret = b"12345678901234567890";

        let expected = [
            755224,
            287082,
            359152,
            969429,
            338314,
            254676,
            287922,
            162583,
            399871,
            520489,
        ];

        for (counter, expected_code) in expected.iter().enumerate() {
            let code = gen_hotp(secret, counter as u64, 6);
            assert_eq!(code, *expected_code, "failed at counter={counter}");
        }
    }
}
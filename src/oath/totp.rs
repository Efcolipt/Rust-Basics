use std::time::{SystemTime, UNIX_EPOCH};

use crate::oath::hotp::gen_hotp;

fn get_unix_time() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Invalid system time")
        .as_secs()
}

pub fn gen_totp_at(secret: &[u8], digits: u32, period: u64, unix_time: u64) -> u32 {
    gen_hotp(secret, unix_time / period, digits)
}

pub fn verify_totp_at(
    secret: &[u8],
    code: u32,
    digits: u32,
    period: u64,
    window: u8,
    unix_time: u64,
) -> bool {
    let counter = unix_time / period;

    if gen_hotp(secret, counter, digits) == code {
        return true;
    }

    for offset in 1..=window {
        let current_offset = offset as u64;

        if gen_hotp(secret, counter + current_offset, digits) == code {
            return true;
        }

        if let Some(backward_counter) = counter.checked_sub(current_offset) {
            if gen_hotp(secret, backward_counter, digits) == code {
                return true;
            }
        }
    }

    false
}

pub fn gen_totp(secret: &[u8], digits: u32, period: u64) -> u32 {
    gen_totp_at(secret, digits, period, get_unix_time())
}

pub fn verify_totp(secret: &[u8], code: u32, digits: u32, period: u64, window: u8) -> bool {
    verify_totp_at(secret, code, digits, period, window, get_unix_time())
}

pub fn gen_totp_uri(
    issuer: &str,
    account_name: &str,
    secret_base32: &str,
    digits: u32,
    period: u64,
) -> String {
    format!(
        "otpauth://totp/{}:{}?secret={}&issuer={}&algorithm=SHA1&digits={}&period={}",
        issuer, account_name, secret_base32, issuer, digits, period
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn totp_at_matches_hotp_counter() {
        let secret = b"12345678901234567890";
        let digits = 6;
        let period = 30;
        let unix_time = 90;

        let totp_code = gen_totp_at(secret, digits, period, unix_time);
        let hotp_code = gen_hotp(secret, unix_time / period, digits);

        assert_eq!(totp_code, hotp_code);
    }

    #[test]
    fn totp_is_same_within_same_time_window() {
        let secret = b"12345678901234567890";
        let digits = 6;
        let period = 30;

        let code1 = gen_totp_at(secret, digits, period, 60);
        let code2 = gen_totp_at(secret, digits, period, 61);
        let code3 = gen_totp_at(secret, digits, period, 89);

        assert_eq!(code1, code2);
        assert_eq!(code2, code3);
    }

    #[test]
    fn totp_changes_on_next_time_window() {
        let secret = b"12345678901234567890";
        let digits = 6;
        let period = 30;

        let code1 = gen_totp_at(secret, digits, period, 89);
        let code2 = gen_totp_at(secret, digits, period, 90);

        assert_ne!(code1, code2);
    }

    #[test]
    fn verify_totp_accepts_current_window_code() {
        let secret = b"12345678901234567890";
        let digits = 6;
        let period = 30;
        let unix_time = 90;

        let code = gen_totp_at(secret, digits, period, unix_time);

        assert!(verify_totp_at(secret, code, digits, period, 1, unix_time));
    }

    #[test]
    fn verify_totp_accepts_previous_window_code() {
        let secret = b"12345678901234567890";
        let digits = 6;
        let period = 30;

        let server_time = 90;
        let previous_code = gen_totp_at(secret, digits, period, 89);

        assert!(verify_totp_at(
            secret,
            previous_code,
            digits,
            period,
            1,
            server_time
        ));
    }

    #[test]
    fn verify_totp_rejects_code_outside_window() {
        let secret = b"12345678901234567890";
        let digits = 6;
        let period = 30;

        let server_time = 90;
        let old_code = gen_totp_at(secret, digits, period, 29);

        assert!(!verify_totp_at(
            secret,
            old_code,
            digits,
            period,
            1,
            server_time
        ));
    }

    #[test]
    fn gen_totp_uri_builds_expected_string() {
        let uri = gen_totp_uri(
            "app",
            "name",
            "asd",
            6,
            30,
        );

        assert_eq!(
            uri,
            "otpauth://totp/app:name?secret=asd&issuer=app&algorithm=SHA1&digits=6&period=30"
        );
    }
}

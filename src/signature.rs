use ring::hmac;
use url::Url;

lazy_static! {
    static ref BASE64_URL_SAFE: base64::Config =
        base64::Config::new(base64::CharacterSet::UrlSafe, true);
}

pub fn sign(url: &Url, secret_b64: &str) -> String {
    // TODO: propagate errors and avoid unwrapping

    // Implements https://developers.google.com/maps/documentation/maps-static/get-api-key#dig-sig-manual

    let key_bytes = base64::decode_config(secret_b64, *BASE64_URL_SAFE).unwrap();
    let key = hmac::Key::new(hmac::HMAC_SHA1_FOR_LEGACY_USE_ONLY, &key_bytes);

    let message = format!("{}?{}", url.path(), url.query().unwrap());
    let signature = hmac::sign(&key, message.as_bytes());

    let encoded_signature = base64::encode_config(signature.as_ref(), *BASE64_URL_SAFE);

    encoded_signature
}

#[cfg(test)]
mod tests {
    use super::*;
    use url::Url;

    #[test]
    fn it_signs_a_url() {
        let url = Url::parse(
            "https://maps.googleapis.com/maps/api/staticmap?\
        center=Brooklyn+Bridge%2CNew+York%2CNY\
        &zoom=13\
        &size=600x300\
        &maptype=roadmap\
        &markers=color%3Ablue%7Clabel%3AS%7C40.702147%2C-74.015794\
        &markers=color%3Agreen%7Clabel%3AG%7C40.711614%2C-74.012318\
        &markers=color%3Ared%7Clabel%3AC%7C40.718217%2C-73.998284\
        &key=YOUR_API_KEY",
        )
        .unwrap();

        let secret_b64 = "X8XXXxxxxxXwrIEQfguOVNGv2jY=";

        let signature = sign(&url, secret_b64);
        let expected_signature = String::from("AlLkp3bubbkQNFc-upyF2jbylSs=");

        assert_eq!(signature, expected_signature);
    }
}

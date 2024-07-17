use dharitri_wasm::types::{
    BoxedBytes, MoaOrDctTokenIdentifier, MoaOrDctTokenPayment, DctTokenPayment,
    TokenIdentifier,
};
use dharitri_wasm_debug::{
    check_managed_top_encode_decode, managed_moa_token_id, managed_token_id,
    managed_token_id_wrapped, DebugApi,
};

#[test]
fn test_moa() {
    let _ = DebugApi::dummy();
    assert!(MoaOrDctTokenIdentifier::<DebugApi>::moa().is_moa());
}

#[test]
fn test_codec() {
    let api = DebugApi::dummy();
    check_managed_top_encode_decode(
        api.clone(),
        MoaOrDctTokenIdentifier::<DebugApi>::moa(),
        MoaOrDctTokenIdentifier::<DebugApi>::MOA_REPRESENTATION,
    );

    let expected = BoxedBytes::from_concat(&[
        &[0, 0, 0, 4],
        &MoaOrDctTokenIdentifier::<DebugApi>::MOA_REPRESENTATION[..],
    ]);
    check_managed_top_encode_decode(
        api,
        vec![MoaOrDctTokenIdentifier::<DebugApi>::moa()],
        expected.as_slice(),
    );
}

#[test]
#[rustfmt::skip]
fn test_is_valid_dct_identifier() {
    let _ = DebugApi::dummy();

    // valid identifier
    assert!(TokenIdentifier::<DebugApi>::from("ALC-6258d2").is_valid_dct_identifier());

    // valid identifier with numbers in ticker
    assert!(TokenIdentifier::<DebugApi>::from("ALC123-6258d2").is_valid_dct_identifier());

    // valid ticker only numbers
    assert!(TokenIdentifier::<DebugApi>::from("12345-6258d2").is_valid_dct_identifier());

    // missing dash
    assert!(!TokenIdentifier::<DebugApi>::from("ALC6258d2").is_valid_dct_identifier());

    // wrong dash position
    assert!(!TokenIdentifier::<DebugApi>::from("AL-C6258d2").is_valid_dct_identifier());

    // lowercase ticker
    assert!(!TokenIdentifier::<DebugApi>::from("alc-6258d2").is_valid_dct_identifier());

    // uppercase random chars
    assert!(!TokenIdentifier::<DebugApi>::from("ALC-6258D2").is_valid_dct_identifier());

    // too many random chars
    assert!(!TokenIdentifier::<DebugApi>::from("ALC-6258d2ff").is_valid_dct_identifier());

    // ticker too short
    assert!(!TokenIdentifier::<DebugApi>::from("AL-6258d2").is_valid_dct_identifier());

    // ticker too long
    assert!(!TokenIdentifier::<DebugApi>::from("ALCCCCCCCCC-6258d2").is_valid_dct_identifier());
}

#[test]
fn test_is_valid_moa_or_dct() {
    let _ = DebugApi::dummy();

    // moa is always valid
    assert!(MoaOrDctTokenIdentifier::<DebugApi>::moa().is_valid());

    // valid dct
    assert!(
        MoaOrDctTokenIdentifier::<DebugApi>::dct(TokenIdentifier::from("ALC-6258d2")).is_valid()
    );

    // invalid dct, see above
    assert!(
        !MoaOrDctTokenIdentifier::<DebugApi>::dct(TokenIdentifier::from("ALCCCCCCCCC-6258d2"))
            .is_valid()
    );
}

#[test]
fn test_token_identifier_eq() {
    let _ = DebugApi::dummy();
    assert_eq!(
        TokenIdentifier::<DebugApi>::from("DCT-00000"),
        TokenIdentifier::<DebugApi>::from("DCT-00000")
    );
    assert_ne!(
        TokenIdentifier::<DebugApi>::from("DCT-00001"),
        TokenIdentifier::<DebugApi>::from("DCT-00002")
    );

    assert_eq!(
        MoaOrDctTokenIdentifier::<DebugApi>::dct(TokenIdentifier::from("DCT-00003")),
        TokenIdentifier::<DebugApi>::from("DCT-00003")
    );
    assert_ne!(
        MoaOrDctTokenIdentifier::<DebugApi>::moa(),
        TokenIdentifier::<DebugApi>::from("ANYTHING-1234")
    );
    assert_ne!(
        MoaOrDctTokenIdentifier::<DebugApi>::moa(),
        TokenIdentifier::<DebugApi>::from("MOA")
    );
}

#[test]
fn test_payment_eq() {
    let _ = DebugApi::dummy();
    assert_eq!(
        DctTokenPayment::<DebugApi>::new("PAY-00000".into(), 0, 1000u32.into()),
        DctTokenPayment::<DebugApi>::new("PAY-00000".into(), 0, 1000u32.into()),
    );
    assert_ne!(
        DctTokenPayment::<DebugApi>::new("PAY-00001".into(), 0, 1000u32.into()),
        DctTokenPayment::<DebugApi>::new("PAY-00002".into(), 0, 1000u32.into()),
    );
    assert_eq!(
        MoaOrDctTokenPayment::<DebugApi>::no_payment(),
        MoaOrDctTokenPayment::<DebugApi>::no_payment(),
    );
    assert_eq!(
        MoaOrDctTokenPayment::<DebugApi>::new(
            MoaOrDctTokenIdentifier::dct("DCTPAY-00000"),
            0,
            1000u32.into()
        ),
        MoaOrDctTokenPayment::<DebugApi>::new(
            MoaOrDctTokenIdentifier::dct("DCTPAY-00000"),
            0,
            1000u32.into()
        ),
    );
    assert_ne!(
        MoaOrDctTokenPayment::<DebugApi>::new(
            MoaOrDctTokenIdentifier::dct("DCTPAY-00001"),
            0,
            1000u32.into()
        ),
        MoaOrDctTokenPayment::<DebugApi>::new(
            MoaOrDctTokenIdentifier::dct("DCTPAY-00002"),
            0,
            1000u32.into()
        ),
    );
    assert_ne!(
        MoaOrDctTokenPayment::<DebugApi>::new(
            MoaOrDctTokenIdentifier::dct("DCTPAY-00001"),
            0,
            1000u32.into()
        ),
        MoaOrDctTokenPayment::<DebugApi>::no_payment(),
    );
}

#[test]
fn test_managed_token_id_macro() {
    let _ = DebugApi::dummy();
    assert_eq!(
        managed_moa_token_id!(),
        MoaOrDctTokenIdentifier::<DebugApi>::moa()
    );
    assert_eq!(
        managed_token_id!(b"ALC-6258d2"),
        TokenIdentifier::<DebugApi>::from("ALC-6258d2")
    );
    assert_eq!(
        managed_token_id_wrapped!(b"ALC-6258d2").unwrap_dct(),
        TokenIdentifier::<DebugApi>::from("ALC-6258d2")
    )
}

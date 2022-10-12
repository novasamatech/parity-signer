use image::{open, Pixel};
use opencv::{objdetect::QRCodeDetector, prelude::Mat};
use qr_reader_pc::process_qr_image;
use qr_reader_phone::process_payload::{InProgress, Ready};

#[test]
fn check_single_qr_hex() {
    let correct_result = String::from(
        "01d43593c715fdd31c61141abd0\
    4a99fd6822c8558854ccde39a5684e7a56da27d82750682cdb4208cd7c\
    13bf399b097dad0a8064c45e79a8bc50978f6a8a5db0775bcb4c335897\
    8ca625496e056f2e7ddf724cf0040e5ff106d06f54efbd95389",
    );

    let gray_img = open("./tests/test_qr_1.jpg").unwrap().into_luma8();

    let mut result = String::new();

    let mut qr_decoder = QRCodeDetector::default().unwrap();
    let mat = Mat::from_slice_2d(
        &gray_img
            .rows()
            .into_iter()
            .map(|row| {
                row.into_iter()
                    .map(|l| l.channels()[0])
                    .collect::<Vec<u8>>()
            })
            .collect::<Vec<_>>(),
    )
    .unwrap();
    let x = process_qr_image(&mut qr_decoder, &mat, InProgress::None).unwrap();
    match x {
        Ready::Yes(a) => result.push_str(&hex::encode(&a)),
        Ready::NotYet(_) => panic!("Waiting animated QR."),
    }

    assert_eq!(result, correct_result);
}

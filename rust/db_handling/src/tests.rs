use bip39::{Language, Mnemonic};
use pretty_assertions::assert_eq;

use constants::ALICE_SEED_PHRASE;

use crate::helpers::validate_mnemonic;
use crate::{
    identities::{check_derivation_set, generate_random_phrase, is_passworded},
    interface_signer::{guess, SeedDraft},
};

#[test]
fn word_search_1() {
    let word_part = "dri";
    let out = guess(word_part);
    let out_expected = vec![
        "drift".to_string(),
        "drill".to_string(),
        "drink".to_string(),
        "drip".to_string(),
        "drive".to_string(),
    ];
    assert_eq!(out, out_expected);
}

#[test]
fn word_search_2() {
    let word_part = "umbra";
    let out = guess(word_part);
    assert!(out.is_empty(), "Found different word set:\n{out:?}");
}

#[test]
fn word_search_3() {
    let word_part = "котик";
    let out = guess(word_part);
    assert!(out.is_empty(), "Found different word set:\n{out:?}");
}

#[test]
fn word_search_4() {
    let word_part = "";
    let out = guess(word_part);
    let out_expected = vec![
        "abandon".to_string(),
        "ability".to_string(),
        "able".to_string(),
        "about".to_string(),
        "above".to_string(),
        "absent".to_string(),
        "absorb".to_string(),
        "abstract".to_string(),
    ];
    assert_eq!(out, out_expected);
}

#[test]
fn word_search_5() {
    let word_part = " ";
    let out = guess(word_part);
    assert!(out.is_empty(), "Found different word set:\n{out:?}");
}

#[test]
fn word_search_6() {
    let word_part = "s";
    let out = guess(word_part);
    let out_expected = vec![
        "sad".to_string(),
        "saddle".to_string(),
        "sadness".to_string(),
        "safe".to_string(),
        "sail".to_string(),
        "salad".to_string(),
        "salmon".to_string(),
        "salon".to_string(),
    ];
    assert_eq!(out, out_expected);
}

#[test]
fn word_search_7() {
    let word_part = "se";
    let out = guess(word_part);
    let out_expected = vec![
        "sea".to_string(),
        "search".to_string(),
        "season".to_string(),
        "seat".to_string(),
        "second".to_string(),
        "secret".to_string(),
        "section".to_string(),
        "security".to_string(),
    ];
    assert_eq!(out, out_expected);
}

#[test]
fn word_search_8() {
    let word_part = "sen";
    let out = guess(word_part);
    let out_expected = vec![
        "senior".to_string(),
        "sense".to_string(),
        "sentence".to_string(),
    ];
    assert_eq!(out, out_expected);
}

#[test]
fn alice_recalls_seed_phrase_1() {
    let mut seed_draft = SeedDraft::initiate();
    seed_draft.added("bottom", None);
    seed_draft.added("lake", None);
    // oops, wrong place
    seed_draft.added("drive", Some(1));
    seed_draft.added("obey", Some(2));
    let print = seed_draft.draft();
    let expected_print = vec!["bottom", "drive", "obey", "lake"];
    assert_eq!(print, expected_print);
    // adding invalid word - should be blocked through UI, expect no changes
    seed_draft.added("занавеска", None);
    let print = seed_draft.draft();
    let expected_print = vec!["bottom", "drive", "obey", "lake"];
    assert_eq!(print, expected_print);
    // removing invalid word - should be blocked through UI, expect no changes
    seed_draft.remove(5);
    let print = seed_draft.draft();
    let expected_print = vec!["bottom", "drive", "obey", "lake"];
    assert_eq!(print, expected_print);
    // removing word
    seed_draft.remove(1);
    let print = seed_draft.draft();
    let expected_print = vec!["bottom", "obey", "lake"];
    assert_eq!(print, expected_print);
}

#[test]
fn alice_recalls_seed_phrase_2() {
    let mut seed_draft = SeedDraft::initiate();
    seed_draft.added("fit", None);
    let print = seed_draft.draft();
    let expected_print = vec!["fit"];
    assert_eq!(print, expected_print);
}

#[test]
fn alice_recalls_seed_phrase_3() {
    let mut seed_draft = SeedDraft::initiate();
    seed_draft.added("obe", None);
    let print = seed_draft.draft();
    let expected_print = vec!["obey"];
    assert_eq!(print, expected_print);
}

#[test]
fn test_generate_random_seed_phrase() {
    let random_phrase = generate_random_phrase(24).unwrap();
    assert!(Mnemonic::validate(&random_phrase, Language::English).is_ok());
    assert!(generate_random_phrase(1).is_err());
    let random_phrase2 = generate_random_phrase(24).unwrap();
    assert!(Mnemonic::validate(&random_phrase2, Language::English).is_ok());
    assert!(random_phrase2 != random_phrase);
}

#[test]
fn test_check_for_seed_validity() {
    assert!(Mnemonic::validate(ALICE_SEED_PHRASE, Language::English).is_ok());
    assert!(Mnemonic::validate("the fox is triangular", Language::English).is_err());
    assert!(Mnemonic::validate("", Language::English).is_err());
    assert!(Mnemonic::validate("низ ехать подчиняться озеро занавеска дым корзина держать гонка одинокий подходящий прогулка", Language::English).is_err());
}

#[test]
fn must_check_for_valid_derivation_phrase() {
    assert!(!is_passworded("").expect("valid empty path"));
    assert!(is_passworded("//").is_err());
    assert!(!is_passworded("//path1").expect("valid path1"));
    assert!(!is_passworded("//path/path").expect("soft derivation"));
    assert!(!is_passworded("//path//path").expect("hard derivation"));
    assert!(is_passworded("//path///password").expect("path with password"));
    assert!(is_passworded("///").is_err());
    assert!(!is_passworded("//$~").expect("weird symbols"));
    assert!(is_passworded("abraca dabre").is_err());
    assert!(is_passworded("////").expect("//// - password is /"));
    assert!(
        is_passworded("//path///password///password").expect("password///password is a password")
    );
    assert!(!is_passworded("//путь").expect("valid utf8 abomination"));
}

#[test]
fn checking_derivation_set() {
    assert!(check_derivation_set(&[
        "/0".to_string(),
        "//Alice/westend".to_string(),
        "//secret//westend".to_string()
    ])
    .is_ok());
    assert!(check_derivation_set(&[
        "/0".to_string(),
        "/0".to_string(),
        "//Alice/westend".to_string(),
        "//secret//westend".to_string()
    ])
    .is_ok());
    assert!(check_derivation_set(&["//remarkably///ugly".to_string()]).is_ok());
    assert!(check_derivation_set(&["no_path_at_all".to_string()]).is_err());
    assert!(check_derivation_set(&["///".to_string()]).is_err());
}

#[test]
fn validate_mnemonic_ok() {
    let seed = "park remain person kitchen mule spell knee armed position rail grid ankle";
    assert!(validate_mnemonic(seed));
}

#[test]
fn validate_mnemonic_err() {
    let invalid_seed = "park remain person kitchen mule spell knee armed position rail grid";
    assert!(!validate_mnemonic(invalid_seed));
}
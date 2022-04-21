//
//  TransactionTypes.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 11.8.2021.
//

/**
 * This is hard-typed decoding of transaction passed from Rust
 *
 * Used in showing transaction content both in signing and log
 */

import Foundation

/**
 * Cards for transaction content
 * all cards must match Rust code!
 */
enum Card {
    case author(Author)
    case authorPlain(AuthorPlain)
    case authorPublicKey(AuthorPublicKey)
    case balance(Currency)
    case bitVec(String)
    case blockHash(String)
    case call(Call)
    case defaultCard(String)
    case derivations([String])
    case enumVariantName(EnumVariantName)
    case eraImmortal
    case eraMortal(EraMortal)
    case error(String)
    case fieldName(FieldName)
    case fieldNumber(FieldNumber)
    case id(Id)
    case identityField(String)
    case meta(MetaSpecs)
    case nameVersion(NameVersion)
    case networkGenesisHash(String)
    case networkName(String)
    case networkInfo(NetworkInfo)
    case newSpecs(NewSpecs)
    case nonce(String)
    case none
    case pallet(String)
    case text(String)
    case tip(Currency)
    case tipPlain(String)
    case txSpec(String)
    case txSpecPlain(TxSpecPlain)
    case typesInfo(TypesInfo)
    case varName(String)
    case verifier(Verifier)
    case warning(String)
}

/**
 * Visualization of transaction author
 */
struct Author: Decodable {
    var base58: String
    var seed: String
    var derivation_path: String
    var has_password: Bool?
    var identicon: String
    
    func intoAddress() -> Address {
        return Address(
            base58: self.base58,
            path: self.derivation_path,
            has_pwd: self.has_password == true,
            identicon: self.identicon,
            seed_name: seed,
            multiselect: false
        )
    }
}

/**
 * Visualization of address without encryption
 */
struct AuthorPlain: Decodable {
    var base58: String
    var identicon: String
}

/**
 * Visualization of unknown address
 */
struct AuthorPublicKey: Decodable {
    var public_key: String
    var encryption: String
    var identicon: String
}

/**
 * Call name with docs
 */
struct Call: Decodable {
    var method_name: String
    var docs: String
}

/**
 * Balance parsed with units
 */
struct Currency: Decodable {
    var amount: String
    var units: String
}

/**
 * EnumVariantName
 */
struct EnumVariantName: Decodable {
    var name: String
    var docs_enum_variant: String
}

/**
 * Visualization of finite lifetime
 */
struct EraMortal: Decodable {
    var era: String
    var phase: String
    var period: String
}

/**
 * FieldName visualization with docs
 */
struct FieldName: Decodable {
    var name: String
    var docs_field_name: String
    var path_type: String
    var docs_type: String
}

/**
 * FiledNumber visualization with docs
 */
struct FieldNumber: Decodable {
    var number: String
    var docs_field_number: String
    var path_type: String
    var docs_type: String
}

/**
 * Visualization of an address
 */
struct Id: Decodable {
    var base58: String
    var identicon: String
}

/**
 * Visualization of metadata to add
 */
struct MetaSpecs: Decodable, Hashable {
    var specname: String
    var spec_version: String
    var meta_hash: String
    var meta_id_pic: String
}

/**
 * Network name and version - this identifies metadata
 */
struct NameVersion: Decodable, Hashable {
    var name: String
    var version: String
}

struct NetworkInfo: Decodable, Hashable {
    var network_title: String
    var network_logo: String
}

/**
 * Description of network specs that are added
 */
struct NewSpecs: Decodable, Hashable {
    var base58prefix: String
    var color: String
    var decimals: String
    var encryption: String
    var genesis_hash: String
    var logo: String
    var name: String
    var path_id: String
    var secondary_color: String
    var title: String
    var unit: String
}

/**
 * Unit-formatted amount of tip
 */
struct Tip: Decodable {
    var amount: String
    var units: String
}

/**
 * Thansaction information if network was not found
 */
struct TxSpecPlain: Decodable {
    var network_genesis_hash: String
    var version: String
    var tx_version: String
}

struct TypesInfo: Decodable {
    var types_hash: String
    var types_id_pic: String
}

/*
/**
 * Visualization of cerificate issuer info
 */
struct Verifier: Decodable, Hashable {
    var public_key: String
    var identicon: String
    var encryption: String
}
*/
 




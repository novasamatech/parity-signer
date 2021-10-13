//
//  TransactionTypes.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 11.8.2021.
//

import Foundation

//all cards must match Rust code!
enum Card {
    case author(Author)
    case authorPlain(AuthorPlain)
    case authorPublicKey(AuthorPublicKey)
    case balance(Currency)
    case bitVec(String)
    case blockHash(String)
    case call(Call)
    case defaultCard(String)
    case enumVariantName(EnumVariantName)
    case eraImmortalNonce(EraImmortalNonce)
    case eraMortalNonce(EraMortalNonce)
    case error(String)
    case fieldName(FieldName)
    case fieldNumber(FieldNumber)
    case id(String)
    case identityField(String)
    case meta(MetaSpecs)
    case newNetwork(NewNetwork)
    case none
    case pallet(String)
    case pathDocs(PathDocs)
    case range(TxRange)
    case tip(Currency)
    case tipPlain(String)
    case txSpec(TxSpec)
    case txSpecPlain(TxSpecPlain)
    case typesInfo(String)
    case varName(String)
    case verifier(Verifier)
    case warning(String)
}

struct Author: Decodable {
    var base58: String
    var seed: String
    var derivation_path: String
    var has_password: Bool
    var name: String
}

struct AuthorPlain: Decodable {
    var base58: String
}

struct AuthorPublicKey: Decodable {
    var hex: String
    var crypto: String
}

struct Call: Decodable {
    var pallet: String
    var method: String
    var docs: String
}

struct Currency: Decodable {
    var amount: String
    var units: String
}

struct EnumVariantName: Decodable {
    var name: String
    var docs: String
}

//TODO: manual decoders for these two
struct EraImmortalNonce: Decodable {
    var era: String
    var nonce: String
}

struct EraMortalNonce: Decodable {
    var era: String
    var phase: String
    var period: String
    var nonce: String
}

struct FieldName: Decodable {
    var name: String
    var docs: String
}

struct FieldNumber: Decodable {
    var number: String
    var docs: String
}

struct MetaSpecs: Decodable, Hashable {
    var specname: String
    var spec_version: String
    var meta_hash: String
}

struct NewNetwork: Decodable, Hashable {
    var specname: String
    var spec_version: String
    var meta_hash: String
    var base58prefix: String
    var color: String
    var decimals: String
    var genesis_hash: String
    var logo: String
    var name: String
    var path_id: String
    var secondary_colod: String
    var title: String
    var unit: String
    var verifier: Verifier
}

struct PathDocs: Decodable {
    var path: [String]
    var docs: String
}

struct TxRange: Decodable {
    var start: String
    var end: String
    var inclusive: String
}

struct Tip: Decodable {
    var amount: String
    var units: String
}

struct TxSpec: Decodable {
    var network: String
    var version: String
    var tx_version: String
}

struct TxSpecPlain: Decodable {
    var network_genesis_hash: String
    var version: String
    var tx_version: String
}

struct Verifier: Decodable, Hashable {
    var hex: String
    var encryption: String
}

struct TransactionCard: Decodable {
    var index: Int
    var indent: Int
    var card: Card
    
    enum CodingKeys: String, CodingKey {
        case index
        case indent
        case type
        case payload
    }
    
    init(from decoder: Decoder) throws {
        var content: String = ""
        let values = try decoder.container(keyedBy: CodingKeys.self)
        index = try values.decode(Int.self, forKey: .index)
        indent = try values.decode(Int.self, forKey: .indent)
        let type = try values.decode(String.self, forKey: .type)
        
        //first handle special cases of complex payloads
        switch type {
        case "author":
            card = .author(try values.decode(Author.self, forKey: .payload))
            return
        case "author_plain":
            card = .authorPlain(try values.decode(AuthorPlain.self, forKey: .payload))
            return
        case "author_public_key":
            card = .authorPublicKey(try values.decode(AuthorPublicKey.self, forKey: .payload))
            return
        case "balance":
            card = .balance(try values.decode(Currency.self, forKey: .payload))
            return
        case "call":
            card = .call(try values.decode(Call.self, forKey: .payload))
            return
        case "enum_variant_name":
            card = .enumVariantName(try values.decode(EnumVariantName.self, forKey: .payload))
            return
        case "era_mortal_nonce":
            card = .eraMortalNonce(try values.decode(EraMortalNonce.self, forKey: .payload))
            return
        case "era_immortal_nonce":
            card = .eraImmortalNonce(try values.decode(EraImmortalNonce.self, forKey: .payload))
            return
        case "field_name":
            card = .fieldName(try values.decode(FieldName.self, forKey: .payload))
            return
        case "field_number":
            card = .fieldNumber(try values.decode(FieldNumber.self, forKey: .payload))
            return
        case "meta":
            card = .meta(try values.decode(MetaSpecs.self, forKey: .payload))
            return
        case "new_network":
            card = .newNetwork(try values.decode(NewNetwork.self, forKey: .payload))
            return
        case "none":
            card = .none
        case "path_and_docs":
            card = .pathDocs(try values.decode(PathDocs.self, forKey: .payload))
            return
        case "range":
            card = .range(try values.decode(TxRange.self, forKey: .payload))
            return
        case "tip":
            card = .tip(try values.decode(Currency.self, forKey: .payload))
            return
        case "tx_spec":
            card = .txSpec(try values.decode(TxSpec.self, forKey: .payload))
            return
        case "tx_spec_plain":
            card = .txSpecPlain(try values.decode(TxSpecPlain.self, forKey: .payload))
            return
        case "verifier":
            card = .verifier(try values.decode(Verifier.self, forKey: .payload))
            return
        default:
            content = try values.decode(String.self, forKey: .payload)
        }
        
        //simple 1-string payloads
        switch type {
        case "bitvec":
            card = .bitVec(content)
        case "block_hash":
            card = .blockHash(content)
        case "default":
            card = .defaultCard(content)
        case "error":
            card = .error(content)
        case "Id":
            card = .id(content)
        case "identity_field":
            card = .identityField(content)
        case "pallet":
            card = .pallet(content)
        case "tip_plain":
            card = .tipPlain(content)
        case "types_hash":
            card = .typesInfo(content)
        case "varname":
            card = .varName(content)
        case "warning":
            card = .warning(content)
        default:
            card = .error("Transaction parsing error!")
        }
    }
}

struct ActionPayload: Decodable, Encodable {
    var type: String
    var checksum: String
}

struct Action: Decodable, Encodable {
    var type: String
    var payload: ActionPayload
}

struct TransactionCardSet: Decodable {
    var author: [TransactionCard]?
    var error: [TransactionCard]?
    var extrinsics: [TransactionCard]?
    var method: [TransactionCard]?
    var warning: [TransactionCard]?
    var types_info: [TransactionCard]?
    var action: Action?
}

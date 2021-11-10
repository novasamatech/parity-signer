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
    case eraImmortal
    case eraMortal(EraMortal)
    case error(String)
    case fieldName(FieldName)
    case fieldNumber(FieldNumber)
    case id(String)
    case identityField(String)
    case meta(MetaSpecs)
    case nameVersion(NameVersion)
    case networkGenesisHash(String)
    case networkName(String)
    case newSpecs(NewSpecs)
    case nonce(String)
    case none
    case pallet(String)
    case text(String)
    case tip(Currency)
    case tipPlain(String)
    case txSpec(String)
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
    var method_name: String
    var docs: String
}

struct Currency: Decodable {
    var amount: String
    var units: String
}

struct EnumVariantName: Decodable {
    var name: String
    var docs_enum_variant: String
}

struct EraMortal: Decodable {
    var era: String
    var phase: String
    var period: String
}

struct FieldName: Decodable {
    var name: String
    var docs_field_name: String
    var path_type: String
    var docs_type: String
}

struct FieldNumber: Decodable {
    var number: String
    var docs_field_number: String
    var path_type: String
    var docs_type: String
}

struct MetaSpecs: Decodable, Hashable {
    var specname: String
    var spec_version: String
    var meta_hash: String
}

struct NameVersion: Decodable, Hashable {
    var name: String
    var version: String
}

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

struct Tip: Decodable {
    var amount: String
    var units: String
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
        case "method":
            card = .call(try values.decode(Call.self, forKey: .payload))
            return
        case "enum_variant_name":
            card = .enumVariantName(try values.decode(EnumVariantName.self, forKey: .payload))
            return
        case "era":
            do {card = .eraMortal(try values.decode(EraMortal.self, forKey: .payload))}
            catch {
                card = .eraImmortal
                return
            }
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
        case "name_version":
            card = .nameVersion(try values.decode(NameVersion.self, forKey: .payload))
            return
        case "new_specs":
            card = .newSpecs(try values.decode(NewSpecs.self, forKey: .payload))
            return
        case "none":
            card = .none
            return
        case "tip":
            card = .tip(try values.decode(Currency.self, forKey: .payload))
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
        case "network_genesis_hash":
            card = .networkGenesisHash(content)
        case "network_name":
            card = .networkName(content)
        case "nonce":
            card = .nonce(content)
        case "pallet":
            card = .pallet(content)
        case "text":
            card = .text(content)
        case "tip_plain":
            card = .tipPlain(content)
        case "tx_version":
            card = .txSpec(content)
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

struct Action: Decodable, Encodable {
    var type: String
    var payload: String
}

struct TransactionCardSet: Decodable {
    var author: [TransactionCard]?
    var error: [TransactionCard]?
    var extensions: [TransactionCard]?
    var message: [TransactionCard]?
    var method: [TransactionCard]?
    var new_specs: [TransactionCard]?
    var verifier: [TransactionCard]?
    var warning: [TransactionCard]?
    var types_info: [TransactionCard]?
    var action: Action?
}

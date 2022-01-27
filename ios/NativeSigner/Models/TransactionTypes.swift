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

/**
 * Visualization of cerificate issuer info
 */
struct Verifier: Decodable, Hashable {
    var public_key: String
    var identicon: String
    var encryption: String
}

/**
 * Complex decoder for transaction cards
 * card format: {index, indent, type, payload}
 *
 * where index is used for cards sorting, indent is an offset for rendering on screen,
 * and payload could be any complex thing needed to render a transaction card
 */
struct TransactionCard: Decodable, Hashable {
    static func == (lhs: TransactionCard, rhs: TransactionCard) -> Bool {
        return lhs.index == rhs.index //guaranteed in backend
    }
    
    func hash(into hasher: inout Hasher) {
        hasher.combine(index)
    }
    
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
        case "derivations":
            card = .derivations(try values.decode([String].self, forKey: .payload))
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
        case "Id":
            card = .id(try values.decode(Id.self, forKey: .payload))
            return
        case "meta":
            card = .meta(try values.decode(MetaSpecs.self, forKey: .payload))
            return
        case "name_version":
            card = .nameVersion(try values.decode(NameVersion.self, forKey: .payload))
            return
        case "network_info":
            card = .networkInfo(try values.decode(NetworkInfo.self, forKey: .payload))
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
        case "types":
            card = .typesInfo(try values.decode(TypesInfo.self, forKey: .payload))
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
        //case "types_hash":
           // card = .typesInfo(content)
        case "varname":
            card = .varName(content)
        case "warning":
            card = .warning(content)
        default:
            card = .error("Transaction parsing error!")
        }
    }
}

/**
 * The JSON object actually passed from Rust is this
 */
struct TransactionCardSet: Decodable, Hashable {
    var author: [TransactionCard]?
    var error: [TransactionCard]?
    var extensions: [TransactionCard]?
    var importing_derivations: [TransactionCard]?
    var message: [TransactionCard]?
    var meta: [TransactionCard]?
    var method: [TransactionCard]?
    var new_specs: [TransactionCard]?
    var verifier: [TransactionCard]?
    var warning: [TransactionCard]?
    var types_info: [TransactionCard]?
    
    /**
     * Prepares transaction cards to be shown in a frame
     */
    func assemble() -> [TransactionCard] {
        var assembled: [TransactionCard] = []
        assembled.append(contentsOf: self.author ?? [])
        assembled.append(contentsOf: self.error ?? [])
        assembled.append(contentsOf: self.extensions ?? [])
        assembled.append(contentsOf: self.importing_derivations ?? [])
        assembled.append(contentsOf: self.message ?? [])
        assembled.append(contentsOf: self.meta ?? [])
        assembled.append(contentsOf: self.method ?? [])
        assembled.append(contentsOf: self.new_specs ?? [])
        assembled.append(contentsOf: self.verifier ?? [])
        assembled.append(contentsOf: self.warning ?? [])
        assembled.append(contentsOf: self.types_info ?? [])
        return assembled.sorted(by: {
            $0.index < $1.index
        })
    }
}

//
//  MTransaction.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 15.12.2021.
//

import Foundation

struct MTransaction: Decodable {
    var content: TransactionCardSet
    var type: TransactionType
    var author_info: TransactionAuthor?
    var network_info: TransactionNetworkInfo?
}

enum TransactionType: String, Decodable {
    case sign
    case stub
    case read
    case import_derivations
    case done
}

struct TransactionAuthor: Decodable, Hashable {
    var base58: String
    var identicon: String
    var seed: String
    var derivation_path: String
    
    func intoAddress() -> Address {
        return Address(
            base58: self.base58,
            path: self.derivation_path,
            has_pwd: false, //TODO: this
            identicon: self.identicon,
            seed_name: self.seed,
            multiselect: false
        )
    }
}

struct TransactionNetworkInfo: Decodable, Hashable {
    var network_title: String
    var network_logo: String
}

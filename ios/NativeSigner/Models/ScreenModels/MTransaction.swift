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
}

struct TransactionAuthor: Decodable {
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
            seed_name: self.seed
        )
    }
}

struct TransactionNetworkInfo: Decodable {
    var network_title: String
    var network_logo: String
}

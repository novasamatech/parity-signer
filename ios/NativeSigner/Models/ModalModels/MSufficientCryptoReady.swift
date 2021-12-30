//
//  MSufficientCryptoReady.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 21.12.2021.
//

import Foundation

struct MSufficientCryptoReady: Decodable {
    var author_info: MSCAuthor
    var signature: String
    var content: MSCContent
}

struct MSCAuthor: Decodable {
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

struct MSCContent: Decodable {
    var type: String
}

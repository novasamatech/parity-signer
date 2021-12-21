//
//  MSignSufficientCrypto.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 21.12.2021.
//

import Foundation

struct MSignSufficientCrypto: Decodable {
    var identities: [MRawKey]
    
    func getSortedKeys() -> [MRawKey] {
        return self.identities.sorted(by: {
            if $0.seed_name == $1.seed_name {
                return $0.path < $1.path
            } else {
                return $0.seed_name < $1.seed_name
            }
        })
    }
}

struct MRawKey: Decodable {
    var seed_name: String
    var address_key: String
    var public_key: String
    var identicon: String
    var has_pwd: Bool
    var path: String
    
    func intoAddress() -> Address {
        return Address(
            base58: self.public_key,
            path: self.path,
            has_pwd: self.has_pwd,
            identicon: self.identicon,
            seed_name: self.seed_name
        )
    }
}

//
//  MSettings.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 21.1.2022.
//

import Foundation

struct MSettings: Decodable {
    var public_key: String?
    var identicon: String?
    var encryption: String?
    var error: String?
    
    func intoVerifier() -> MVerifierDetails? {
        if (self.public_key != nil && self.identicon != nil && self.encryption != nil) {
            return MVerifierDetails(public_key: self.public_key!, identicon: self.identicon!, encryption: self.encryption!)
        } else {return nil}
    }
}

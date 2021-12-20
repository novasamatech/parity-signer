//
//  MManageMetadata.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 20.12.2021.
//

import Foundation

struct MManageMetadata: Decodable {
    var name: String
    var version: String
    var meta_hash: String
    var meta_id_pic: String
    var networks: [MMMNetwork]
    
    func forCard() -> MMetadataRecord {
        return MMetadataRecord(spec_version: self.version, meta_hash: self.meta_hash, meta_id_pic: self.meta_id_pic)
    }
}

struct MMMNetwork: Decodable {
    var title: String
    var logo: String
    var order: Int
    var current_on_screen: Bool
}

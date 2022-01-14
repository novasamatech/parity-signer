//
//  MPasswordConfirm.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 13.12.2021.
//

import Foundation

struct MPasswordConfirm: Decodable, Equatable {
    var pwd: String
    var seed_name: String
    var cropped_path: String
}

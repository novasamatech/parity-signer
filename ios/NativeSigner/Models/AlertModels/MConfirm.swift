//
//  MConfirm.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 17.12.2021.
//

import Foundation

struct MConfirm: Decodable {
    var header: String
    var subheader: String
    var yes: String = "Confirm"
    var no: String = "Cancel"
}

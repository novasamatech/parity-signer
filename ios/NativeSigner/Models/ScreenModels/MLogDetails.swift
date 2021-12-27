//
//  MLogDetails.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 27.12.2021.
//

import Foundation

struct MLogDetails: Decodable {
    var timestamp: String
    var events: [Event]
}

//
//  MLog.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 2.12.2021.
//

import Foundation

struct MLog: Decodable {
    var log: [History] = []
    var total_entries: Int = 0 //This is prepared for lazy history loading
}

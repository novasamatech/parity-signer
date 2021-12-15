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
}

enum TransactionType: String, Decodable {
    case sign
    case stub
    case read
}

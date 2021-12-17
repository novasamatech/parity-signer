//
//  MEnterPassword.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 15.12.2021.
//

import Foundation

struct MEnterPassword: Decodable {
    var author_info: TransactionAuthor
    var counter: Int
}

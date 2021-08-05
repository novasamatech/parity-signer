//
//  Transaction.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 5.8.2021.
//

import Foundation

enum TransactionState {
    case scanning
    case parsing
    case preview
    case show
}

class Transaction: ObservableObject {
    @Published var state: TransactionState = .scanning
    
    init() {}
}

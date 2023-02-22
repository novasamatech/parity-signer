//
//  Utils.swift
//  Polkadot Vault
//
//  Created by Alexander Slesarev on 9.8.2021.
//

import Foundation

extension Verifier {
    func show() -> String {
        switch v {
        case let .standard(value):
            return value[0]
        case .none:
            return "None"
        }
    }
}

extension VerifierValue {
    func show() -> String {
        switch self {
        case let .standard(value):
            return value[0]
        }
    }
}

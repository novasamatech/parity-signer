//
//  Verifier+Show.swift
//  PolkadotVault
//
//  Created by Krzysztof Rodak on 14/03/2023.
//

import Foundation
extension Verifier {
    func show() -> String {
        switch v {
        case let .standard(value):
            value[0]
        case .none:
            "None"
        }
    }
}

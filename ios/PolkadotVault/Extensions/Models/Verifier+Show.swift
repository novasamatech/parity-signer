//
//  Verifier+show.swift
//  PolkadotVault
//
//  Created by Krzysztof Rodak on 14/03/2023.
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

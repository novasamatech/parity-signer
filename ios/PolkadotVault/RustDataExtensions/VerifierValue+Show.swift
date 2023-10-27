//
//  VerifierValue+Show.swift
//  PolkadotVault
//
//  Created by Krzysztof Rodak on 14/03/2023.
//

import Foundation

extension VerifierValue {
    func show() -> String {
        switch self {
        case let .standard(value):
            value[0]
        }
    }
}

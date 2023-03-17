//
//  SignerImage+Helpers.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 08/12/2022.
//

import Foundation

extension SignerImage {
    /// Convienience accessor for `svg` payload for `SignerImage`
    /// Returns `[]` for `.png` value
    var svgPayload: [UInt8] {
        switch self {
        case let .svg(payload):
            return payload
        case .png:
            return []
        }
    }

    /// Convienience accessor for `png` payload for `SignerImage`
    /// Returns `[]` for `.svg` value
    var pngPayload: [UInt8] {
        switch self {
        case .svg:
            return []
        case let .png(payload):
            return payload
        }
    }
}

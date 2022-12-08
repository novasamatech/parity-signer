//
//  SignerImage+Helpers.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 08/12/2022.
//

import Foundation

extension SignerImage {
    var svgPayload: [UInt8] {
        switch self {
        case let .svg(payload):
            return payload
        case .png:
            return []
        }
    }
}

//
//  DerivationPathComponents.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 13/01/2023.
//

import Foundation

enum DerivationPathComponent: String, CustomStringConvertible, CaseIterable {
    case soft = "/"
    case hard = "//"
    case passworded = "///"

    var description: String { rawValue }
}

extension String {
    var formattedAsPasswordedPath: String {
        let components = components(separatedBy: DerivationPathComponent.passworded.description)
        guard components.count > 1 else { return self }
        return components[0] + DerivationPathComponent.passworded
            .description + String(repeating: "•", count: components[1].count)
    }
}

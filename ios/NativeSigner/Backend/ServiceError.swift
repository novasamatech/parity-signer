//
//  ServiceError.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 20/01/2023.
//

import Foundation

struct ServiceError: Error, CustomStringConvertible {
    let message: String

    var description: String {
        [Localizable.Error.Service.Label.prefix.string, message, Localizable.Error.Service.Label.suffix.string]
            .joined(separator: "\n")
    }
}

//
//  QrData+Helpers.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 08/12/2022.
//

import Foundation

extension QrData {
    enum DataType {
        case regular
        case sensitive
    }

    var payload: [UInt8] {
        switch self {
        case let .regular(data):
            data
        case let .sensitive(data):
            data
        }
    }

    var type: DataType {
        switch self {
        case .regular:
            .regular
        case .sensitive:
            .sensitive
        }
    }
}

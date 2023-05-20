//
//  QrData+Stubs.swift
//  PolkadotVault
//
//  Created by Krzysztof Rodak on 10/05/2023.
//

import Foundation

extension QrData {
    static let stubRegular: QrData = .regular(data: Stubs.stubQRCode)
    static let stubSensitive: QrData = .sensitive(data: Stubs.stubQRCode)
}

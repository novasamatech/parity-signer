//
//  ProcessInfoProtocol.swift
//  PolkadotVault
//
//  Created by Krzysztof Rodak on 16/01/2024.
//

import Foundation

// sourcery: AutoMockable
protocol ProcessInfoProtocol: AnyObject {
    var environment: [String: String] { get }
}

extension ProcessInfo: ProcessInfoProtocol {}

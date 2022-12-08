//
//  ProcessInfoProtocol.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 02/08/2022.
//

import Foundation

/// A collection of information about the current app process.
protocol ProcessInfoProtocol {
    /// The variable names (keys) and their values in the environment from which the process was launched.
    var environment: [String: String] { get }
}

extension ProcessInfo: ProcessInfoProtocol {}

//
//  RuntimePropertiesProvider.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 02/08/2022.
//
import Foundation


/// Protocol that provides access to app process properties
protocol RuntimePropertiesProviding: AnyObject {
    /// Indicates whether application is launched in development mode with stubed data
    var isInDevelopmentMode: Bool { get }
}

/// Wrapper for accessing `RuntimeProperties` and other application runtime values
final class RuntimePropertiesProvider: RuntimePropertiesProviding {
    enum Properties: String, CustomStringConvertible {
        case developmentMode

        var description: String { return rawValue }
    }

    enum PropertiesValues: String, CustomStringConvertible {
        case `true`
        case `false`

        var description: String { return rawValue }
    }

    private let processInfo: ProcessInfoProtocol

    init(processInfo: ProcessInfoProtocol = ProcessInfo.processInfo) {
        self.processInfo = processInfo
    }

    var isInDevelopmentMode: Bool {
        processInfo.environment[Properties.developmentMode.description] == PropertiesValues.true.description
    }
}

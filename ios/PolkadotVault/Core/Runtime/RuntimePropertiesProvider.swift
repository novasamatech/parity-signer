//
//  RuntimePropertiesProvider.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 02/08/2022.
//
import Foundation

enum ApplicationRuntimeMode: String, Equatable {
    case production
    case qa
    case debug
}

/// Protocol that provides access to app process properties
protocol RuntimePropertiesProviding: AnyObject {
    var runtimeMode: ApplicationRuntimeMode { get }
    var dynamicDerivationsEnabled: Bool { get }
}

/// Wrapper for accessing `RuntimeProperties` and other application runtime values
final class RuntimePropertiesProvider: RuntimePropertiesProviding {
    private enum PropertiesValues: String, CustomStringConvertible {
        case `true`
        case `false`

        var description: String { rawValue }
    }

    private let appInformationContainer: ApplicationInformationContaining.Type
    private let processInfo: ProcessInfoProtocol

    init(
        appInformationContainer: ApplicationInformationContaining.Type = ApplicationInformation.self,
        processInfo: ProcessInfoProtocol = ProcessInfo.processInfo
    ) {
        self.appInformationContainer = appInformationContainer
        self.processInfo = processInfo
    }

    var runtimeMode: ApplicationRuntimeMode {
        ApplicationRuntimeMode(rawValue: appInformationContainer.appRuntimeMode) ?? .production
    }

    var dynamicDerivationsEnabled: Bool {
        appInformationContainer.dynamicDerivationsEnabled == PropertiesValues.true.rawValue
    }
}

extension ApplicationInformation: ApplicationInformationContaining {}

protocol ApplicationInformationContaining {
    static var dynamicDerivationsEnabled: String { get }
    static var appRuntimeMode: String { get }
}

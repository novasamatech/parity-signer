//
//  ErrorBottomModalViewModel+TransactionErrors.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 09/02/2023.
//

import Foundation

extension ErrorBottomModalViewModel {
    static func metadataForUnknownNetwork(
        _ networkName: String,
        _ action: @escaping @autoclosure () -> Void = {}()
    ) -> ErrorBottomModalViewModel {
        ErrorBottomModalViewModel(
            title: "Metadata for unknown network: \(networkName)",
            content: "",
            steps: [],
            secondaryAction: .init(label: Localizable.TransactionSign.Action.error.key, action: action)
        )
    }

    static func networkAlreadyAdded(
        _ networkName: String,
        _ encryption: String,
        _ action: @escaping @autoclosure () -> Void = {}()
    ) -> ErrorBottomModalViewModel {
        ErrorBottomModalViewModel(
            title: "Network Already Added: \(networkName) \(encryption)",
            content: "",
            steps: [],
            secondaryAction: .init(label: Localizable.TransactionSign.Action.error.key, action: action)
        )
    }

    static func metadataAlreadyAdded(
        _ networkName: String,
        _ version: String,
        _ action: @escaping @autoclosure () -> Void = {}()
    ) -> ErrorBottomModalViewModel {
        ErrorBottomModalViewModel(
            title: "Metadata Already Added: \(networkName) \(version)",
            content: "",
            steps: [],
            secondaryAction: .init(label: Localizable.TransactionSign.Action.error.key, action: action)
        )
    }

    static func signingInvalidNetworkVersion(
        _ networkName: String,
        _ action: @escaping @autoclosure () -> Void = {}()
    ) -> ErrorBottomModalViewModel {
        ErrorBottomModalViewModel(
            title: Localizable.TransactionSign.Error.InvalidNetworkVersion.title(networkName),
            content: Localizable.TransactionSign.Error.InvalidNetworkVersion.message(networkName),
            steps: [
                .init(
                    step: "1",
                    content: Localizable.signingInvalidNetworkVersionStepOne()
                ),
                .init(
                    step: "2",
                    content: AttributedString(Localizable.TransactionSign.Error.InvalidNetworkVersion.step2.string)
                ),
                .init(
                    step: "3",
                    content: AttributedString(Localizable.TransactionSign.Error.InvalidNetworkVersion.step3.string)
                )
            ],
            secondaryAction: .init(label: Localizable.TransactionSign.Action.error.key, action: action)
        )
    }

    static func signingUnknownNetwork(_ action: @escaping @autoclosure () -> Void = {}()) -> ErrorBottomModalViewModel {
        ErrorBottomModalViewModel(
            title: Localizable.TransactionSign.Error.UnknownNetwork.title.string,
            content: Localizable.TransactionSign.Error.UnknownNetwork.message.string,
            steps: [
                .init(
                    step: "1",
                    content: Localizable.signingUnknownNetworkStepOne()
                ),
                .init(
                    step: "2",
                    content: AttributedString(Localizable.TransactionSign.Error.UnknownNetwork.step2.string)
                ),
                .init(
                    step: "3",
                    content: AttributedString(Localizable.TransactionSign.Error.UnknownNetwork.step3.string)
                ),
                .init(
                    step: "4",
                    content: AttributedString(Localizable.TransactionSign.Error.UnknownNetwork.step4.string)
                )
            ],
            secondaryAction: .init(label: Localizable.TransactionSign.Action.error.key, action: action)
        )
    }
}

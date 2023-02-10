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
            title: Localizable.TransactionSign.Error.MetadataUnknownNetwork.title(networkName),
            content: Localizable.TransactionSign.Error.MetadataUnknownNetwork.message(networkName),
            steps: [
                .init(
                    step: "1",
                    content: Localizable.signingMetadataUnknownNetwork()
                ),
                .init(
                    step: "2",
                    content: AttributedString(Localizable.TransactionSign.Error.MetadataUnknownNetwork.step2.string)
                ),
                .init(
                    step: "3",
                    content: AttributedString(Localizable.TransactionSign.Error.MetadataUnknownNetwork.step3.string)
                )
            ],
            secondaryAction: .init(label: Localizable.TransactionSign.Action.error.key, action: action)
        )
    }

    static func networkAlreadyAdded(
        _ networkName: String,
        _: String,
        _ action: @escaping @autoclosure () -> Void = {}()
    ) -> ErrorBottomModalViewModel {
        ErrorBottomModalViewModel(
            title: Localizable.TransactionSign.Error.NetworkAlreadyAdded.title(networkName),
            content: Localizable.TransactionSign.Error.NetworkAlreadyAdded.message.string,
            secondaryAction: .init(label: Localizable.TransactionSign.Action.error.key, action: action)
        )
    }

    static func metadataAlreadyAdded(
        _ networkName: String,
        _ version: String,
        _ action: @escaping @autoclosure () -> Void = {}()
    ) -> ErrorBottomModalViewModel {
        ErrorBottomModalViewModel(
            title: Localizable.TransactionSign.Error.MetadataAlreadyAdded.title(networkName, version),
            content: Localizable.TransactionSign.Error.MetadataAlreadyAdded.message.string,
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

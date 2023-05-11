//
//  ErrorBottomModalViewModel+TransactionErrors.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 09/02/2023.
//

import Foundation
import SwiftUI

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

    static func outdatedMetadata(
        _ networkName: String,
        _ currentVersion: String,
        _ expectedVersion: String,
        _ action: @escaping @autoclosure () -> Void = {}()
    ) -> ErrorBottomModalViewModel {
        ErrorBottomModalViewModel(
            title: Localizable.TransactionSign.Error.OutdatedMetadata.title(networkName),
            content: Localizable.TransactionSign.Error.OutdatedMetadata.message(
                networkName,
                expectedVersion,
                currentVersion
            ),
            steps: [
                .init(
                    step: "1",
                    content: Localizable.signingOutdatedMetadataStepOne()
                ),
                .init(
                    step: "2",
                    content: AttributedString(Localizable.TransactionSign.Error.OutdatedMetadata.step2.string)
                ),
                .init(
                    step: "3",
                    content: AttributedString(Localizable.TransactionSign.Error.OutdatedMetadata.step3.string)
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
                )
            ],
            secondaryAction: .init(label: Localizable.TransactionSign.Action.error.key, action: action)
        )
    }

    static func noMetadataForNetwork(
        _ networkName: String,
        _ action: @escaping @autoclosure () -> Void = {}()
    ) -> ErrorBottomModalViewModel {
        ErrorBottomModalViewModel(
            title: Localizable.TransactionSign.Error.NoMetadataForNetwork.title(networkName),
            content: Localizable.TransactionSign.Error.NoMetadataForNetwork.message(networkName),
            steps: [
                .init(
                    step: "1",
                    content: Localizable.signingMetadataUnknownNetwork()
                ),
                .init(
                    step: "2",
                    content: AttributedString(Localizable.TransactionSign.Error.NoMetadataForNetwork.step2.string)
                ),
                .init(
                    step: "3",
                    content: AttributedString(Localizable.TransactionSign.Error.NoMetadataForNetwork.step3.string)
                )
            ],
            secondaryAction: .init(label: Localizable.TransactionSign.Action.error.key, action: action)
        )
    }
}

#if DEBUG
    struct ErrorBottomModalTransactionSigning_Previews: PreviewProvider {
        static var previews: some View {
            Group {
                ErrorBottomModal(
                    viewModel: .metadataForUnknownNetwork("Westend"),
                    isShowingBottomAlert: Binding<Bool>.constant(true)
                )
                ErrorBottomModal(
                    viewModel: .networkAlreadyAdded("Westend"),
                    isShowingBottomAlert: Binding<Bool>.constant(true)
                )
                ErrorBottomModal(
                    viewModel: .metadataAlreadyAdded("Westend", "3119"),
                    isShowingBottomAlert: Binding<Bool>.constant(true)
                )
                ErrorBottomModal(
                    viewModel: .outdatedMetadata("Westend", "3119", "3220"),
                    isShowingBottomAlert: Binding<Bool>.constant(true)
                )
                ErrorBottomModal(
                    viewModel: .signingUnknownNetwork(),
                    isShowingBottomAlert: Binding<Bool>.constant(true)
                )
                ErrorBottomModal(
                    viewModel: .noMetadataForNetwork("Westend"),
                    isShowingBottomAlert: Binding<Bool>.constant(true)
                )
            }
            .preferredColorScheme(.dark)
            .previewLayout(.sizeThatFits)
        }
    }
#endif

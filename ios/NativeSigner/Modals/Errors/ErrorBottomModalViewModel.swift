//
//  ErrorBottomModalViewModel.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 28/09/2022.
//

import SwiftUI

struct ErrorBottomModalViewModel {
    struct Step {
        let step: String
        let content: AttributedString
    }

    let icon: Image?
    let title: String
    let content: String
    let details: String?
    let steps: [Step]
    let primaryAction: ActionModel?
    let secondaryAction: ActionModel?
    let tertiaryAction: ActionModel?

    init(
        icon: Image? = nil,
        title: String,
        content: String,
        details: String? = nil,
        steps: [Step] = [],
        primaryAction: ActionModel? = nil,
        secondaryAction: ActionModel? = nil,
        tertiaryAction: ActionModel? = nil
    ) {
        self.icon = icon
        self.title = title
        self.content = content
        self.details = details
        self.steps = steps
        self.primaryAction = primaryAction
        self.secondaryAction = secondaryAction
        self.tertiaryAction = tertiaryAction
    }

    static func signingForgotPassword(_ action: @escaping @autoclosure () -> Void = {}()) -> ErrorBottomModalViewModel {
        ErrorBottomModalViewModel(
            title: Localizable.Transaction.EnterPassword.Error.title.string,
            content: Localizable.Transaction.EnterPassword.Error.message.string,
            secondaryAction: .init(label: Localizable.ErrorModal.Action.ok.key, action: action)
        )
    }

    static func importDerivedKeysMissingNetwork(_ action: @escaping @autoclosure () -> Void = {}())
        -> ErrorBottomModalViewModel {
        ErrorBottomModalViewModel(
            title: Localizable.ImportKeys.ErrorModal.MissingNetwork.Label.title.string,
            content: Localizable.ImportKeys.ErrorModal.MissingNetwork.Label.content.string,
            secondaryAction: .init(
                label: Localizable.ImportKeys.ErrorModal.MissingNetwork.Action.ok.key,
                action: action
            )
        )
    }

    static func importDerivedKeysMissingKeySet(_ action: @escaping @autoclosure () -> Void = {}())
        -> ErrorBottomModalViewModel {
        ErrorBottomModalViewModel(
            title: Localizable.ImportKeys.ErrorModal.MissingKeySets.Label.title.string,
            content: Localizable.ImportKeys.ErrorModal.MissingKeySets.Label.content.string,
            secondaryAction: .init(
                label: Localizable.ImportKeys.ErrorModal.MissingKeySets.Action.ok.key,
                action: action
            )
        )
    }

    static func importDerivedKeysBadFormat(_ action: @escaping @autoclosure () -> Void = {}())
        -> ErrorBottomModalViewModel {
        ErrorBottomModalViewModel(
            title: Localizable.ImportKeys.ErrorModal.BadFormat.Label.title.string,
            content: Localizable.ImportKeys.ErrorModal.BadFormat.Label.content.string,
            secondaryAction: .init(label: Localizable.ImportKeys.ErrorModal.BadFormat.Action.ok.key, action: action)
        )
    }

    static func allKeysAlreadyExist(_ action: @escaping @autoclosure () -> Void = {}())
        -> ErrorBottomModalViewModel {
        ErrorBottomModalViewModel(
            title: Localizable.ImportKeys.ErrorModal.AlreadyExists.Label.title.string,
            content: Localizable.ImportKeys.ErrorModal.AlreadyExists.Label.content.string,
            secondaryAction: .init(label: Localizable.ImportKeys.ErrorModal.AlreadyExists.Action.ok.key, action: action)
        )
    }

    static func signingUnknownNetwork(
        _ networkName: String,
        _ action: @escaping @autoclosure () -> Void = {}()
    ) -> ErrorBottomModalViewModel {
        ErrorBottomModalViewModel(
            title: Localizable.TransactionSign.Error.InvalidNetworkVersion.title(networkName),
            content: Localizable.TransactionSign.Error.InvalidNetworkVersion.message.string,
            steps: [
                .init(
                    step: "1",
                    content: {
                        var stepOnePrefix = AttributedString(
                            Localizable.TransactionSign.Error.InvalidNetworkVersion.step1
                                .string
                        )
                        stepOnePrefix.foregroundColor = Asset.textAndIconsPrimary.swiftUIColor
                        var stepOneSuffix = AttributedString(
                            Localizable.TransactionSign.Error.InvalidNetworkVersion.Step1.suffix
                                .string
                        )
                        stepOneSuffix.foregroundColor = Asset.accentPink300.swiftUIColor
                        return stepOnePrefix + stepOneSuffix
                    }()
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

    static func signingInvalidNetworkVersion(_ action: @escaping @autoclosure () -> Void = {}())
        -> ErrorBottomModalViewModel {
        ErrorBottomModalViewModel(
            title: Localizable.TransactionSign.Error.UnknownNetwork.title.string,
            content: Localizable.TransactionSign.Error.UnknownNetwork.message.string,
            steps: [
                .init(
                    step: "1",
                    content: {
                        var stepOnePrefix = AttributedString(
                            Localizable.TransactionSign.Error.UnknownNetwork.step1
                                .string
                        )
                        stepOnePrefix.foregroundColor = Asset.textAndIconsPrimary.swiftUIColor
                        var stepOneSuffix = AttributedString(
                            Localizable.TransactionSign.Error.UnknownNetwork.Step1.suffix
                                .string
                        )
                        stepOneSuffix.foregroundColor = Asset.accentPink300.swiftUIColor
                        return stepOnePrefix + stepOneSuffix
                    }()
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

    static func connectivityOn(_ action: @escaping @autoclosure () -> Void = {}()) -> ErrorBottomModalViewModel {
        ErrorBottomModalViewModel(
            icon: Asset.wifiOn.swiftUIImage,
            title: Localizable.Connectivity.Label.title.string,
            content: Localizable.Connectivity.Label.content.string,
            secondaryAction: .init(label: Localizable.ErrorModal.Action.ok.key, action: action)
        )
    }

    static func connectivityWasOn(
        backAction: @escaping @autoclosure () -> Void = {}(),
        continueAction: @escaping @autoclosure () -> Void
    ) -> ErrorBottomModalViewModel {
        ErrorBottomModalViewModel(
            icon: Asset.wifiWasOn.swiftUIImage,
            title: Localizable.PastConnectivity.Label.title.string,
            content: Localizable.PastConnectivity.Label.content.string,
            primaryAction: .init(label: Localizable.PastConnectivity.Action.back.key, action: backAction),
            tertiaryAction: .init(label: Localizable.PastConnectivity.Action.continue.key, action: continueAction)
        )
    }

    static func alertError(
        message: String,
        _ action: @escaping @autoclosure () -> Void = {}()
    ) -> ErrorBottomModalViewModel {
        ErrorBottomModalViewModel(
            title: Localizable.GenericErrorModal.Label.title.string,
            content: Localizable.GenericErrorModal.Label.messagePrefix.string,
            details: message,
            secondaryAction: .init(label: Localizable.GenericErrorModal.Action.ok.key, action: action)
        )
    }

    static func transactionSigningError(
        message: String,
        _ action: @escaping @autoclosure () -> Void = {}()
    ) -> ErrorBottomModalViewModel {
        ErrorBottomModalViewModel(
            title: Localizable.TransactionSign.Error.Generic.title.string,
            content: Localizable.GenericErrorModal.Label.messagePrefix.string,
            details: message,
            secondaryAction: .init(label: Localizable.GenericErrorModal.Action.ok.key, action: action)
        )
    }

    static func seedPhraseAlreadyExists(_ action: @escaping @autoclosure () -> Void = {}())
        -> ErrorBottomModalViewModel {
        ErrorBottomModalViewModel(
            title: Localizable.EnterBananaSplitPasswordModal.Error.SeedPhraseExists.title.string,
            content: Localizable.EnterBananaSplitPasswordModal.Error.SeedPhraseExists.message.string,
            secondaryAction: .init(label: Localizable.ErrorModal.Action.ok.key, action: action)
        )
    }

    static func derivedKeysInfo(_ action: @escaping @autoclosure () -> Void = {}())
        -> ErrorBottomModalViewModel {
        ErrorBottomModalViewModel(
            title: Localizable.CreateDerivedKey.InfoModal.DerivedKeys.title.string,
            content: Localizable.CreateDerivedKey.InfoModal.DerivedKeys.content.string,
            secondaryAction: .init(label: Localizable.CreateDerivedKey.InfoModal.DerivedKeys.action.key, action: action)
        )
    }

    static func derivationPathsInfo(_ action: @escaping @autoclosure () -> Void = {}())
        -> ErrorBottomModalViewModel {
        ErrorBottomModalViewModel(
            title: Localizable.CreateDerivedKey.InfoModal.DerivationPaths.title.string,
            content: Localizable.CreateDerivedKey.InfoModal.DerivationPaths.content.string,
            secondaryAction: .init(
                label: Localizable.CreateDerivedKey.InfoModal.DerivationPaths.action.key,
                action: action
            )
        )
    }
}

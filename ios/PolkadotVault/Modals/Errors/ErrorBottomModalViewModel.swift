//
//  ErrorBottomModalViewModel.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 28/09/2022.
//

import SwiftUI

struct ErrorBottomModalViewModel: Equatable {
    struct Step: Equatable {
        let step: String
        let content: AttributedString
    }

    let icon: Image?
    let title: String
    let content: String
    let attributedContent: AttributedString?
    let details: String?
    let steps: [Step]
    let primaryAction: ActionModel?
    let secondaryAction: ActionModel?
    let tertiaryAction: ActionModel?

    init(
        icon: Image? = nil,
        title: String,
        content: String = "",
        attributedContent: AttributedString? = nil,
        details: String? = nil,
        steps: [Step] = [],
        primaryAction: ActionModel? = nil,
        secondaryAction: ActionModel? = nil,
        tertiaryAction: ActionModel? = nil
    ) {
        self.icon = icon
        self.title = title
        self.content = content
        self.attributedContent = attributedContent
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

    static func seedPhraseAlreadyExists(_ action: @escaping @autoclosure () -> Void = {}())
        -> ErrorBottomModalViewModel {
        ErrorBottomModalViewModel(
            title: Localizable.EnterBananaSplitPasswordView.Error.SeedPhraseExists.title.string,
            content: Localizable.EnterBananaSplitPasswordView.Error.SeedPhraseExists.message.string,
            secondaryAction: .init(label: Localizable.ErrorModal.Action.ok.key, action: action)
        )
    }

    static func noNetworksAvailable(_ action: @escaping @autoclosure () -> Void = {}()) -> ErrorBottomModalViewModel {
        ErrorBottomModalViewModel(
            title: Localizable.KeyDetails.Error.NoNetworks.title.string,
            content: Localizable.KeyDetails.Error.NoNetworks.message.string,
            secondaryAction: .init(label: Localizable.ErrorModal.Action.ok.key, action: action)
        )
    }

    static func bananaSplitExplanation(
        _ action: @escaping @autoclosure () -> Void = {}()
    ) -> ErrorBottomModalViewModel {
        ErrorBottomModalViewModel(
            title: Localizable.NewSeed.Backup.BananaSplit.Label.title.string,
            attributedContent: Localizable.bananaSplitExplanation(),
            secondaryAction: .init(label: Localizable.ErrorModal.Action.ok.key, action: action)
        )
    }

    static func recoverySeedPhraseIncorrectPhrase(
        _ action: @escaping @autoclosure () -> Void = {}()
    ) -> ErrorBottomModalViewModel {
        ErrorBottomModalViewModel(
            title: Localizable.RecoverSeedPhrase.Error.IncorrectPhrase.title.string,
            content: Localizable.RecoverSeedPhrase.Error.IncorrectPhrase.message.string,
            secondaryAction: .init(label: Localizable.ErrorModal.Action.ok.key, action: action)
        )
    }

    static func featureNotAvailable(
        _ action: @escaping @autoclosure () -> Void = {}()
    ) -> ErrorBottomModalViewModel {
        ErrorBottomModalViewModel(
            title: Localizable.Error.FeatureNotAvailable.title.string,
            content: Localizable.Error.FeatureNotAvailable.message.string,
            secondaryAction: .init(label: Localizable.ErrorModal.Action.ok.key, action: action)
        )
    }
}

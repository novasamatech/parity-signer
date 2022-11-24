//
//  ErrorBottomModalViewModel.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 28/09/2022.
//

import SwiftUI

struct ErrorBottomModalViewModel {
    let icon: Image?
    let title: String
    let content: String
    let details: String?
    let primaryAction: ActionModel?
    let secondaryAction: ActionModel?
    let tertiaryAction: ActionModel?

    init(
        icon: Image? = nil,
        title: String,
        content: String,
        details: String? = nil,
        primaryAction: ActionModel? = nil,
        secondaryAction: ActionModel? = nil,
        tertiaryAction: ActionModel? = nil
    ) {
        self.icon = icon
        self.title = title
        self.content = content
        self.details = details
        self.primaryAction = primaryAction
        self.secondaryAction = secondaryAction
        self.tertiaryAction = tertiaryAction
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
}

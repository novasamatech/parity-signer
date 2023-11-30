//
//  NetworkSettingsDetailsActionModal.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 26/12/2022.
//

import SwiftUI

struct NetworkSettingsDetailsActionModal: View {
    @State private var animateBackground: Bool = false
    @Binding var isShowingActionSheet: Bool
    @Binding var shouldPresentRemoveNetworkConfirmation: Bool
    @Binding var shouldSignSpecs: Bool

    var body: some View {
        FullScreenRoundedModal(
            backgroundTapAction: { animateDismissal() },
            animateBackground: $animateBackground,
            content: {
                VStack(alignment: .leading, spacing: 0) {
                    // Sign Specs
                    ActionSheetButton(
                        action: { animateDismissal { shouldSignSpecs = true } },
                        icon: Image(.signSpecs),
                        text: Localizable.Settings.NetworkDetails.More.Action.sign.key
                    )
                    // Remove Network
                    ActionSheetButton(
                        action: { animateDismissal { shouldPresentRemoveNetworkConfirmation = true } },
                        icon: Image(.delete),
                        text: Localizable.Settings.NetworkDetails.More.Action.delete.key,
                        style: .destructive
                    )
                    ActionButton(
                        action: { animateDismissal() },
                        text: Localizable.LogsList.More.Action.cancel.key,
                        style: .emptySecondary()
                    )
                }
                .padding(.horizontal, Spacing.large)
                .padding(.top, -Spacing.extraSmall)
                .padding(.bottom, Spacing.medium)
            }
        )
    }

    private func animateDismissal(_ completion: @escaping () -> Void = {}) {
        Animations.chainAnimation(
            animateBackground.toggle(),
            delayedAnimationClosure: {
                isShowingActionSheet = false
                completion()
            }()
        )
    }
}

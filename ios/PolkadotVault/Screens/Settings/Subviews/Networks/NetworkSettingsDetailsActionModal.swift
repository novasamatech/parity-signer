//
//  NetworkSettingsDetailsActionModal.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 26/12/2022.
//

import SwiftUI

struct NetworkSettingsDetailsActionModal: View {
    @StateObject var viewModel: ViewModel

    var body: some View {
        FullScreenRoundedModal(
            backgroundTapAction: { viewModel.dismissActionSheet() },
            animateBackground: $viewModel.animateBackground,
            content: {
                VStack(alignment: .leading, spacing: 0) {
                    // Sign Specs
                    ActionSheetButton(
                        action: { viewModel.toggleSignSpecs() },
                        icon: Image(.signSpecs),
                        text: Localizable.Settings.NetworkDetails.More.Action.sign.key
                    )
                    // Remove Network
                    ActionSheetButton(
                        action: { viewModel.toggleRemoveNetworkConfirmation() },
                        icon: Image(.delete),
                        text: Localizable.Settings.NetworkDetails.More.Action.delete.key,
                        style: .destructive
                    )
                    ActionButton(
                        action: { viewModel.dismissActionSheet() },
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
}

extension NetworkSettingsDetailsActionModal {
    final class ViewModel: ObservableObject {
        @Published var animateBackground: Bool = false
        @Binding var isPresented: Bool
        @Binding var shouldPresentRemoveNetworkConfirmation: Bool
        @Binding var shouldSignSpecs: Bool

        init(
            isPresented: Binding<Bool>,
            shouldPresentRemoveNetworkConfirmation: Binding<Bool>,
            shouldSignSpecs: Binding<Bool>
        ) {
            _isPresented = isPresented
            _shouldPresentRemoveNetworkConfirmation = shouldPresentRemoveNetworkConfirmation
            _shouldSignSpecs = shouldSignSpecs
        }

        func toggleSignSpecs() {
            shouldSignSpecs = true
            animateDismissal()
        }

        func toggleRemoveNetworkConfirmation() {
            shouldPresentRemoveNetworkConfirmation = true
            animateDismissal()
        }

        func dismissActionSheet() {
            animateDismissal()
        }

        func animateDismissal() {
            Animations.chainAnimation(
                animateBackground.toggle(),
                // swiftformat:disable all
                delayedAnimationClosure: self.hide()
            )
        }

        private func hide() {
            isPresented = false
        }
    }
}

#if DEBUG
    struct NetworkSettingsDetailsActionModal_Previews: PreviewProvider {
        static var previews: some View {
            NetworkSettingsDetailsActionModal(
                viewModel: .init(
                    isPresented: .constant(true),
                    shouldPresentRemoveNetworkConfirmation: .constant(false),
                    shouldSignSpecs: .constant(false)
                )
            )
        }
    }
#endif

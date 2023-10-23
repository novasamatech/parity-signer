//
//  PublicKeyActionsModal.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 14/09/2022.
//

import SwiftUI

struct PublicKeyActionsModal: View {
    @State private var animateBackground: Bool = false
    @Binding private var shouldPresentExportKeysWarningModal: Bool
    @Binding private var isShowingActionSheet: Bool
    @Binding private var shouldPresentRemoveConfirmationModal: Bool
    private let isExportKeyAvailable: Bool

    init(
        shouldPresentExportKeysWarningModal: Binding<Bool> = Binding<Bool>.constant(false),
        isShowingActionSheet: Binding<Bool> = Binding<Bool>.constant(false),
        shouldPresentRemoveConfirmationModal: Binding<Bool> = Binding<Bool>.constant(false),
        isExportKeyAvailable: Bool
    ) {
        _shouldPresentExportKeysWarningModal = shouldPresentExportKeysWarningModal
        _isShowingActionSheet = isShowingActionSheet
        _shouldPresentRemoveConfirmationModal = shouldPresentRemoveConfirmationModal
        self.isExportKeyAvailable = isExportKeyAvailable
    }

    var body: some View {
        FullScreenRoundedModal(
            backgroundTapAction: { animateDismissal() },
            animateBackground: $animateBackground,
            content: {
                VStack(alignment: .leading, spacing: 0) {
                    if isExportKeyAvailable {
                        ActionSheetButton(
                            action: {
                                animateDismissal {
                                    shouldPresentExportKeysWarningModal = true
                                }
                            },
                            icon: Image(.exportPrivateKeySmall),
                            text: Localizable.PublicKeyDetailsModal.Action.share.key
                        )
                    }
                    ActionSheetButton(
                        action: { animateDismissal { shouldPresentRemoveConfirmationModal.toggle() } },
                        icon: Image(.delete),
                        text: Localizable.PublicKeyDetailsModal.Action.delete.key,
                        style: .destructive
                    )
                    EmptyButton(
                        action: animateDismissal(),
                        text: Localizable.AddKeySet.Button.cancel.key,
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

#if DEBUG
    struct PublicKeyActionsModal_Previews: PreviewProvider {
        static var previews: some View {
            PublicKeyActionsModal(
                isShowingActionSheet: Binding<Bool>.constant(true),
                isExportKeyAvailable: true
            )
            .preferredColorScheme(.dark)
            .previewLayout(.sizeThatFits)
            VStack {
                PublicKeyActionsModal(
                    isShowingActionSheet: Binding<Bool>.constant(true),
                    isExportKeyAvailable: false
                )
                .preferredColorScheme(.light)
                .previewLayout(.sizeThatFits)
            }
            .background(.black)
            .preferredColorScheme(.light)
            .previewLayout(.sizeThatFits)
        }
    }
#endif

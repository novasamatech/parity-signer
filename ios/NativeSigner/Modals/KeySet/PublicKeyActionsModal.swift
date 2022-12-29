//
//  PublicKeyActionsModal.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 14/09/2022.
//

import SwiftUI

struct PublicKeyActionsModal: View {
    @State private var animateBackground: Bool = false
    @Binding private var shouldPresentExportKeysWarningModal: Bool
    @Binding private var isShowingActionSheet: Bool
    @Binding private var shouldPresentRemoveConfirmationModal: Bool
    @EnvironmentObject private var navigation: NavigationCoordinator

    init(
        shouldPresentExportKeysWarningModal: Binding<Bool> = Binding<Bool>.constant(false),
        isShowingActionSheet: Binding<Bool> = Binding<Bool>.constant(false),
        shouldPresentRemoveConfirmationModal: Binding<Bool> = Binding<Bool>.constant(false)
    ) {
        _shouldPresentExportKeysWarningModal = shouldPresentExportKeysWarningModal
        _isShowingActionSheet = isShowingActionSheet
        _shouldPresentRemoveConfirmationModal = shouldPresentRemoveConfirmationModal
    }

    var body: some View {
        FullScreenRoundedModal(
            backgroundTapAction: { animateDismissal() },
            animateBackground: $animateBackground,
            content: {
                VStack(alignment: .leading, spacing: 0) {
                    // Don't show `Export Private Key` if intermediate state is broken or when key is password protected
                    if let currentKeyDetails = navigation.currentKeyDetails,
                       currentKeyDetails.address.hasPwd == false {
                        ActionSheetButton(
                            action: {
                                animateDismissal {
                                    shouldPresentExportKeysWarningModal = true
                                }
                            },
                            icon: Asset.exportPrivateKeySmall.swiftUIImage,
                            text: Localizable.PublicKeyDetailsModal.Action.share.key
                        )
                    }
                    ActionSheetButton(
                        action: { animateDismissal { shouldPresentRemoveConfirmationModal.toggle() } },
                        icon: Asset.delete.swiftUIImage,
                        text: Localizable.PublicKeyDetailsModal.Action.delete.key,
                        style: .destructive
                    )
                    EmptyButton(
                        action: { animateDismissal() },
                        text: Localizable.AddKeySet.Button.cancel.key,
                        style: .emptySecondary()
                    )
                }
                .padding([.leading, .trailing], Spacing.large)
                .padding(.top, -Spacing.extraSmall)
                .padding(.bottom, Spacing.medium)
            }
        )
    }

    private func animateDismissal(_ completion: @escaping () -> Void = {}) {
        Animations.chainAnimation(
            animateBackground.toggle(),
            delayedAnimationClosure: {
                isShowingActionSheet.toggle()
                completion()
            }()
        )
    }
}

struct PublicKeyActionsModal_Previews: PreviewProvider {
    static var previews: some View {
        PublicKeyActionsModal(
            isShowingActionSheet: Binding<Bool>.constant(true)
        )
        .preferredColorScheme(.dark)
        .previewLayout(.sizeThatFits)
        VStack {
            PublicKeyActionsModal(
                isShowingActionSheet: Binding<Bool>.constant(true)
            )
            .preferredColorScheme(.light)
            .previewLayout(.sizeThatFits)
        }
        .background(.black)
        .preferredColorScheme(.light)
        .previewLayout(.sizeThatFits)
        .environmentObject(NavigationCoordinator())
    }
}

//
//  KeyDetailsActionsModal.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 06/09/2022.
//

import SwiftUI

struct KeyDetailsActionsModal: View {
    @State private var animateBackground: Bool = false
    @Binding var isShowingActionSheet: Bool
    @Binding var shouldPresentRemoveConfirmationModal: Bool
    @Binding var shouldPresentBackupModal: Bool
    @Binding var shouldPresentSelectionOverlay: Bool

    var body: some View {
        FullScreenRoundedModal(
            backgroundTapAction: { animateDismissal() },
            animateBackground: $animateBackground,
            content: {
                VStack(alignment: .leading, spacing: 0) {
                    // Export Keys
                    ActionSheetButton(
                        action: { animateDismissal { shouldPresentSelectionOverlay.toggle() } },
                        icon: Asset.selectUnselected.swiftUIImage,
                        text: Localizable.KeySetsModal.Action.export.key
                    )
                    ActionSheetButton(
                        action: {
                            animateDismissal { shouldPresentBackupModal.toggle() }
                        },
                        icon: Asset.backupKey.swiftUIImage,
                        text: Localizable.KeySetsModal.Action.backup.key
                    )
                    ActionSheetButton(
                        action: { animateDismissal { shouldPresentRemoveConfirmationModal.toggle() } },
                        icon: Asset.delete.swiftUIImage,
                        text: Localizable.KeySetsModal.Action.delete.key,
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
                isShowingActionSheet = false
                completion()
            }()
        )
    }
}

// struct KeyDetailsActionsModal_Previews: PreviewProvider {
//    static var previews: some View {
//        KeyDetailsActionsModal(
//            isShowingActionSheet: Binding<Bool>.constant(true),
//            removeSeed: {}
//        )
//        .preferredColorScheme(.dark)
//        .previewLayout(.sizeThatFits)
//        VStack {
//            KeyDetailsActionsModal(
//                isShowingActionSheet: Binding<Bool>.constant(true),
//                removeSeed: {}
//            )
//            .preferredColorScheme(.light)
//            .previewLayout(.sizeThatFits)
//        }
//        .background(.black)
//        .preferredColorScheme(.light)
//        .previewLayout(.sizeThatFits)
//    }
// }

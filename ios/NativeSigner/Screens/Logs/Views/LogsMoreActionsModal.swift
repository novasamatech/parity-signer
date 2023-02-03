//
//  LogsMoreActionsModal.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 07/12/2022.
//

import SwiftUI

struct LogsMoreActionsModal: View {
    @State private var animateBackground: Bool = false
    @Binding var isShowingActionSheet: Bool
    @Binding var shouldPresentClearConfirmationModal: Bool
    @Binding var shouldPresentAddNoteModal: Bool
    @EnvironmentObject private var navigation: NavigationCoordinator

    var body: some View {
        FullScreenRoundedModal(
            backgroundTapAction: { animateDismissal() },
            animateBackground: $animateBackground,
            content: {
                VStack(alignment: .leading, spacing: 0) {
                    // Add Note
                    ActionSheetButton(
                        action: { animateDismissal { shouldPresentAddNoteModal.toggle() } },
                        icon: Asset.add.swiftUIImage,
                        text: Localizable.LogsList.More.Action.add.key
                    )
                    // Clear Log
                    ActionSheetButton(
                        action: { animateDismissal { shouldPresentClearConfirmationModal.toggle() } },
                        icon: Asset.delete.swiftUIImage,
                        text: Localizable.LogsList.More.Action.clear.key,
                        style: .destructive
                    )
                    EmptyButton(
                        action: { animateDismissal() },
                        text: Localizable.LogsList.More.Action.cancel.key,
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
//            navigation: NavigationCoordinator(),
//            removeSeed: {}
//        )
//        .preferredColorScheme(.dark)
//        .previewLayout(.sizeThatFits)
//        VStack {
//            KeyDetailsActionsModal(
//                isShowingActionSheet: Binding<Bool>.constant(true),
//                navigation: NavigationCoordinator(),
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

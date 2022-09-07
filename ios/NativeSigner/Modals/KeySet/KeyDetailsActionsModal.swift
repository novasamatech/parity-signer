//
//  KeyDetailsActionsModal.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 06/09/2022.
//

import SwiftUI

struct KeyDetailsActionsModal: View {
    @State private var removeConfirm = false
    @State private var animateBackground: Bool = false

    @Binding var isShowingActionSheet: Bool
    @ObservedObject var navigation: NavigationCoordinator
    let removeSeed: () -> Void

    var body: some View {
        FullScreenRoundedModal(
            backgroundTapAction: { animateDismissal() },
            animateBackground: $animateBackground,
            content: {
                VStack(alignment: .leading, spacing: 0) {
                    ActionSheetButton(
                        action: {
                            animateDismissal {
                                navigation.perform(navigation: .init(action: .newSeed))
                            }
                        },
                        icon: Asset.selectUnselected.swiftUIImage,
                        text: Localizable.KeySetsModal.Action.select.key
                    )
                    ActionSheetButton(
                        action: {
                            animateDismissal {
                                navigation.perform(navigation: .init(action: .newKey))
                            }
                        },
                        icon: Asset.deriveKey.swiftUIImage,
                        text: Localizable.KeySetsModal.Action.derive.key
                    )
                    ActionSheetButton(
                        action: {
                            animateDismissal {
                                navigation.perform(navigation: .init(action: .backupSeed))
                            }
                        },
                        icon: Asset.backupKey.swiftUIImage,
                        text: Localizable.KeySetsModal.Action.backup.key
                    )
                    ActionSheetButton(
                        action: { removeConfirm.toggle() },
                        icon: Asset.delete.swiftUIImage,
                        text: Localizable.KeySetsModal.Action.delete.key,
                        style: .destructive
                    )
                    EmptyButton(
                        action: { animateDismissal() },
                        text: Localizable.AddKeySet.Button.cancel.key,
                        foregroundColor: Asset.textAndIconsSecondary.swiftUIColor
                    )
                }
                .padding([.leading, .trailing], Spacing.large)
                .padding(.top, -Spacing.extraSmall)
            }
        )
        .onAppear {
            // We need to fake right button action here, or Rust state machine won't work for `Backup` action
            navigation.perform(navigation: .init(action: .rightButtonAction))
        }
        .alert(isPresented: $removeConfirm, content: {
            Alert(
                title: Localizable.forgetThisSeed.text,
                message: Localizable.ThisSeedWillBeRemovedForAllNetworks.ThisIsNotReversible.areYouSure.text,
                primaryButton: .destructive(
                    Localizable.removeSeed.text,
                    action: removeSeed
                ),
                secondaryButton: .cancel(Localizable.cancel.text)
            )
        })
    }

    private func animateDismissal(_ completion: @escaping () -> Void = {}) {
        withAnimation(
            Animation.easeIn(duration: AnimationDuration.standard)
        ) {
            animateBackground.toggle()
        }
        DispatchQueue.main.asyncAfter(deadline: .now() + AnimationDuration.standard) {
            withAnimation(
                Animation.easeIn(duration: AnimationDuration.standard)
            ) {
                isShowingActionSheet.toggle()
                completion()
            }
        }
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

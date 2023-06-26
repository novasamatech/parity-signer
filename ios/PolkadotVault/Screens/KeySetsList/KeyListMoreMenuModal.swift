//
//  KeyListMoreMenuModal.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 18/10/2022.
//

import SwiftUI

struct KeyListMoreMenuModal: View {
    @Binding var isPresented: Bool
    @Binding var isExportKeysSelected: Bool
    @State private var animateBackground: Bool = false

    var body: some View {
        FullScreenRoundedModal(
            backgroundTapAction: {
                animateDismissal()
            },
            animateBackground: $animateBackground,
            content: {
                VStack(alignment: .leading) {
                    ActionSheetButton(
                        action: {
                            animateDismissal {
                                isExportKeysSelected.toggle()
                            }
                        },
                        icon: Asset.exportKeys.swiftUIImage,
                        text: Localizable.KeySets.More.Action.export.key
                    )
                    EmptyButton(
                        action: animateDismissal(),
                        text: Localizable.AddKeySet.Button.cancel.key,
                        style: .emptySecondary()
                    )
                }
                .padding(.horizontal, Spacing.large)
                .padding(.bottom, Spacing.extraSmall + Spacing.medium)
            }
        )
    }

    private func animateDismissal(_ completion: @escaping () -> Void = {}) {
        Animations.chainAnimation(
            animateBackground.toggle(),
            delayedAnimationClosure: {
                isPresented = false
                completion()
            }()
        )
    }
}

#if DEBUG
    struct KeyListMoreMenuModal_Previews: PreviewProvider {
        static var previews: some View {
            KeyListMoreMenuModal(
                isPresented: Binding<Bool>.constant(true),
                isExportKeysSelected: Binding<Bool>.constant(false)
            )
            .preferredColorScheme(.dark)
            .previewLayout(.sizeThatFits)
        }
    }
#endif

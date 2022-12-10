//
//  AddKeySetModal.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 24/08/2022.
//

import SwiftUI

struct AddKeySetModal: View {
    @Binding var isShowingNewSeedMenu: Bool
    @State private var animateBackground: Bool = false
    @EnvironmentObject private var navigation: NavigationCoordinator

    var body: some View {
        FullScreenRoundedModal(
            backgroundTapAction: {
                animateDismissal {
                    navigation.perform(navigation: .init(action: .rightButtonAction))
                }
            },
            animateBackground: $animateBackground,
            content: {
                VStack(alignment: .leading) {
                    Localizable.AddKeySet.title.text
                        .foregroundColor(Asset.textAndIconsSecondary.swiftUIColor)
                        .font(PrimaryFont.titleS.font)
                        .padding([.bottom, .top], Spacing.medium)
                    Divider()
                    ActionSheetButton(
                        action: {
                            animateDismissal {
                                navigation.perform(navigation: .init(action: .newSeed))
                            }
                        },
                        icon: Asset.add.swiftUIImage,
                        text: Localizable.AddKeySet.Button.add.key
                    )
                    ActionSheetButton(
                        action: {
                            animateDismissal {
                                navigation.perform(navigation: .init(action: .recoverSeed))
                            }
                        },
                        icon: Asset.recover.swiftUIImage,
                        text: Localizable.AddKeySet.Button.recover.key
                    )
                    EmptyButton(
                        action: {
                            animateDismissal {
                                navigation.perform(navigation: .init(action: .rightButtonAction))
                            }
                        },
                        text: Localizable.AddKeySet.Button.cancel.key,
                        style: .emptySecondary()
                    )
                }
                .padding([.leading, .trailing], Spacing.large)
                .padding(.bottom, Spacing.small + Spacing.medium)
            }
        )
    }

    private func animateDismissal(_ completion: @escaping () -> Void = {}) {
        Animations.chainAnimation(
            animateBackground.toggle(),
            delayedAnimationClosure: {
                isShowingNewSeedMenu.toggle()
                completion()
            }()
        )
    }
}

// struct AddKeySetModal_Previews: PreviewProvider {
//    static var previews: some View {
//        AddKeySetModal(
//            isShowingNewSeedMenu: Binding<Bool>.constant(true),
//            navigation: NavigationCoordinator()
//        )
//        .preferredColorScheme(.dark)
//        .previewLayout(.sizeThatFits)
//    }
// }

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
    @ObservedObject var navigation: NavigationCoordinator

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
                        .font(Fontstyle.titleS.base)
                        .padding([.bottom, .top], Spacing.medium)
                    Divider()
                    MenuButton(
                        action: {
                            animateDismissal {
                                navigation.perform(navigation: .init(action: .newSeed))
                            }
                        },
                        icon: Asset.add.swiftUIImage,
                        text: Localizable.AddKeySet.Button.add.key
                    )
                    MenuButton(
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
                        foregroundColor: Asset.textAndIconsSecondary.swiftUIColor
                    )
                }
                .padding([.leading, .trailing], Spacing.large)
                .padding(.bottom, Spacing.small)
            }
        )
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
                isShowingNewSeedMenu.toggle()
                completion()
            }
        }
    }
}

struct AddKeySetModal_Previews: PreviewProvider {
    static var previews: some View {
        AddKeySetModal(
            isShowingNewSeedMenu: Binding<Bool>.constant(true),
            navigation: NavigationCoordinator()
        )
        .preferredColorScheme(.dark)
        .previewLayout(.sizeThatFits)
    }
}

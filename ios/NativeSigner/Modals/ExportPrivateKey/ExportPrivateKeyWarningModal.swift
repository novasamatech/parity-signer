//
//  ExportPrivateKeyWarningModal.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 01/09/2022.
//

import SwiftUI

struct ExportPrivateKeyWarningModal: View {
    @Binding var isPresentingExportKeysWarningModal: Bool
    @Binding var shouldPresentExportKeysModal: Bool
    @State private var animateBackground: Bool = false

    var body: some View {
        FullScreenRoundedModal(
            backgroundTapAction: {
                animateDismissal()
            },
            animateBackground: $animateBackground,
            content: {
                // Modal content
                VStack(alignment: .center, spacing: Spacing.medium) {
                    Asset.privateKeyIcon.swiftUIImage
                        .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                        .padding([.bottom, .top], Spacing.small)
                    Localizable.KeyExportWarning.Label.header.text
                        .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                        .font(PrimaryFont.titleL.font)
                        .padding(.bottom, Spacing.extraSmall)
                    Localizable.KeyExportWarning.Label.content.text
                        .foregroundColor(Asset.textAndIconsSecondary.swiftUIColor)
                        .font(PrimaryFont.bodyL.font)
                        .multilineTextAlignment(.center)
                        .padding(.bottom, Spacing.small)
                    PrimaryButton(
                        action: {
                            shouldPresentExportKeysModal.toggle()
                            animateDismissal()
                        },
                        text: Localizable.KeyExportWarning.Action.export.key
                    )
                    EmptyButton(
                        action: {
                            animateDismissal()
                        },
                        text: Localizable.KeyExportWarning.Action.cancel.key
                    )
                }
                .padding([.leading, .trailing], Spacing.large)
                .padding(.bottom, Spacing.medium)
            }
        )
    }

    private func animateDismissal() {
        Animations.chainAnimation(
            animateBackground.toggle(),
            delayedAnimationClosure: isPresentingExportKeysWarningModal.toggle()
        )
    }
}

struct ExportPrivateKeyWarningModal_Previews: PreviewProvider {
    static var previews: some View {
        VStack {
            ExportPrivateKeyWarningModal(
                isPresentingExportKeysWarningModal: Binding<Bool>.constant(true),
                shouldPresentExportKeysModal: Binding<Bool>.constant(false)
            )
        }
        .background(.red)
        .preferredColorScheme(.dark)
        .previewLayout(.sizeThatFits)
    }
}

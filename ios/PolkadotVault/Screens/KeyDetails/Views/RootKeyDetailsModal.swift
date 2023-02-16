//
//  RootKeyDetailsModal.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 28/12/2022.
//

import SwiftUI

struct RootKeyDetailsModal: View {
    @State private var animateBackground: Bool = false
    @Binding var isPresented: Bool
    let viewModel: ViewModel

    var body: some View {
        FullScreenRoundedModal(
            backgroundTapAction: {
                animateDismissal()
            },
            animateBackground: $animateBackground,
            safeAreaInsetsMode: .partial,
            content: {
                VStack(alignment: .leading, spacing: 0) {
                    // Header with X button
                    HStack {
                        Text(viewModel.name)
                            .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                            .font(PrimaryFont.titleS.font)
                        Spacer()
                        CloseModalButton(action: animateDismissal)
                    }
                    .padding([.leading], Spacing.large)
                    .padding([.trailing], Spacing.medium)
                    .padding(.vertical, Spacing.medium)
                    Divider()
                        .padding(.horizontal, Spacing.large)
                    // Content
                    VStack(alignment: .leading, spacing: Spacing.extraSmall) {
                        Localizable.KeyDetails.Root.Label.publicKey.text
                            .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
                            .font(PrimaryFont.bodyL.font)
                        Text(viewModel.publicKey)
                            .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                            .font(PrimaryFont.bodyL.font)
                    }
                    .padding(.horizontal, Spacing.large)
                    .padding(.vertical, Spacing.small)
                }
                .padding(.bottom, Spacing.medium)
                .padding(.top, -Spacing.medium)
            }
        )
    }

    private func animateDismissal() {
        Animations.chainAnimation(
            animateBackground.toggle(),
            delayedAnimationClosure: { isPresented = false }()
        )
    }
}

extension RootKeyDetailsModal {
    struct ViewModel {
        let name: String
        let publicKey: String
    }
}

#if DEBUG
    struct RootKeyDetailsModal_Previews: PreviewProvider {
        static var previews: some View {
            RootKeyDetailsModal(
                isPresented: .constant(true),
                viewModel: .init(name: "Parity", publicKey: "5CfLC887VYVLN6gG5rmp6wyUoXQYVQxEwNekdCbUUphnnQgW")
            )
            .previewLayout(.sizeThatFits)
            .preferredColorScheme(.dark)
        }
    }
#endif

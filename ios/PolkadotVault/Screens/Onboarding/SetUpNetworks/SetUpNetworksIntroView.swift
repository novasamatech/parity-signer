//
//  SetUpNetworksIntroView.swift
//  PolkadotVault
//
//  Created by Krzysztof Rodak on 14/03/2023.
//

import SwiftUI

struct SetUpNetworksIntroView: View {
    @StateObject var viewModel: ViewModel

    var body: some View {
        VStack(alignment: .center, spacing: 0) {
            // Header text
            Localizable.Onboarding.SetUpNetworks.Label.title.text
                .font(PrimaryFont.titleL.font)
                .foregroundColor(.textAndIconsPrimary)
                .multilineTextAlignment(.center)
                .padding(.top, Spacing.extraLarge)
                .padding(.horizontal, Spacing.extraLarge)
                .padding(.bottom, Spacing.medium)
            Localizable.Onboarding.SetUpNetworks.Label.content.text
                .font(PrimaryFont.bodyM.font)
                .foregroundColor(.textAndIconsTertiary)
                .multilineTextAlignment(.center)
                .padding(.horizontal, Spacing.large)
            Spacer()
            // Networks preview
            Image(.networkPreviewIcons)
            Spacer()
            ActionButton(
                action: viewModel.onSetUpTap,
                text: Localizable.Onboarding.SetUpNetworks.Action.setUp.key,
                style: .primary()
            )
            .padding(.horizontal, Spacing.large)
            .padding(.bottom, Spacing.extraSmall)
            ActionButton(
                action: viewModel.onLaterTap,
                text: Localizable.Onboarding.SetUpNetworks.Action.later.key,
                style: .secondary()
            )
            .padding(.horizontal, Spacing.large)
            .padding(.bottom, Spacing.large)
        }
        .background(.backgroundSystem)
    }
}

extension SetUpNetworksIntroView {
    final class ViewModel: ObservableObject {
        private let onNextTap: () -> Void
        private let onSkipTap: () -> Void

        init(
            onNextTap: @escaping () -> Void,
            onSkipTap: @escaping () -> Void
        ) {
            self.onNextTap = onNextTap
            self.onSkipTap = onSkipTap
        }

        func onSetUpTap() {
            onNextTap()
        }

        func onLaterTap() {
            onSkipTap()
        }
    }
}

#if DEBUG
    struct SetUpNetworksIntroView_Previews: PreviewProvider {
        static var previews: some View {
            SetUpNetworksIntroView(
                viewModel: .init(onNextTap: {}, onSkipTap: {})
            )
            .preferredColorScheme(.dark)
        }
    }
#endif

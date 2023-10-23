//
//  NoKeySetsView.swift
//  PolkadotVault
//
//  Created by Krzysztof Rodak on 18/09/2023.
//

import SwiftUI

struct NoKeySetsView: View {
    @StateObject var viewModel: ViewModel

    var body: some View {
        VStack {
            Spacer()
            VStack(alignment: .center, spacing: Spacing.medium) {
                Image(.logo)
                Localizable.NoKeySets.Label.header.text
                    .font(PrimaryFont.titleXL.font)
                    .foregroundColor(.textAndIconsPrimary)
                    .multilineTextAlignment(.center)
                Localizable.NoKeySets.Label.subheader.text
                    .font(PrimaryFont.bodyL.font)
                    .foregroundColor(.textAndIconsSecondary)
                    .multilineTextAlignment(.center)
            }
            .padding(.horizontal, Spacing.extraLarge)
            .padding(.bottom, Spacing.flexibleComponentSpacer)
            VStack(alignment: .leading, spacing: Spacing.extraSmall) {
                PrimaryButton(
                    action: viewModel.onAddTap,
                    text: Localizable.NoKeySets.Action.add.key,
                    style: .primary()
                )
                PrimaryButton(
                    action: viewModel.onRecoverTap,
                    text: Localizable.NoKeySets.Action.recover.key,
                    style: .secondary()
                )
            }
            .padding(.horizontal, Spacing.large)
            Spacer()
        }
        .fullScreenModal(
            isPresented: $viewModel.isPresentingAddKeySet
        ) {
            EnterKeySetNameView(
                viewModel: .init(
                    isPresented: $viewModel.isPresentingAddKeySet,
                    onCompletion: viewModel.onKeySetAddCompletion(_:)
                )
            )
        }
        .fullScreenModal(
            isPresented: $viewModel.isPresentingRecoverKeySet
        ) {
            RecoverKeySetNameView(
                viewModel: .init(
                    isPresented: $viewModel.isPresentingRecoverKeySet,
                    onCompletion: viewModel.onKeySetAddCompletion(_:)
                )
            )
        }
    }
}

extension NoKeySetsView {
    final class ViewModel: ObservableObject {
        @Published var isPresentingRecoverKeySet = false
        @Published var isPresentingAddKeySet = false
        private let onCompletion: (CreateKeysForNetworksView.OnCompletionAction) -> Void

        init(onCompletion: @escaping (CreateKeysForNetworksView.OnCompletionAction) -> Void) {
            self.onCompletion = onCompletion
        }

        func onKeySetAddCompletion(_ completionAction: CreateKeysForNetworksView.OnCompletionAction) {
            onCompletion(completionAction)
        }

        func onAddTap() {
            isPresentingAddKeySet = true
        }

        func onRecoverTap() {
            isPresentingRecoverKeySet = true
        }
    }
}

#if DEBUG
    struct NoKeySetsView_Previews: PreviewProvider {
        static var previews: some View {
            NoKeySetsView(
                viewModel: .init(onCompletion: { _ in })
            )
        }
    }
#endif

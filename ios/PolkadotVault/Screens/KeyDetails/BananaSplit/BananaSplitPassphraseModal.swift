//
//  BananaSplitPassphraseModal.swift
//  PolkadotVault
//
//  Created by Krzysztof Rodak on 26/02/2024.
//

import SwiftUI

struct BananaSplitPassphraseModal: View {
    @StateObject var viewModel: ViewModel

    var body: some View {
        FullScreenRoundedModal(
            backgroundTapAction: { viewModel.dismissActionSheet() },
            animateBackground: $viewModel.animateBackground,
            content: {
                VStack(alignment: .leading, spacing: Spacing.medium) {
                    HStack {
                        Localizable.BananaSplitPassphraseModal.Label.header.text
                            .foregroundColor(.textAndIconsPrimary)
                            .font(PrimaryFont.titleS.font)
                        Spacer()
                        CloseModalButton(action: viewModel.dismissActionSheet)
                    }
                    Text(viewModel.passphrase)
                        .multilineTextAlignment(.leading)
                        .padding(.vertical, Spacing.medium)
                }
                .padding(.leading, Spacing.large)
                .padding(.trailing, Spacing.medium)
                .padding(.top, Spacing.small)
                .padding(.bottom, Spacing.medium)
            }
        )
    }
}

extension BananaSplitPassphraseModal {
    final class ViewModel: ObservableObject {
        @Published var animateBackground: Bool = false
        @Published var passphrase: String = ""
        @Binding var isPresented: Bool
        private let seedName: String
        private let bananaSplitMediator: KeychainBananaSplitAccessAdapting

        init(
            seedName: String,
            isPresented: Binding<Bool>,
            bananaSplitMediator: KeychainBananaSplitAccessAdapting = KeychainBananaSplitAccessAdapter()
        ) {
            _isPresented = isPresented
            self.seedName = seedName
            self.bananaSplitMediator = bananaSplitMediator
            loadPassphrase()
        }

        func dismissActionSheet() {
            animateDismissal()
        }

        func animateDismissal() {
            Animations.chainAnimation(
                animateBackground.toggle(),
                // swiftformat:disable all
                delayedAnimationClosure: self.hide()
            )
        }

        private func hide() {
            isPresented = false
        }

        private func loadPassphrase() {
            switch bananaSplitMediator.retrieveBananaSplitPassphrase(with: seedName) {
            case let .success(passphrase):
                self.passphrase = passphrase.passphrase
            case .failure:
                ()
            }

        }
    }
}

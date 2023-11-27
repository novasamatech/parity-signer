//
//  AddKeysForNetworkModal.swift
//  PolkadotVault
//
//  Created by Krzysztof Rodak on 19/06/2023.
//

import SwiftUI

struct AddKeysForNetworkModal: View {
    @StateObject var viewModel: ViewModel
    @State private var animateBackground: Bool = false

    var body: some View {
        FullScreenRoundedModal(
            backgroundTapAction: { animateDismissal(viewModel.onCancelTap) },
            animateBackground: $animateBackground,
            safeAreaInsetsMode: .full,
            content: {
                VStack(alignment: .center, spacing: 0) {
                    NetworkLogoIcon(networkName: viewModel.networkName, size: Heights.networkLogoInModal)
                        .padding(.bottom, Spacing.large)
                    Text(Localizable.AddKeysForNetworkModal.Label.title(viewModel.networkName.capitalized))
                        .font(PrimaryFont.titleL.font)
                        .padding(.bottom, Spacing.extraExtraLarge)
                        .fixedSize(horizontal: false, vertical: true)
                        .multilineTextAlignment(.center)
                    VStack(alignment: .center, spacing: Spacing.extraSmall) {
                        ActionButton(
                            action: { animateDismissal(viewModel.onCreateTap) },
                            text: Localizable.AddKeysForNetworkModal.Action.accept.key,
                            style: .primary()
                        )
                        ActionButton(
                            action: { animateDismissal(viewModel.onCancelTap) },
                            text: Localizable.AddKeysForNetworkModal.Action.cancel.key,
                            style: .emptyPrimary()
                        )
                    }
                }
                .padding(.horizontal, Spacing.large)
                .padding(.top, Spacing.large)
                .padding(.bottom, Spacing.small)
            }
        )
    }

    private func animateDismissal(_ completion: @escaping () -> Void) {
        Animations.chainAnimation(
            animateBackground.toggle(),
            delayedAnimationClosure: completion()
        )
    }
}

extension AddKeysForNetworkModal {
    enum OnCompletionAction: Equatable {
        case cancel
        case create
    }

    final class ViewModel: ObservableObject {
        let networkName: String
        private let onCompletion: (OnCompletionAction) -> Void

        @Binding var isPresented: Bool

        init(
            networkName: String,
            isPresented: Binding<Bool>,
            onCompletion: @escaping (OnCompletionAction) -> Void
        ) {
            self.networkName = networkName
            self.onCompletion = onCompletion
            _isPresented = isPresented
        }

        func onCreateTap() {
            isPresented = false
            onCompletion(.create)
        }

        func onCancelTap() {
            isPresented = false
            onCompletion(.cancel)
        }
    }
}

#if DEBUG
    struct AddKeysForNetworkModal_Previews: PreviewProvider {
        static var previews: some View {
            Group {
                AddKeysForNetworkModal(
                    viewModel: .init(
                        networkName: "kusama",
                        isPresented: Binding<Bool>.constant(true),
                        onCompletion: { _ in }
                    )
                )
            }
            .previewLayout(.sizeThatFits)
        }
    }
#endif

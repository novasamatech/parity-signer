//
//  RecoverKeySetNameView.swift
//  PolkadotVault
//
//  Created by Krzysztof Rodak on 06/03/2023.
//

import SwiftUI

struct RecoverKeySetNameView: View {
    @StateObject var viewModel: ViewModel
    @FocusState private var nameFocused: Bool

    var body: some View {
        NavigationView {
            VStack(alignment: .leading, spacing: 0) {
                NavigationBarView(
                    viewModel: .init(
                        title: .progress(current: 1, upTo: 3),
                        leftButtons: [.init(
                            type: .xmark,
                            action: viewModel.onBackTap
                        )],
                        rightButtons: [.init(
                            type: .activeAction(
                                Localizable.RecoverSeedName.Action.next.key,
                                .constant(!viewModel.isActionAvailable())
                            ),
                            action: {
                                nameFocused = false
                                viewModel.onNextTap()
                            }
                        )]
                    )
                )
                mainContent()
                NavigationLink(
                    destination:
                    RecoverKeySetSeedPhraseView(
                        viewModel: .init(
                            seedName: viewModel.seedName,
                            isPresented: $viewModel.isPresented,
                            onCompletion: viewModel.onCompletion
                        )
                    )
                    .navigationBarHidden(true),
                    isActive: $viewModel.isPresentingDetails
                ) { EmptyView() }
            }
            .navigationViewStyle(.stack)
            .navigationBarHidden(true)
            .background(.backgroundPrimary)
        }
    }

    @ViewBuilder
    func mainContent() -> some View {
        VStack(alignment: .leading, spacing: 0) {
            Localizable.RecoverSeedName.Label.title.text
                .foregroundColor(.textAndIconsPrimary)
                .font(PrimaryFont.titleL.font)
                .padding(.top, Spacing.extraSmall)
            Localizable.RecoverSeedName.Label.content.text
                .foregroundColor(.textAndIconsPrimary)
                .font(PrimaryFont.bodyL.font)
                .padding(.vertical, Spacing.extraSmall)
            TextField("", text: $viewModel.seedName)
                .submitLabel(.done)
                .primaryTextFieldStyle(
                    Localizable.seedName.string,
                    text: $viewModel.seedName
                )
                .focused($nameFocused)
                .onSubmit {
                    nameFocused = false
                    viewModel.onSubmitTap()
                }
                .onAppear {
                    nameFocused = true
                }
                .padding(.vertical, Spacing.medium)
            Localizable.RecoverSeedName.Label.footer.text
                .foregroundColor(.textAndIconsTertiary)
                .font(PrimaryFont.captionM.font)
            Spacer()
        }
        .padding(.horizontal, Spacing.large)
    }
}

extension RecoverKeySetNameView {
    final class ViewModel: ObservableObject {
        @Published var seedName: String = ""
        private let seedsMediator: SeedsMediating
        let onCompletion: (CreateKeysForNetworksView.OnCompletionAction) -> Void
        @Binding var isPresented: Bool
        @Published var isPresentingDetails: Bool = false

        init(
            seedsMediator: SeedsMediating = ServiceLocator.seedsMediator,
            isPresented: Binding<Bool>,
            onCompletion: @escaping (CreateKeysForNetworksView.OnCompletionAction) -> Void
        ) {
            self.seedsMediator = seedsMediator
            self.onCompletion = onCompletion
            _isPresented = isPresented
        }

        func onBackTap() {
            isPresented = false
        }

        func onNextTap() {
            isPresentingDetails = true
        }

        func isActionAvailable() -> Bool {
            !seedName.isEmpty && !seedsMediator.checkSeedCollision(seedName: seedName)
        }

        func onSubmitTap() {
            guard isActionAvailable() else { return }
            onNextTap()
        }
    }
}

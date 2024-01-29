//
//  EnterKeySetNameView.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 08/02/2023.
//

import SwiftUI

struct EnterKeySetNameView: View {
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
                                Localizable.NewSeed.Name.Action.next.key,
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
                    CreateKeySetSeedPhraseView(
                        viewModel: .init(
                            dataModel: viewModel.detailsContent,
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
            .fullScreenModal(
                isPresented: $viewModel.isPresentingError
            ) {
                ErrorBottomModal(
                    viewModel: viewModel.presentableError,
                    isShowingBottomAlert: $viewModel.isPresentingError
                )
                .clearModalBackground()
            }
        }
    }

    @ViewBuilder
    func mainContent() -> some View {
        VStack(alignment: .leading, spacing: 0) {
            Localizable.NewSeed.Name.Label.title.text
                .foregroundColor(.textAndIconsPrimary)
                .font(PrimaryFont.titleL.font)
                .padding(.top, Spacing.extraSmall)
            Localizable.NewSeed.Name.Label.header.text
                .foregroundColor(.textAndIconsPrimary)
                .font(PrimaryFont.bodyL.font)
                .padding(.vertical, Spacing.extraSmall)
            TextField("", text: $viewModel.seedName)
                .submitLabel(.done)
                .primaryTextFieldStyle(
                    Localizable.NewSeed.Name.Label.placeholder.string,
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
            Localizable.NewSeed.Name.Label.footer.text
                .foregroundColor(.textAndIconsTertiary)
                .font(PrimaryFont.captionM.font)
            Spacer()
        }
        .padding(.horizontal, Spacing.large)
    }
}

extension EnterKeySetNameView {
    final class ViewModel: ObservableObject {
        @Published var seedName: String = ""
        @Published var isPresentingDetails: Bool = false
        @Published var detailsContent: MNewSeedBackup!
        @Published var isPresentingError: Bool = false
        @Published var presentableError: ErrorBottomModalViewModel!
        @Binding var isPresented: Bool
        let onCompletion: (CreateKeysForNetworksView.OnCompletionAction) -> Void

        private let seedsMediator: SeedsMediating
        private let service: CreateKeySetServicing

        init(
            seedsMediator: SeedsMediating = ServiceLocator.seedsMediator,
            service: CreateKeySetServicing = CreateKeySetService(),
            isPresented: Binding<Bool>,
            onCompletion: @escaping (CreateKeysForNetworksView.OnCompletionAction) -> Void
        ) {
            self.seedsMediator = seedsMediator
            self.service = service
            self.onCompletion = onCompletion
            _isPresented = isPresented
        }

        func onBackTap() {
            isPresented = false
        }

        func onNextTap() {
            service.createKeySet(seedName: seedName) { result in
                switch result {
                case let .success(seedBackup):
                    self.detailsContent = seedBackup
                    self.isPresentingDetails = true
                case let .failure(error):
                    self.presentableError = .alertError(message: error.localizedDescription)
                    self.isPresentingError = true
                }
            }
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

#if DEBUG
    struct EnterKeySetNameView_Previews: PreviewProvider {
        static var previews: some View {
            EnterKeySetNameView(
                viewModel: .init(
                    isPresented: .constant(true),
                    onCompletion: { _ in }
                )
            )
            .previewLayout(.sizeThatFits)
        }
    }
#endif

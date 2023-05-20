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
                        title: nil,
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
                            content: viewModel.detailsContent,
                            isPresented: $viewModel.isPresented
                        )
                    )
                    .navigationBarHidden(true),
                    isActive: $viewModel.isPresentingDetails
                ) { EmptyView() }
            }
            .navigationViewStyle(StackNavigationViewStyle())
            .navigationBarHidden(true)
        }
    }

    @ViewBuilder
    func mainContent() -> some View {
        VStack(alignment: .leading, spacing: 0) {
            Localizable.RecoverSeedName.Label.title.text
                .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                .font(PrimaryFont.titleL.font)
                .padding(.top, Spacing.extraSmall)
            Localizable.RecoverSeedName.Label.content.text
                .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
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
                    viewModel.onAppear()
                }
                .padding(.vertical, Spacing.medium)
            Localizable.RecoverSeedName.Label.footer.text
                .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
                .font(PrimaryFont.captionM.font)
            Spacer()
        }
        .padding(.horizontal, Spacing.large)
    }
}

extension RecoverKeySetNameView {
    final class ViewModel: ObservableObject {
        @Published var seedName: String = ""
        private let service: RecoverKeySetService
        private let seedsMediator: SeedsMediating
        @Binding var isPresented: Bool
        @Published var isPresentingDetails: Bool = false
        @Published var detailsContent: MRecoverSeedPhrase!

        init(
            service: RecoverKeySetService = RecoverKeySetService(),
            seedsMediator: SeedsMediating = ServiceLocator.seedsMediator,
            isPresented: Binding<Bool>
        ) {
            self.service = service
            self.seedsMediator = seedsMediator
            _isPresented = isPresented
        }

        func onAppear() {
            seedName = service.recoverKeySetStart(seedsMediator.seedNames.isEmpty).seedName
        }

        func onBackTap() {
            isPresented = false
        }

        func onNextTap() {
            detailsContent = service.continueKeySetRecovery(seedName)
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

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
    @EnvironmentObject var navigation: NavigationCoordinator

    var body: some View {
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
                            self.nameFocused = false
                            viewModel.onNextTap()
                        }
                    )]
                )
            )
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
        .onAppear {
            viewModel.use(navigation: navigation)
        }
    }
}

extension RecoverKeySetNameView {
    final class ViewModel: ObservableObject {
        @Published var seedName: String = ""
        weak var navigation: NavigationCoordinator!
        private let content: MRecoverSeedName

        private let seedsMediator: SeedsMediating

        init(
            content: MRecoverSeedName,
            seedsMediator: SeedsMediating = ServiceLocator.seedsMediator
        ) {
            self.content = content
            self.seedsMediator = seedsMediator
        }

        func use(navigation: NavigationCoordinator) {
            self.navigation = navigation
        }

        func onAppear() {
            seedName = content.seedName
        }

        func onBackTap() {
            navigation.perform(navigation: .init(action: .goBack))
        }

        func onNextTap() {
            navigation.perform(navigation: .init(action: .goForward, details: seedName))
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

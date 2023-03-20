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
            VStack(alignment: .leading, spacing: 0) {
                Localizable.NewSeed.Name.Label.title.text
                    .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                    .font(PrimaryFont.titleL.font)
                    .padding(.top, Spacing.extraSmall)
                Localizable.NewSeed.Name.Label.header.text
                    .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
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
                    .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
                    .font(PrimaryFont.captionM.font)
                Spacer()
            }
            .padding(.horizontal, Spacing.large)
            .onAppear {
                viewModel.use(navigation: navigation)
            }
        }
    }
}

extension EnterKeySetNameView {
    final class ViewModel: ObservableObject {
        @Published var seedName: String = ""
        weak var navigation: NavigationCoordinator!

        private let seedsMediator: SeedsMediating

        init(
            seedsMediator: SeedsMediating = ServiceLocator.seedsMediator
        ) {
            self.seedsMediator = seedsMediator
        }

        func use(navigation: NavigationCoordinator) {
            self.navigation = navigation
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

#if DEBUG
    struct EnterKeySetNameView_Previews: PreviewProvider {
        static var previews: some View {
            EnterKeySetNameView(
                viewModel: .init()
            )
            .environmentObject(NavigationCoordinator())
            .previewLayout(.sizeThatFits)
        }
    }
#endif

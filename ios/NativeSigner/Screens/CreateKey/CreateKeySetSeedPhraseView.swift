//
//  CreateKeySetSeedPhraseView.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 08/02/2023.
//

import SwiftUI

struct CreateKeySetSeedPhraseView: View {
    @StateObject var viewModel: ViewModel

    @EnvironmentObject var navigation: NavigationCoordinator

    var body: some View {
        VStack(spacing: 0) {
            NavigationBarView(
                viewModel: .init(
                    title: nil,
                    leftButtons: [.init(
                        type: .arrow,
                        action: viewModel.onBackTap
                    )]
                )
            )
            ScrollView(showsIndicators: false) {
                VStack(alignment: .center, spacing: 0) {
                    Localizable.NewSeed.Backup.Label.header.text
                        .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                        .font(PrimaryFont.bodyL.font)
                        .multilineTextAlignment(.center)
                        .lineSpacing(Spacing.extraSmall)
                }
                .padding(.top, Spacing.extraSmall)
                .padding(.bottom, Spacing.medium)
                .padding(.horizontal, Spacing.medium)
                VStack(alignment: .leading, spacing: 0) {
                    SeedPhraseView(viewModel: .init(dataModel: .init(seedPhrase: viewModel.dataModel.seedPhrase)))
                        .padding(.bottom, Spacing.extraSmall)
                    AttributedTintInfoBox(text: Localizable.createKeySetSeedPhraseInfo())
                        .padding(.bottom, Spacing.medium)
                    Button(
                        action: {
                            viewModel.confirmBackup.toggle()
                        },
                        label: {
                            HStack {
                                (
                                    viewModel.confirmBackup ? Asset.checkboxChecked.swiftUIImage : Asset.checkboxEmpty
                                        .swiftUIImage
                                )
                                .foregroundColor(Asset.accentPink300.swiftUIColor)
                                Localizable.iHaveWrittenDownMySeedPhrase.text
                                    .multilineTextAlignment(.leading)
                                    .foregroundColor(Asset.textAndIconsSecondary.swiftUIColor)
                                Spacer()
                            }
                        }
                    )
                    .padding(.vertical, Spacing.small)
                    Spacer()
                    PrimaryButton(
                        action: viewModel.onCreateTap,
                        text: Localizable.NewSeed.Backup.Action.create.key,
                        style: .primary(isDisabled: .constant(!viewModel.confirmBackup))
                    )
                    .padding(.vertical, Spacing.medium)
                }
                .padding(.horizontal, Spacing.medium)
            }
        }
        .background(Asset.backgroundSecondary.swiftUIColor)
        .onAppear {
            viewModel.use(navigation: navigation)
        }
    }
}

extension CreateKeySetSeedPhraseView {
    final class ViewModel: ObservableObject {
        private let seedsMediator: SeedsMediating
        private weak var navigation: NavigationCoordinator!

        let dataModel: MNewSeedBackup
        @Published var confirmBackup = false

        init(
            dataModel: MNewSeedBackup,
            seedsMediator: SeedsMediating = ServiceLocator.seedsMediator
        ) {
            self.dataModel = dataModel
            self.seedsMediator = seedsMediator
        }

        func use(navigation: NavigationCoordinator) {
            self.navigation = navigation
        }

        func onBackTap() {
            navigation.perform(navigation: .init(action: .goBack))
        }

        func onCreateTap() {
            seedsMediator.restoreSeed(
                seedName: dataModel.seed,
                seedPhrase: dataModel.seedPhrase,
                navigate: true
            )
        }
    }
}

#if DEBUG
    struct CreateKeySetSeedPhraseView_Previews: PreviewProvider {
        static var previews: some View {
            CreateKeySetSeedPhraseView(
                viewModel: .init(
                    dataModel: .init(
                        seed: "Key Set Name",
                        seedPhrase: """
                        awesome change room lottery song useless elephant dry educate type debate
                        season give exact gift push bid rich atom system pig put welcome exit
                        """,
                        identicon: .svg(image: PreviewData.exampleIdenticon)
                    )
                )
            )
            .environmentObject(NavigationCoordinator())
        }
    }
#endif

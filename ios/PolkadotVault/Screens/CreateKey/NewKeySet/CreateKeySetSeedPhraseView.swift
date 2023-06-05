//
//  CreateKeySetSeedPhraseView.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 08/02/2023.
//

import SwiftUI

struct CreateKeySetSeedPhraseView: View {
    @StateObject var viewModel: ViewModel
    @Environment(\.presentationMode) var mode: Binding<PresentationMode>

    var body: some View {
        VStack(spacing: 0) {
            NavigationBarView(
                viewModel: .init(
                    title: nil,
                    leftButtons: [.init(type: .arrow, action: { mode.wrappedValue.dismiss() })]
                )
            )
            ScrollView(showsIndicators: false) {
                VStack(alignment: .leading, spacing: 0) {
                    Localizable.NewSeed.Backup.Label.header.text
                        .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                        .font(PrimaryFont.titleM.font)
                        .multilineTextAlignment(.leading)
                        .lineSpacing(Spacing.extraSmall)
                    HStack {
                        Spacer()
                    }
                }
                .padding(.top, Spacing.extraExtraSmall)
                .padding(.bottom, Spacing.medium)
                .padding(.horizontal, Spacing.large)
                VStack(alignment: .leading, spacing: 0) {
                    SeedPhraseView(viewModel: .init(dataModel: .init(seedPhrase: viewModel.dataModel.seedPhrase)))
                        .padding(.bottom, Spacing.extraSmall)
                        .padding(.horizontal, Spacing.medium)
                    AttributedTintInfoBox(text: Localizable.createKeySetSeedPhraseInfo())
                        .padding(.horizontal, Spacing.medium)
                        .padding(.bottom, Spacing.large)
                        .contentShape(Rectangle())
                        .onTapGesture { viewModel.onInfoBoxTap() }
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
                                Localizable.NewSeed.Backup.Label.confirmation.text
                                    .multilineTextAlignment(.leading)
                                    .foregroundColor(Asset.textAndIconsSecondary.swiftUIColor)
                                Spacer()
                            }
                        }
                    )
                    .padding(.horizontal, Spacing.large)
                    .padding(.bottom, Spacing.extraSmall)
                    Spacer()
                    PrimaryButton(
                        action: viewModel.onCreateTap,
                        text: Localizable.NewSeed.Backup.Action.create.key,
                        style: .primary(isDisabled: .constant(!viewModel.confirmBackup))
                    )
                    .padding(Spacing.large)
                }
            }
            NavigationLink(
                destination:
                CreateKeysForNetworksView(
                    viewModel: viewModel.createDerivedKeys()
                )
                .navigationBarHidden(true),
                isActive: $viewModel.isPresentingDetails
            ) { EmptyView() }
        }
        .background(Asset.backgroundPrimary.swiftUIColor)
        .fullScreenModal(
            isPresented: $viewModel.isPresentingInfo
        ) {
            ErrorBottomModal(
                viewModel: viewModel.presentableInfo,
                isShowingBottomAlert: $viewModel.isPresentingInfo
            )
            .clearModalBackground()
        }
    }
}

extension CreateKeySetSeedPhraseView {
    final class ViewModel: ObservableObject {
        private let seedsMediator: SeedsMediating

        let dataModel: MNewSeedBackup
        @Binding var isPresented: Bool
        @Published var isPresentingDetails: Bool = false
        @Published var confirmBackup = false
        @Published var isPresentingInfo: Bool = false
        @Published var presentableInfo: ErrorBottomModalViewModel = .bananaSplitExplanation()
        private let service: CreateKeySetService

        init(
            dataModel: MNewSeedBackup,
            isPresented: Binding<Bool>,
            service: CreateKeySetService = CreateKeySetService(),
            seedsMediator: SeedsMediating = ServiceLocator.seedsMediator
        ) {
            self.dataModel = dataModel
            self.service = service
            self.seedsMediator = seedsMediator
            _isPresented = isPresented
        }

        func onCreateTap() {
            seedsMediator.createSeed(
                seedName: dataModel.seed,
                seedPhrase: dataModel.seedPhrase,
                shouldCheckForCollision: true
            )
            service.confirmKeySetCreation(dataModel.seedPhrase)
            isPresentingDetails = true
        }

        func onInfoBoxTap() {
            isPresentingInfo = true
        }

        func createDerivedKeys() -> CreateKeysForNetworksView.ViewModel {
            .init(seedName: dataModel.seed, isPresented: $isPresented)
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
                        identicon: .stubIdenticon
                    ),
                    isPresented: .constant(true)
                )
            )
        }
    }
#endif

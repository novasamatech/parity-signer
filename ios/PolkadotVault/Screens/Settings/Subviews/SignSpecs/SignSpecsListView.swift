//
//  SignSpecsListView.swift
//  PolkadotVault
//
//  Created by Krzysztof Rodak on 22/03/2023.
//

import SwiftUI

struct SignSpecsListView: View {
    @StateObject var viewModel: ViewModel
    @EnvironmentObject var navigation: NavigationCoordinator

    var body: some View {
        VStack {
            NavigationBarView(
                viewModel: NavigationBarViewModel(
                    title: Localizable.SignSpecsList.Label.title.string,
                    leftButtons: [.init(type: .arrow, action: viewModel.onBackTap)],
                    backgroundColor: Asset.backgroundPrimary.swiftUIColor
                )
            )
            ScrollView {
                LazyVStack {
                    ForEach(viewModel.content.identities, id: \.addressKey) { keyRecord in
                        rawKeyRow(keyRecord)
                            .contentShape(Rectangle())
                            .onTapGesture {
                                viewModel.onRecordTap(keyRecord)
                            }
                    }
                }
            }
        }
        .background(Asset.backgroundPrimary.swiftUIColor)
        .onAppear {
            viewModel.use(navigation: navigation)
        }
        .fullScreenCover(
            isPresented: $viewModel.isPresentingEnterPassword,
            onDismiss: {
                viewModel.enterPassword = nil
                if viewModel.detailsContent != nil {
                    viewModel.isPresentingDetails = true
                    return
                }
                if viewModel.shouldPresentError {
                    viewModel.shouldPresentError = false
                    viewModel.isPresentingError = true
                } else {
                    navigation.perform(navigation: .init(action: .goBack))
                }
            }
        ) {
            SignSpecEnterPasswordModal(
                viewModel: .init(
                    isPresented: $viewModel.isPresentingEnterPassword,
                    shouldPresentError: $viewModel.shouldPresentError,
                    dataModel: $viewModel.enterPassword,
                    detailsContent: $viewModel.detailsContent
                )
            )
            .clearModalBackground()
        }
        .fullScreenCover(
            isPresented: $viewModel.isPresentingDetails,
            onDismiss: {
                viewModel.detailsContent = nil
            }
        ) {
            SignSpecDetails(
                viewModel: .init(
                    content: viewModel.detailsContent,
                    isPresented: $viewModel.isPresentingDetails
                )
            )
        }
        .fullScreenCover(
            isPresented: $viewModel.isPresentingError,
            onDismiss: {
                navigation.perform(navigation: .init(action: .navbarSettings))
            }
        ) {
            ErrorBottomModal(
                viewModel: viewModel.presentableError,
                isShowingBottomAlert: $viewModel.isPresentingError
            )
            .clearModalBackground()
        }
    }

    @ViewBuilder
    func rawKeyRow(_ rawKey: MRawKey) -> some View {
        HStack(alignment: .center, spacing: Spacing.small) {
            NetworkIdenticon(
                identicon: rawKey.address.identicon,
                network: rawKey.networkLogo,
                background: Asset.backgroundPrimary.swiftUIColor,
                size: Sizes.signSpecsIdenticonSize
            )
            VStack(alignment: .leading, spacing: Spacing.extraExtraSmall) {
                if !rawKey.address.displayablePath.isEmpty {
                    Text(rawKey.address.displayablePath)
                        .font(PrimaryFont.captionM.font)
                        .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
                }
                Text(rawKey.publicKey.truncateMiddle())
                    .font(PrimaryFont.bodyL.font)
                    .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                Text(rawKey.address.seedName)
                    .font(PrimaryFont.bodyM.font)
                    .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
            }
            Spacer()
            Asset.chevronRight.swiftUIImage
                .resizable()
                .aspectRatio(contentMode: .fit)
                .frame(height: Heights.chevronRightInList)
                .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
                .padding(Spacing.small)
        }
        .frame(height: Heights.signSpecsListRowHeight)
        .padding(.horizontal, Spacing.medium)
    }
}

extension SignSpecsListView {
    final class ViewModel: ObservableObject {
        let content: MSignSufficientCrypto
        private let seedsMediator: SeedsMediating
        private weak var navigation: NavigationCoordinator!

        @Published var detailsContent: MSufficientCryptoReady!
        @Published var isPresentingDetails: Bool = false
        @Published var isPresentingEnterPassword: Bool = false
        @Published var shouldPresentError: Bool = false
        @Published var isPresentingError: Bool = false
        @Published var enterPassword: MEnterPassword!
        @Published var presentableError: ErrorBottomModalViewModel = .signingForgotPassword()

        init(
            content: MSignSufficientCrypto,
            seedsMediator: SeedsMediating = ServiceLocator.seedsMediator
        ) {
            self.content = content
            self.seedsMediator = seedsMediator
        }

        func use(navigation: NavigationCoordinator) {
            self.navigation = navigation
        }

        func onBackTap() {
            navigation.perform(navigation: .init(action: .goBack))
        }

        func onRecordTap(_ keyRecord: MRawKey) {
            let seedPhrase = seedsMediator.getSeed(seedName: keyRecord.address.seedName)
            guard !seedPhrase.isEmpty else { return }
            let actionResult = navigation.performFake(
                navigation: .init(
                    action: .goForward,
                    details: keyRecord.addressKey,
                    seedPhrase: seedPhrase
                )
            )
            switch actionResult.modalData {
            case let .enterPassword(enterPassword):
                self.enterPassword = enterPassword
                isPresentingEnterPassword = true
            case let .sufficientCryptoReady(detailsContent):
                self.detailsContent = detailsContent
                isPresentingDetails = true
            default:
                ()
            }
        }
    }
}

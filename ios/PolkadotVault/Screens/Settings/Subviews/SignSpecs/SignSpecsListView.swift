//
//  SignSpecsListView.swift
//  PolkadotVault
//
//  Created by Krzysztof Rodak on 22/03/2023.
//

import Combine
import SwiftUI

struct SignSpecsListView: View {
    @StateObject var viewModel: ViewModel
    @Environment(\.presentationMode) var presentationMode

    var body: some View {
        VStack {
            NavigationBarView(
                viewModel: NavigationBarViewModel(
                    title: Localizable.SignSpecsList.Label.title.string,
                    leftButtons: [.init(
                        type: .arrow,
                        action: {
                            presentationMode.wrappedValue.dismiss()
                        }
                    )],
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
            NavigationLink(
                destination: SignSpecDetails(viewModel: .init(
                    content: viewModel.detailsContent,
                    onComplete: viewModel.onDetailsCompletion
                ))
                .navigationBarHidden(true),
                isActive: $viewModel.isPresentingDetails
            ) { EmptyView() }
        }
        .onReceive(viewModel.dismissViewRequest) { _ in
            presentationMode.wrappedValue.dismiss()
        }
        .background(Asset.backgroundPrimary.swiftUIColor)
        .fullScreenCover(
            isPresented: $viewModel.isPresentingEnterPassword,
            onDismiss: viewModel.onPasswordModalDismiss
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
            isPresented: $viewModel.isPresentingError
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
        private let networkKey: String
        let content: MSignSufficientCrypto
        private let seedsMediator: SeedsMediating
        private let navigation: NavigationCoordinator
        private let manageDetailsService: ManageNetworkDetailsService
        @Published var detailsContent: MSufficientCryptoReady!
        @Published var isPresentingDetails: Bool = false
        @Published var isPresentingEnterPassword: Bool = false
        @Published var shouldPresentError: Bool = false
        @Published var isPresentingError: Bool = false
        @Published var enterPassword: MEnterPassword!
        @Published var presentableError: ErrorBottomModalViewModel = .signingForgotPassword()
        var dismissViewRequest: AnyPublisher<Void, Never> {
            dismissRequest.eraseToAnyPublisher()
        }

        private let dismissRequest = PassthroughSubject<Void, Never>()

        init(
            networkKey: String,
            content: MSignSufficientCrypto,
            seedsMediator: SeedsMediating = ServiceLocator.seedsMediator,
            manageDetailsService: ManageNetworkDetailsService = ManageNetworkDetailsService(),
            navigation: NavigationCoordinator = NavigationCoordinator()
        ) {
            self.networkKey = networkKey
            self.content = content
            self.seedsMediator = seedsMediator
            self.manageDetailsService = manageDetailsService
            self.navigation = navigation
        }

        func onDetailsCompletion() {
            manageDetailsService.signSpecList(networkKey)
        }

        func onPasswordModalDismiss() {
            enterPassword = nil
            if detailsContent != nil {
                isPresentingDetails = true
                return
            } else {
                manageDetailsService.signSpecList(networkKey)
            }
            if shouldPresentError {
                shouldPresentError = false
                isPresentingError = true
                manageDetailsService.signSpecList(networkKey)
            }
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

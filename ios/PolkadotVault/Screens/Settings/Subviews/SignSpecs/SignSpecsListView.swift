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
                    title: .title(Localizable.SignSpecsList.Label.title.string),
                    leftButtons: [.init(
                        type: .arrow,
                        action: {
                            presentationMode.wrappedValue.dismiss()
                        }
                    )],
                    backgroundColor: .backgroundPrimary
                )
            )
            if let content = viewModel.content {
                ScrollView {
                    LazyVStack {
                        ForEach(content.identities, id: \.addressKey) { keyRecord in
                            rawKeyRow(keyRecord)
                                .contentShape(Rectangle())
                                .onTapGesture {
                                    viewModel.onRecordTap(keyRecord)
                                }
                        }
                    }
                }
            }
            NavigationLink(
                destination: SignSpecDetails(
                    viewModel: .init(
                        content: viewModel.detailsContent,
                        type: viewModel.type
                    )
                )
                .navigationBarHidden(true),
                isActive: $viewModel.isPresentingDetails
            ) { EmptyView() }
        }
        .onAppear { viewModel.onAppear() }
        .onReceive(viewModel.dismissViewRequest) { _ in
            presentationMode.wrappedValue.dismiss()
        }
        .background(.backgroundPrimary)
        .fullScreenModal(
            isPresented: $viewModel.isPresentingEnterPassword
        ) {
            SignSpecEnterPasswordModal(
                viewModel: .init(
                    isPresented: $viewModel.isPresentingEnterPassword,
                    selectedKeyRecord: viewModel.selectedKeyRecord,
                    onDoneTapAction: viewModel.onPasswordModalDoneTapAction(_:)
                )
            )
            .clearModalBackground()
        }
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

    @ViewBuilder
    func rawKeyRow(_ rawKey: MRawKey) -> some View {
        HStack(alignment: .center, spacing: Spacing.small) {
            NetworkIdenticon(
                identicon: rawKey.address.identicon,
                network: rawKey.networkLogo,
                background: .backgroundPrimary,
                size: Sizes.signSpecsIdenticonSize
            )
            VStack(alignment: .leading, spacing: Spacing.extraExtraSmall) {
                if !rawKey.address.displayablePath.isEmpty {
                    Text(rawKey.address.displayablePath)
                        .font(PrimaryFont.captionM.font)
                        .foregroundColor(.textAndIconsTertiary)
                }
                Text(rawKey.publicKey.truncateMiddle())
                    .font(PrimaryFont.bodyL.font)
                    .foregroundColor(.textAndIconsPrimary)
                Text(rawKey.address.seedName)
                    .font(PrimaryFont.bodyM.font)
                    .foregroundColor(.textAndIconsTertiary)
            }
            Spacer()
            Image(.chevronRight)
                .resizable()
                .aspectRatio(contentMode: .fit)
                .frame(height: Heights.chevronRightInList)
                .foregroundColor(.textAndIconsTertiary)
                .padding(Spacing.small)
        }
        .frame(height: Heights.signSpecsListRowHeight)
        .padding(.horizontal, Spacing.medium)
    }
}

extension SignSpecsListView {
    final class ViewModel: ObservableObject {
        private let networkKey: String
        private let seedsMediator: SeedsMediating
        private let service: ManageNetworkDetailsService
        let type: SpecSignType
        @Published var detailsContent: MSufficientCryptoReady!
        @Published var content: MSignSufficientCrypto?
        @Published var selectedKeyRecord: MRawKey!
        @Published var isPresentingDetails: Bool = false
        @Published var isPresentingEnterPassword: Bool = false
        @Published var shouldPresentError: Bool = false
        @Published var isPresentingError: Bool = false
        @Published var presentableError: ErrorBottomModalViewModel = .signingForgotPassword()
        var dismissViewRequest: AnyPublisher<Void, Never> {
            dismissRequest.eraseToAnyPublisher()
        }

        private let dismissRequest = PassthroughSubject<Void, Never>()

        init(
            networkKey: String,
            type: SpecSignType,
            seedsMediator: SeedsMediating = ServiceLocator.seedsMediator,
            service: ManageNetworkDetailsService = ManageNetworkDetailsService()
        ) {
            self.networkKey = networkKey
            self.type = type
            self.seedsMediator = seedsMediator
            self.service = service
        }

        func onAppear() {
            loadData()
        }

        func onRecordTap(_ keyRecord: MRawKey) {
            if keyRecord.address.hasPwd {
                selectedKeyRecord = keyRecord
                isPresentingEnterPassword = true
            } else {
                attemptSigning(keyRecord)
            }
        }

        func onPasswordModalDoneTapAction(_ modal: SignSpecEnterPasswordModal.ViewModel) {
            let seedPhrase = seedsMediator.getSeed(seedName: modal.selectedKeyRecord.address.seedName)
            guard !seedPhrase.isEmpty else { return }
            service.signSpec(
                type,
                networkKey,
                signingAddressKey: modal.selectedKeyRecord.addressKey,
                seedPhrase: seedPhrase,
                password: modal.password
            ) { [weak self] result in
                guard let self else { return }
                switch result {
                case let .success(detailsContent):
                    isPresentingEnterPassword = false
                    self.detailsContent = detailsContent
                    isPresentingDetails = true
                case let .failure(error):
                    switch error {
                    case .wrongPassword:
                        modal.isValid = false
                    case let .error(serviceError):
                        isPresentingEnterPassword = false
                        presentableError = .alertError(message: serviceError.localizedDescription)
                        isPresentingError = true
                    }
                }
            }
        }
    }
}

private extension SignSpecsListView.ViewModel {
    func loadData() {
        service.signSpecList { [weak self] result in
            guard let self else { return }
            switch result {
            case let .success(content):
                self.content = content
            case let .failure(error):
                presentableError = .alertError(message: error.localizedDescription)
                isPresentingError = true
            }
        }
    }

    func attemptSigning(_ keyRecord: MRawKey) {
        let seedPhrase = seedsMediator.getSeed(seedName: keyRecord.address.seedName)
        guard !seedPhrase.isEmpty else { return }
        service.signSpec(
            type,
            networkKey,
            signingAddressKey: keyRecord.addressKey,
            seedPhrase: seedPhrase,
            password: nil
        ) { [weak self] result in
            guard let self else { return }
            switch result {
            case let .success(detailsContent):
                self.detailsContent = detailsContent
                isPresentingDetails = true
            case let .failure(error):
                switch error {
                case .wrongPassword:
                    ()
                case let .error(serviceError):
                    presentableError = .alertError(message: serviceError.localizedDescription)
                    isPresentingError = true
                }
            }
        }
    }
}

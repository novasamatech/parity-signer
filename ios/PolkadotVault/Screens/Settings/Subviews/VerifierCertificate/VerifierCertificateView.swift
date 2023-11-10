//
//  VerifierCertificateView.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 12/12/2022.
//

import Combine
import SwiftUI

struct VerifierCertificateView: View {
    @StateObject var viewModel: ViewModel
    @Environment(\.presentationMode) var presentationMode

    var body: some View {
        VStack(alignment: .leading, spacing: 0) {
            NavigationBarView(
                viewModel: NavigationBarViewModel(
                    title: .title(Localizable.VerifierCertificate.Label.title.string),
                    leftButtons: [.init(
                        type: .arrow,
                        action: {
                            presentationMode.wrappedValue.dismiss()
                        }
                    )],
                    rightButtons: [.init(type: .empty)]
                )
            )
            if let content = viewModel.content {
                VStack {
                    VStack(spacing: Spacing.small) {
                        VStack(alignment: .leading, spacing: Spacing.extraSmall) {
                            Localizable.Transaction.Verifier.Label.key.text
                                .foregroundColor(.textAndIconsTertiary)
                            Text(content.publicKey)
                                .foregroundColor(.textAndIconsPrimary)
                        }
                        Divider()
                        VStack(alignment: .leading) {
                            HStack {
                                Localizable.Transaction.Verifier.Label.crypto.text
                                    .foregroundColor(.textAndIconsTertiary)
                                Spacer()
                                Text(content.encryption)
                                    .foregroundColor(.textAndIconsPrimary)
                            }
                        }
                    }
                    .padding(Spacing.medium)
                }
                .containerBackground()
                .padding(.bottom, Spacing.extraSmall)
                .padding(.horizontal, Spacing.extraSmall)
                HStack {
                    Text(Localizable.VerifierCertificate.Action.remove.string)
                        .font(PrimaryFont.titleS.font)
                        .foregroundColor(.accentRed400)
                    Spacer()
                }
                .contentShape(Rectangle())
                .onTapGesture {
                    viewModel.onRemoveTap()
                }
                .frame(height: Heights.verifierCertificateActionHeight)
                .padding(.horizontal, Spacing.large)
            }
            Spacer()
        }.onReceive(viewModel.dismissViewRequest) { _ in
            presentationMode.wrappedValue.dismiss()
        }
        .background(.backgroundPrimary)
        .fullScreenModal(isPresented: $viewModel.isPresentingRemoveConfirmation) {
            VerticalActionsBottomModal(
                viewModel: .removeGeneralVerifier,
                mainAction: viewModel.onRemoveConfirmationTap(),
                isShowingBottomAlert: $viewModel.isPresentingRemoveConfirmation
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
}

extension VerifierCertificateView {
    final class ViewModel: ObservableObject {
        @Published var isPresentingRemoveConfirmation = false
        @Published var content: MVerifierDetails?
        @Published var isPresentingError: Bool = false
        @Published var presentableError: ErrorBottomModalViewModel = .alertError(message: "")
        private let onboardingMediator: OnboardingMediator
        private let service: GeneralVerifierService
        var dismissViewRequest: AnyPublisher<Void, Never> {
            dismissRequest.eraseToAnyPublisher()
        }

        private let dismissRequest = PassthroughSubject<Void, Never>()

        init(
            onboardingMediator: OnboardingMediator = ServiceLocator.onboardingMediator,
            service: GeneralVerifierService = GeneralVerifierService()
        ) {
            self.onboardingMediator = onboardingMediator
            self.service = service
            loadData()
        }

        func onRemoveTap() {
            isPresentingRemoveConfirmation = true
        }

        func onRemoveConfirmationTap() {
            onboardingMediator.onboard(verifierRemoved: true)
            isPresentingRemoveConfirmation = false
            dismissRequest.send()
        }

        func loadData() {
            service.getGeneralVerifier { result in
                switch result {
                case let .success(content):
                    self.content = content
                case let .failure(error):
                    self.presentableError = .alertError(message: error.localizedDescription)
                    self.isPresentingError = true
                }
            }
        }
    }
}

#if DEBUG
    struct VerfierCertificateView_Previews: PreviewProvider {
        static var previews: some View {
            VerifierCertificateView(viewModel: .init())
        }
    }
#endif

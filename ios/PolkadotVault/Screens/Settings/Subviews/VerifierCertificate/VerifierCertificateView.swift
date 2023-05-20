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
    @EnvironmentObject private var appState: AppState
    @Environment(\.presentationMode) var presentationMode

    var body: some View {
        VStack(alignment: .leading, spacing: 0) {
            NavigationBarView(
                viewModel: NavigationBarViewModel(
                    title: Localizable.VerifierCertificate.Label.title.string,
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
                                .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
                            Text(content.publicKey)
                                .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                        }
                        Divider()
                        VStack(alignment: .leading) {
                            HStack {
                                Localizable.Transaction.Verifier.Label.crypto.text
                                    .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
                                Spacer()
                                Text(content.encryption)
                                    .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
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
                        .foregroundColor(Asset.accentRed400.swiftUIColor)
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
        .background(Asset.backgroundPrimary.swiftUIColor)
        .fullScreenModal(isPresented: $viewModel.isPresentingRemoveConfirmation) {
            VerticalActionsBottomModal(
                viewModel: .removeGeneralVerifier,
                mainAction: viewModel.onRemoveConfirmationTap(),
                isShowingBottomAlert: $viewModel.isPresentingRemoveConfirmation
            )
            .clearModalBackground()
        }
    }
}

extension VerifierCertificateView {
    final class ViewModel: ObservableObject {
        @Published var isPresentingRemoveConfirmation = false
        @Published var content: MVerifierDetails?

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
            content = service.getGeneralVerifier()
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

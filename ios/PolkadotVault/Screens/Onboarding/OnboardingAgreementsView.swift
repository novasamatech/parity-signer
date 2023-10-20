//
//  OnboardingAgreementsView.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 24/01/2023.
//

import SwiftUI

struct OnboardingAgreementsView: View {
    @StateObject var viewModel: ViewModel

    var body: some View {
        VStack(alignment: .leading, spacing: 0) {
            Localizable.Onboarding.Agreements.Label.title.text
                .font(PrimaryFont.titleXL.font)
                .foregroundColor(.textAndIconsPrimary)
                .padding(.top, Spacing.extraExtraLarge)
                .padding(.bottom, Spacing.medium)
                .padding(.horizontal, Spacing.large)
            VStack(spacing: 0) {
                HStack(spacing: 0) {
                    Localizable.Onboarding.Agreements.Label.privacyCell.text
                        .padding(.leading, Spacing.medium)
                    Spacer()
                    Image(.chevronRight)
                        .foregroundColor(.textAndIconsTertiary)
                        .padding(.trailing, Spacing.medium)
                }
                .frame(height: Heights.onboardingAgreementRecord)
                .contentShape(Rectangle())
                .onTapGesture {
                    viewModel.onPrivacyPolicyTap()
                }
                Divider()
                    .padding(.horizontal, Spacing.medium)
                HStack(spacing: 0) {
                    Localizable.Onboarding.Agreements.Label.toSCell.text
                        .padding(.leading, Spacing.medium)
                    Spacer()
                    Image(.chevronRight)
                        .foregroundColor(.textAndIconsTertiary)
                        .padding(.trailing, Spacing.medium)
                }
                .frame(height: Heights.onboardingAgreementRecord)
                .contentShape(Rectangle())
                .onTapGesture {
                    viewModel.onTermsOfServiceTap()
                }
            }
            .foregroundColor(.textAndIconsPrimary)
            .font(PrimaryFont.titleS.font)
            .containerBackground(CornerRadius.small)
            .padding(Spacing.medium)
            Spacer()
            HStack(spacing: Spacing.small) {
                if viewModel.isCheckboxSelected {
                    Image(.checkboxChecked)
                        .foregroundColor(.accentPink300)
                } else {
                    Image(.checkboxEmpty)
                        .foregroundColor(.textAndIconsTertiary)
                }
                Localizable.Onboarding.Agreements.Label.confirmation.text
                    .foregroundColor(.textAndIconsPrimary)
                    .font(PrimaryFont.bodyL.font)
            }
            .contentShape(Rectangle())
            .onTapGesture {
                viewModel.toggleCheckbox()
            }
            .padding(.horizontal, Spacing.large)
            .padding(.bottom, Spacing.extraLarge)
            PrimaryButton(
                action: viewModel.onDoneTap,
                text: Localizable.Onboarding.Agreements.Action.accept.key,
                style: .primary(isDisabled: $viewModel.isActionDisabled)
            )
            .padding(.horizontal, Spacing.large)
            .padding(.bottom, Spacing.large)
        }
        .background(.backgroundSystem)
        .fullScreenModal(isPresented: $viewModel.isPresentingTermsOfService) {
            TermsOfServiceView(viewModel: .init(onBackTap: { viewModel.isPresentingTermsOfService = false }))
        }
        .fullScreenModal(isPresented: $viewModel.isPresentingPrivacyPolicy) {
            PrivacyPolicyView(viewModel: .init(onBackTap: { viewModel.isPresentingPrivacyPolicy = false }))
        }
    }
}

extension OnboardingAgreementsView {
    final class ViewModel: ObservableObject {
        @Published var isCheckboxSelected: Bool = false
        @Published var isPresentingTermsOfService = false
        @Published var isPresentingPrivacyPolicy = false
        @Published var isActionDisabled: Bool = true
        private let onNextTap: () -> Void

        init(onNextTap: @escaping () -> Void) {
            self.onNextTap = onNextTap
        }

        func onTermsOfServiceTap() {
            isPresentingTermsOfService = true
        }

        func onPrivacyPolicyTap() {
            isPresentingPrivacyPolicy = true
        }

        func onDoneTap() {
            onNextTap()
        }

        func toggleCheckbox() {
            isCheckboxSelected.toggle()
            isActionDisabled = !isCheckboxSelected
        }
    }
}

#if DEBUG
    struct OnboardingAgreementsView_Previews: PreviewProvider {
        static var previews: some View {
            OnboardingAgreementsView(
                viewModel: .init(onNextTap: {})
            )
        }
    }
#endif

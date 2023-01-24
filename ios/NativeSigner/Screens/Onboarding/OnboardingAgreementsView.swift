//
//  OnboardingAgreementsView.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 24/01/2023.
//

import SwiftUI

struct OnboardingAgreementsView: View {
    @StateObject var viewModel: ViewModel
    @EnvironmentObject var data: SignerDataModel

    var body: some View {
        VStack(alignment: .leading, spacing: 0) {
            Localizable.Onboarding.Agreements.Label.title.text
                .font(PrimaryFont.titleXL.font)
                .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                .padding(.top, Spacing.extraExtraLarge)
                .padding(.bottom, Spacing.medium)
                .padding(.horizontal, Spacing.large)
            VStack(spacing: 0) {
                HStack(spacing: 0) {
                    Localizable.Onboarding.Agreements.Label.privacyCell.text
                        .padding(.leading, Spacing.medium)
                    Spacer()
                    Asset.chevronRight.swiftUIImage
                        .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
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
                    Asset.chevronRight.swiftUIImage
                        .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
                        .padding(.trailing, Spacing.medium)
                }
                .frame(height: Heights.onboardingAgreementRecord)
                .contentShape(Rectangle())
                .onTapGesture {
                    viewModel.onTermsOfServiceTap()
                }
            }
            .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
            .font(PrimaryFont.titleS.font)
            .containerBackground(CornerRadius.small)
            .padding(Spacing.medium)
            Spacer()
            HStack(spacing: Spacing.small) {
                if viewModel.isCheckboxSelected {
                    Asset.checkboxChecked.swiftUIImage
                        .foregroundColor(Asset.accentPink300.swiftUIColor)
                } else {
                    Asset.checkboxEmpty.swiftUIImage
                        .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
                }
                Localizable.Onboarding.Agreements.Label.confirmation.text
                    .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
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
        .background(Asset.backgroundPrimary.swiftUIColor)
        .onAppear { viewModel.use(data: data) }
        .fullScreenCover(isPresented: $viewModel.isPresentingTermsOfService) {
            TermsOfServiceView(viewModel: .init(isPresented: $viewModel.isPresentingTermsOfService))
        }
        .fullScreenCover(isPresented: $viewModel.isPresentingPrivacyPolicy) {
            PrivacyPolicyView(viewModel: .init(isPresented: $viewModel.isPresentingPrivacyPolicy))
        }
    }
}

extension OnboardingAgreementsView {
    final class ViewModel: ObservableObject {
        @Published var isCheckboxSelected: Bool = false
        @Published var isPresentingTermsOfService = false
        @Published var isPresentingPrivacyPolicy = false
        @Published var isActionDisabled: Bool = true

        private weak var data: SignerDataModel!

        func use(data: SignerDataModel) {
            self.data = data
        }

        func onTermsOfServiceTap() {
            isPresentingTermsOfService = true
        }

        func onPrivacyPolicyTap() {
            isPresentingPrivacyPolicy = true
        }

        func onDoneTap() {
            data.onboard()
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
                viewModel: .init()
            )
        }
    }
#endif

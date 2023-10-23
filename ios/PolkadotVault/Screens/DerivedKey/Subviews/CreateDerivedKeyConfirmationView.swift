//
//  CreateDerivedKeyConfirmationView.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 12/01/2023.
//

import SwiftUI

struct CreateDerivedKeyConfirmationView: View {
    @StateObject var viewModel: ViewModel

    var body: some View {
        FullScreenRoundedModal(
            animateBackground: $viewModel.animateBackground,
            ignoredEdges: .bottom,
            content: {
                VStack(alignment: .leading, spacing: 0) {
                    // Header
                    Localizable.CreateDerivedKey.Modal.Confirmation.title.text
                        .foregroundColor(.textAndIconsPrimary)
                        .font(PrimaryFont.titleL.font)
                        .padding(.bottom, Spacing.small)
                    Localizable.CreateDerivedKey.Modal.Confirmation.content.text
                        .foregroundColor(.textAndIconsSecondary)
                        .font(PrimaryFont.bodyL.font)
                        .padding(.bottom, Spacing.large)
                    VStack(alignment: .leading, spacing: 0) {
                        Localizable.CreateDerivedKey.Modal.Confirmation.header.text
                            .foregroundColor(.textAndIconsSecondary)
                            .font(PrimaryFont.bodyL.font)
                            .padding(.bottom, Spacing.extraSmall)
                        if viewModel.derivationPath.formattedAsPasswordedPath.isEmpty {
                            Localizable.CreateDerivedKey.Modal.Confirmation.emptyPath.text
                                .padding(.bottom, Spacing.extraSmall)
                        } else {
                            Text(viewModel.derivationPath.formattedAsPasswordedPath)
                                .padding(.bottom, Spacing.extraSmall)
                        }
                        HStack { Spacer() }
                    }
                    .foregroundColor(.accentPink300)
                    .font(PrimaryFont.bodyL.font)
                    .padding(Spacing.medium)
                    .strokeContainerBackground()
                    .padding(.bottom, Spacing.medium)
                    HStack(spacing: Spacing.small) {
                        if viewModel.isCheckboxSelected {
                            Image(.checkboxChecked)
                                .foregroundColor(.textAndIconsPrimary)
                        } else {
                            Image(.checkboxEmpty)
                                .foregroundColor(.textAndIconsPrimary)
                        }
                        Localizable.CreateDerivedKey.Modal.Confirmation.confirmation.text
                            .foregroundColor(.textAndIconsPrimary)
                            .font(PrimaryFont.bodyL.font)
                    }
                    .contentShape(Rectangle())
                    .onTapGesture {
                        viewModel.toggleCheckbox()
                    }
                    .padding(.bottom, Spacing.large)
                    PrimaryButton(
                        action: viewModel.onDoneTap,
                        text: Localizable.CreateDerivedKey.Modal.Confirmation.action.key,
                        style: .primary(isDisabled: $viewModel.isActionDisabled)
                    )
                }
                .padding(.horizontal, Spacing.large)
                .padding(.bottom, Spacing.large)
                .padding(.top, Spacing.small)
            }
        )
    }
}

extension CreateDerivedKeyConfirmationView {
    final class ViewModel: ObservableObject {
        private let onCompletion: () -> Void
        @Published var animateBackground: Bool = false
        @Published var isActionDisabled: Bool = true
        @Published var isCheckboxSelected: Bool = false

        let derivationPath: String

        init(
            derivationPath: String,
            onCompletion: @escaping () -> Void
        ) {
            self.derivationPath = derivationPath
            self.onCompletion = onCompletion
        }

        func onDoneTap() {
            Animations.chainAnimation(
                animateBackground.toggle(),
                // swiftformat:disable all
                delayedAnimationClosure: self.confirmAction()
            )
        }

        func confirmAction() {

            onCompletion()
        }

        func toggleCheckbox() {
            isCheckboxSelected.toggle()
            isActionDisabled = !isCheckboxSelected
        }
    }
}

#if DEBUG
    struct CreateDerivedKeyConfirmationView_Previews: PreviewProvider {
        static var previews: some View {
            CreateDerivedKeyConfirmationView(
                viewModel: .init(
                    derivationPath: "//polkadot//1///•••••••",
                    onCompletion: { }
                )
            )
        }
    }
#endif

//
//  CreateDerivedKeyConfirmationView.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 12/01/2023.
//

import SwiftUI

struct CreateDerivedKeyConfirmationView: View {
    @StateObject var viewModel: ViewModel
    @EnvironmentObject private var navigation: NavigationCoordinator

    var body: some View {
        FullScreenRoundedModal(
            animateBackground: $viewModel.animateBackground,
            ignoredEdges: .bottom,
            content: {
                VStack(alignment: .leading, spacing: 0) {
                    // Header
                    Localizable.CreateDerivedKey.Modal.Confirmation.title.text
                        .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                        .font(PrimaryFont.titleL.font)
                        .padding(.bottom, Spacing.small)
                    Localizable.CreateDerivedKey.Modal.Confirmation.content.text
                        .foregroundColor(Asset.textAndIconsSecondary.swiftUIColor)
                        .font(PrimaryFont.bodyL.font)
                        .padding(.bottom, Spacing.large)
                    VStack(alignment: .leading, spacing: 0) {
                        Localizable.CreateDerivedKey.Modal.Confirmation.header.text
                            .foregroundColor(Asset.textAndIconsSecondary.swiftUIColor)
                            .font(PrimaryFont.bodyL.font)
                            .padding(.bottom, Spacing.extraSmall)
                        Text(viewModel.derivationPath.formattedAsPasswordedPath)
                            .foregroundColor(Asset.accentPink300.swiftUIColor)
                            .font(PrimaryFont.bodyL.font)
                            .padding(.bottom, Spacing.extraSmall)
                        HStack { Spacer() }
                    }
                    .padding(Spacing.medium)
                    .strokeContainerBackground()
                    .padding(.bottom, Spacing.medium)
                    HStack(spacing: Spacing.small) {
                        if viewModel.isCheckboxSelected {
                            Asset.checkboxChecked.swiftUIImage
                                .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                        } else {
                            Asset.checkboxEmpty.swiftUIImage
                                .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                        }
                        Localizable.CreateDerivedKey.Modal.Confirmation.confirmation.text
                            .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
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
                .onAppear {
                    viewModel.use(navigation: navigation)
                }
            }
        )
    }
}

extension CreateDerivedKeyConfirmationView {
    final class ViewModel: ObservableObject {
        private weak var navigation: NavigationCoordinator!
        private let snackbarPresentation: BottomSnackbarPresentation

        @Published var animateBackground: Bool = false
        @Binding var isPresented: Bool
        @Binding var derivationPath: String
        @Published var isActionDisabled: Bool = true
        @Published var isCheckboxSelected: Bool = false

        init(
            isPresented: Binding<Bool>,
            derivationPath: Binding<String>,
            snackbarPresentation: BottomSnackbarPresentation = ServiceLocator.bottomSnackbarPresentation
        ) {
            _isPresented = isPresented
            _derivationPath = derivationPath
            self.snackbarPresentation = snackbarPresentation
        }

        func use(navigation: NavigationCoordinator) {
            self.navigation = navigation
        }

        func onDoneTap() {
            Animations.chainAnimation(
                animateBackground.toggle(),
                // swiftformat:disable all
                delayedAnimationClosure: self.confirmAction()
            )
        }

        func confirmAction() {
            navigation.perform(navigation: .init(action: .goBack))
            snackbarPresentation.viewModel = .init(
                title: Localizable.CreateDerivedKey.Snackbar.created.string,
                style: .info
            )
            snackbarPresentation.isSnackbarPresented = true
        }

        func toggleCheckbox() {
            isCheckboxSelected.toggle()
            isActionDisabled = !isCheckboxSelected
        }

        private func hide() {
            isPresented.toggle()
        }
    }
}

#if DEBUG
    struct CreateDerivedKeyConfirmationView_Previews: PreviewProvider {
        static var previews: some View {
            CreateDerivedKeyConfirmationView(
                viewModel: .init(
                    isPresented: .constant(true),
                    derivationPath: .constant("//polkadot//1///•••••••")
                )
            )
            .environmentObject(NavigationCoordinator())
        }
    }
#endif

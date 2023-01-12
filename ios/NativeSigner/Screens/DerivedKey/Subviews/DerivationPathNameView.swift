//
//  DerivationPathNameView.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 11/01/2023.
//

import SwiftUI

struct DerivationPathNameView: View {
    @FocusState var focusedPath: Bool
    @FocusState var focusedField: SecurePrimaryTextField.Field?
    @StateObject var viewModel: ViewModel
    @EnvironmentObject private var navigation: NavigationCoordinator
    @EnvironmentObject private var appState: AppState
    @State var isUpdatingText = false

    var body: some View {
        VStack(alignment: .leading, spacing: 0) {
            NavigationBarView(
                viewModel: NavigationBarViewModel(
                    title: Localizable.CreateDerivedKey.Label.title.string,
                    leftButton: .xmark,
                    rightButton: .action(Localizable.CreateDerivedKey.Modal.Path.Action.navigation.key),
                    backgroundColor: Asset.backgroundSystem.swiftUIColor
                ),
                actionModel: .init(
                    leftBarMenuAction: viewModel.onDismissTap,
                    rightBarMenuAction: viewModel.onRightNavigationButtonTap
                )
            )
            VStack(alignment: .leading, spacing: 0) {
                TextField("", text: $viewModel.maskedText)
                    .primaryTextFieldStyle(
                        Localizable.CreateDerivedKey.Modal.Path.Placeholder.path.string,
                        text: $viewModel.maskedText
                    )
                    .autocorrectionDisabled()

                    .submitLabel(.next)
                    .focused($focusedPath)
                    .onSubmit {
                        if viewModel.isPassworded {
                            focusedField = .secure
                        }
                    }
                    .onChange(of: viewModel.maskedText) { newValue in
//                        guard !isUpdatingText else { return }
//                        // Handle new characters appending (real ones, not masked "•")
//                        // Support multiple characters change in one edit
//                        if newValue.count > self.viewModel.inputText.count {
//                            self.viewModel
//                                .inputText += String(newValue.suffix(newValue.count - viewModel.inputText.count))
//                        }
//                        // Handle delete action, support multiple characters change in one edit
//                        if newValue.count < self.viewModel.inputText.count {
//                            self.viewModel
//                                .inputText = String(self.viewModel.inputText.prefix(newValue.count))
//                        }
                        let components = newValue
                            .components(separatedBy: ViewModel.DerivationPathComponent.passworded.description)
                        if components.count > 1 {
                            self.viewModel.isPassworded = true
//                            self.isUpdatingText = true
//                            self.viewModel
//                                .maskedText = components[0] + "///" + String(repeating: "•", count:
//                                components[1].count)
//                            isUpdatingText = false
                        } else {
//                            self.viewModel.isPassworded = false
//                            self.viewModel.inputText = newValue
                        }
                    }
                    .padding(.bottom, Spacing.extraSmall)
                quickActions()
                    .padding(.bottom, Spacing.extraSmall)
                Localizable.CreateDerivedKey.Modal.Path.Footer.path.text
                    .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
                    .font(PrimaryFont.captionM.font)
                    .padding(.vertical, Spacing.extraSmall)
                if viewModel.isPassworded {
                    Localizable.CreateDerivedKey.Modal.Path.Header.password.text
                        .font(PrimaryFont.bodyL.font)
                        .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                        .padding(.bottom, Spacing.medium)
                        .padding(.top, Spacing.medium)
                    SecurePrimaryTextField(
                        placeholder: Localizable.CreateDerivedKey.Modal.Path.Placeholder.password.string,
                        text: $viewModel.passwordConfirmation,
                        isValid: $viewModel.isPasswordValid,
                        focusedField: _focusedField,
                        onCommit: {
                            self.focusedField = nil
                            focusedPath = false
                            viewModel.onPasswordConfirmationDoneTap()
                        }
                    )
                    .padding(.bottom, Spacing.small)
                    if !viewModel.isPasswordValid {
                        Localizable.CreateDerivedKey.Modal.Path.Error.password.text
                            .foregroundColor(Asset.accentRed300.swiftUIColor)
                            .font(PrimaryFont.captionM.font)
                            .padding(.bottom, Spacing.small)
                    }
                }
                AttributedInfoBoxView(text: Localizable.createDerivedKeyModalPathInfo())
                    .onTapGesture { viewModel.onInfoBoxTap() }
                    .padding(.vertical, Spacing.extraSmall)
//                Text("Masked Text: " + viewModel.maskedText)
//                Text("Input Text: " + viewModel.inputText)
                Spacer()
            }
            .padding(.horizontal, Spacing.large)
            .padding(.vertical, Spacing.medium)
        }
        .background(Asset.backgroundPrimary.swiftUIColor)
        .onAppear {
            viewModel.use(navigation: navigation)
        }
        .fullScreenCover(
            isPresented: $viewModel.isPresentingInfoModal
        ) {
            ErrorBottomModal(
                viewModel: viewModel.presentableInfoModal,
                isShowingBottomAlert: $viewModel.isPresentingInfoModal
            )
            .clearModalBackground()
        }
    }

    @ViewBuilder
    func quickActions() -> some View {
        HStack(spacing: Spacing.extraExtraSmall) {
            Localizable.CreateDerivedKey.Modal.Path.Action.softPath.text
                .asSoftCapsuleButton()
                .onTapGesture {
                    viewModel.onSoftPathTap()
                }
            Localizable.CreateDerivedKey.Modal.Path.Action.hardPath.text
                .asSoftCapsuleButton()
                .onTapGesture {
                    viewModel.onHardPathTap()
                }
            Localizable.CreateDerivedKey.Modal.Path.Action.passwordedPath.text
                .asSoftCapsuleButton()
                .onTapGesture {
                    viewModel.onPasswordedPathTap()
                }
            Spacer()
        }
    }
}

struct SoftCapsuleButton: ViewModifier {
    func body(content: Content) -> some View {
        content
            .foregroundColor(Asset.accentPink300.swiftUIColor)
            .font(PrimaryFont.labelS.font)
            .padding(.vertical, Spacing.extraSmall)
            .padding(.horizontal, Spacing.medium)
            .background(Asset.accentPink12.swiftUIColor)
            .clipShape(Capsule())
    }
}

extension View {
    func asSoftCapsuleButton() -> some View {
        modifier(SoftCapsuleButton())
    }
}

extension DerivationPathNameView {
    final class ViewModel: ObservableObject {
        enum DerivationPathComponent: String, CustomStringConvertible {
            case soft = "/"
            case hard = "//"
            case passworded = "///"

            var description: String { rawValue }
        }

        private weak var navigation: NavigationCoordinator!
        private let createKeyService: CreateDerivedKeyService

        @Published var isPassworded: Bool = false
        @Published var inputText: String = ""
        @Published var maskedText: String = ""
        @Published var passwordConfirmation: String = ""
        @Published var isPasswordValid: Bool = true
        @Binding var derivationPath: String
        @Binding var selectedNetworks: [MmNetwork]
        @Binding var isPresented: Bool
        private let cancelBag = CancelBag()

        // State presentatation
        @Published var isPresentingInfoModal: Bool = false
        @Published var presentableInfoModal: ErrorBottomModalViewModel = .derivationPathsInfo()

        init(
            derivationPath: Binding<String>,
            isPresented: Binding<Bool>,
            selectedNetworks: Binding<[MmNetwork]>,
            createKeyService: CreateDerivedKeyService = CreateDerivedKeyService()
        ) {
            _derivationPath = derivationPath
            _isPresented = isPresented
            _selectedNetworks = selectedNetworks
            self.createKeyService = createKeyService
            subscribeToChanges()
        }

        func use(navigation: NavigationCoordinator) {
            self.navigation = navigation
        }

        func onDismissTap() {
            isPresented = false
        }

        func onRightNavigationButtonTap() {
            derivationPath = inputText
            isPresented = false
        }

        func onDerivationPathQuestionTap() {
            isPresented = false
        }

        func onInfoBoxTap() {
            isPresentingInfoModal = true
        }

        func onSoftPathTap() {
            maskedText.append(DerivationPathComponent.soft.description)
        }

        func onHardPathTap() {
            maskedText.append(DerivationPathComponent.hard.description)
        }

        func onPasswordedPathTap() {
            maskedText.append(DerivationPathComponent.passworded.description)
        }

        func onPasswordConfirmationDoneTap() {
            guard let password = maskedText
                .components(separatedBy: ViewModel.DerivationPathComponent.passworded.description).last else { return }
            isPasswordValid = password == passwordConfirmation
        }

        private func subscribeToChanges() {}
    }
}

#if DEBUG
    struct DerivationPathNameView_Previews: PreviewProvider {
        static var previews: some View {
            DerivationPathNameView(
                viewModel: .init(
                    derivationPath: .constant("path"),
                    isPresented: .constant(true),
                    selectedNetworks: .constant([])
                )
            )
            .environmentObject(NavigationCoordinator())
        }
    }
#endif

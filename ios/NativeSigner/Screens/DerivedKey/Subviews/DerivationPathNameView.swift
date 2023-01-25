//
//  DerivationPathNameView.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 11/01/2023.
//

import Combine
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
                    title: Localizable.CreateDerivedKey.Modal.Path.title.string,
                    leftButton: .xmark,
                    rightButton: .activeAction(
                        Localizable.CreateDerivedKey.Modal.Path.Action.navigation.key,
                        $viewModel.isMainActionDisabled
                    ),
                    backgroundColor: Asset.backgroundSystem.swiftUIColor
                ),
                actionModel: .init(
                    leftBarMenuAction: viewModel.onDismissTap,
                    rightBarMenuAction: viewModel.onRightNavigationButtonTap
                )
            )
            ScrollView(showsIndicators: false) {
                VStack(alignment: .leading, spacing: 0) {
                    TextField("", text: $viewModel.inputText)
                        .primaryTextFieldStyle(
                            Localizable.CreateDerivedKey.Modal.Path.Placeholder.path.string,
                            text: $viewModel.inputText,
                            isValid: .constant(viewModel.derivationPathError == nil)
                        )
                        .autocorrectionDisabled()
                        .submitLabel(.next)
                        .focused($focusedPath)
                        .onSubmit {
                            if viewModel.isPassworded {
                                focusedField = .secure
                            }
                        }
                        .onChange(of: viewModel.inputText) { newValue in
                            self.viewModel.isPassworded = newValue
                                .contains(DerivationPathComponent.passworded.description)
                            self.viewModel.validateDerivationPath()
                        }
                        .padding(.bottom, Spacing.extraSmall)
                    if let derivationPathError = viewModel.derivationPathError {
                        Text(derivationPathError)
                            .foregroundColor(Asset.accentRed300.swiftUIColor)
                            .font(PrimaryFont.captionM.font)
                            .padding(.bottom, Spacing.small)
                    }
                    if viewModel.isEntrySuggestionActive {
                        Localizable.CreateDerivedKey.Modal.Path.Suggestion.path.text
                            .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                            .font(PrimaryFont.captionM.font)
                            .padding(.bottom, Spacing.small)
                    }
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
                    Spacer()
                }
                .padding(.horizontal, Spacing.large)
                .padding(.vertical, Spacing.medium)
            }
            .background(Asset.backgroundPrimary.swiftUIColor)
        }
        .background(Asset.backgroundPrimary.swiftUIColor)
        .onAppear {
            viewModel.use(navigation: navigation)
            viewModel.onAppear()
            focusedPath = true
        }
        .fullScreenCover(
            isPresented: $viewModel.isPresentingInfoModal
        ) {
            DerivationMethodsInfoView(
                viewModel: .init(
                    isPresented: $viewModel.isPresentingInfoModal
                )
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
        private weak var navigation: NavigationCoordinator!
        private let createKeyService: CreateDerivedKeyService

        private let seedName: String
        @Published var inputText: String = ""
        @Published var maskedText: String = ""
        @Published var passwordConfirmation: String = ""
        @Published var isPassworded: Bool = false
        @Published var isPasswordValid: Bool = true
        @Published var isMainActionDisabled: Bool = true
        @Published var derivationPathError: String?
        @Binding var derivationPath: String?
        @Binding var networkSelection: NetworkSelection
        @Binding var isPresented: Bool
        private var skipValidation = false
        private let cancelBag = CancelBag()

        var isEntrySuggestionActive: Bool {
            DerivationPathComponent.allCases.contains { inputText == $0.description } && derivationPathError == nil
        }

        // State presentatation
        @Published var isPresentingInfoModal: Bool = false
        @Published var presentableInfoModal: ErrorBottomModalViewModel = .derivationPathsInfo()

        init(
            seedName: String,
            derivationPath: Binding<String?>,
            isPresented: Binding<Bool>,
            networkSelection: Binding<NetworkSelection>,
            createKeyService: CreateDerivedKeyService = CreateDerivedKeyService()
        ) {
            self.seedName = seedName
            _derivationPath = derivationPath
            _isPresented = isPresented
            _networkSelection = networkSelection
            self.createKeyService = createKeyService
            subscribeToChanges()
        }

        func use(navigation: NavigationCoordinator) {
            self.navigation = navigation
        }

        func onAppear() {
            if derivationPath == nil {
                skipValidation = true
                inputText = DerivationPathComponent.hard.description
                isMainActionDisabled = true
            } else {
                inputText = derivationPath ?? ""
            }
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
            inputText.append(DerivationPathComponent.soft.description)
        }

        func onHardPathTap() {
            inputText.append(DerivationPathComponent.hard.description)
        }

        func onPasswordedPathTap() {
            inputText.append(DerivationPathComponent.passworded.description)
        }

        func validateDerivationPath() {
            guard !skipValidation else {
                skipValidation = false
                return
            }
            switch networkSelection {
            case let .network(network):
                pathErrorCheck(createKeyService.checkForCollision(seedName, inputText, network.key))
            case let .allowedOnAnyNetwork(networks):
                let checks = networks
                    .map { network in createKeyService.checkForCollision(seedName, inputText, network.key) }
                if let firstEncounteredError = checks
                    .compactMap({
                        if case let .success(derivationCheck) = $0 {
                            return derivationCheck
                        } else {
                            return nil
                        }
                    }).first(where: { $0.buttonGood == false || $0.error != nil }) {
                    pathErrorCheck(.success(firstEncounteredError))
                } else {
                    derivationPathError = nil
                }
            }
        }

        private func pathErrorCheck(_ result: Result<DerivationCheck, ServiceError>) {
            switch result {
            case let .success(derivationCheck):
                if derivationCheck.collision != nil {
                    derivationPathError = Localizable.CreateDerivedKey.Modal.Path.Error.alreadyExists.string
                } else if !derivationCheck.buttonGood {
                    derivationPathError = Localizable.CreateDerivedKey.Modal.Path.Error.pathInvalid.string
                } else if derivationCheck.error != nil {
                    derivationPathError = derivationCheck.error
                } else {
                    derivationPathError = nil
                }
            case let .failure(error):
                presentableInfoModal = .alertError(message: error.localizedDescription)
                isPresentingInfoModal = true
            }
        }

        func onPasswordConfirmationDoneTap() {
            isPasswordValid = isPasswordConfirmationValid()
        }

        private func subscribeToChanges() {
            Publishers
                .CombineLatest3($isPassworded, $isPasswordValid, $derivationPathError)
                .map { validators -> Bool in
                    let (isPassworded, isPasswordValid, derivationPathError) = validators
                    if isPassworded {
                        return (
                            !isPasswordValid ||
                                !self.isPasswordConfirmationValid() ||
                                self.derivationPathError != nil
                        )
                    } else {
                        return derivationPathError != nil || self.isInitialEntry()
                    }
                }
                .assign(to: \.isMainActionDisabled, on: self)
                .store(in: cancelBag)
        }

        func isPasswordConfirmationValid() -> Bool {
            guard isPassworded else { return true }
            if let range = inputText.range(of: DerivationPathComponent.passworded.description) {
                let substring = inputText.suffix(from: range.upperBound)
                return substring == passwordConfirmation
            }
            return false
        }

        private func isInitialEntry() -> Bool {
            inputText == DerivationPathComponent.hard.description
        }
    }
}

#if DEBUG
    struct DerivationPathNameView_Previews: PreviewProvider {
        static var previews: some View {
            DerivationPathNameView(
                viewModel: .init(
                    seedName: "Keys",
                    derivationPath: .constant("path"),
                    isPresented: .constant(true),
                    networkSelection: .constant(.allowedOnAnyNetwork([]))
                )
            )
            .environmentObject(NavigationCoordinator())
        }
    }
#endif

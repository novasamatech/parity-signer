//
//  DerivationPathNameView.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 11/01/2023.
//

import SwiftUI

struct DerivationPathNameView: View {
    enum Field: Hashable {
        case path
        case password
    }

    @FocusState var focusedField: Field?
    @StateObject var viewModel: ViewModel
    @EnvironmentObject private var navigation: NavigationCoordinator
    @EnvironmentObject private var appState: AppState

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
                TextField("", text: $viewModel.derivationPath)
                    .primaryTextFieldStyle(
                        Localizable.CreateDerivedKey.Modal.Path.Placeholder.path.string,
                        text: $viewModel.derivationPath
                    )
                    .submitLabel(.next)
                    .focused($focusedField, equals: .path)
                    .onSubmit {
                        // focusedField = .secure
                    }
                    .padding(.bottom, Spacing.extraSmall)
                quickActions()
                    .padding(.bottom, Spacing.extraSmall)
                Localizable.CreateDerivedKey.Modal.Path.Footer.path.text
                    .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
                    .font(PrimaryFont.captionM.font)
                    .padding(.vertical, Spacing.extraSmall)
                AttributedInfoBoxView(text: Localizable.createDerivedKeyModalPathInfo())
                    .padding(.vertical, Spacing.extraSmall)
                Spacer()
            }
            .padding(.horizontal, Spacing.large)
            .padding(.vertical, Spacing.medium)
        }
        .background(Asset.backgroundPrimary.swiftUIColor)
        .onAppear {
            viewModel.use(navigation: navigation)
            focusedField = .path
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

        @Published var editedDerivationPath: String = ""
        @Binding var derivationPath: String
        @Binding var selectedNetworks: [MmNetwork]
        @Binding var isPresented: Bool
        private let cancelBag = CancelBag()

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
            derivationPath = editedDerivationPath
            isPresented = false
        }

        func onDerivationPathQuestionTap() {
            isPresented = false
        }

        func onSoftPathTap() {}
        func onHardPathTap() {}
        func onPasswordedPathTap() {}

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

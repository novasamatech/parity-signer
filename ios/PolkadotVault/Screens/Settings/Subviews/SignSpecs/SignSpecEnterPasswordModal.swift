//
//  SignSpecEnterPasswordModal.swift
//  PolkadotVault
//
//  Created by Krzysztof Rodak on 27/03/2023.
//

import SwiftUI

struct SignSpecEnterPasswordModal: View {
    @StateObject var viewModel: ViewModel
    @FocusState var focusedField: SecurePrimaryTextField.Field?
    @State var animateBackground: Bool = false
    @Environment(\.safeAreaInsets) private var safeAreaInsets

    var body: some View {
        FullScreenRoundedModal(
            backgroundTapAction: {
                viewModel.onCancelTap()
            },
            animateBackground: $animateBackground,
            ignoredEdges: .top
        ) {
            VStack(spacing: Spacing.medium) {
                HStack {
                    Button(
                        action: viewModel.onCancelTap
                    ) {
                        Asset.xmarkButtonMedium.swiftUIImage
                            .foregroundColor(Asset.textAndIconsSecondary.swiftUIColor)
                            .frame(
                                width: Heights.navigationButton,
                                height: Heights.navigationButton,
                                alignment: .center
                            )
                    }
                    .padding(.leading, Spacing.extraExtraSmall)
                    Spacer()
                    CapsuleButton(
                        action: viewModel.onDoneTap,
                        title: Localizable.Transaction.EnterPassword.Action.done.string,
                        isDisabled: $viewModel.isActionDisabled
                    )
                }
                .padding(.top, -Spacing.extraSmall)
                .padding(.horizontal, Spacing.extraSmall)
                VStack(alignment: .leading, spacing: 0) {
                    Localizable.Transaction.EnterPassword.Label.title.text
                        .font(PrimaryFont.titleL.font)
                        .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                        .padding(.bottom, Spacing.extraSmall)
                    keyComponent()
                        .padding(.top, Spacing.medium)
                    SecurePrimaryTextField(
                        placeholder: Localizable.Transaction.EnterPassword.Label.placeholder.string,
                        text: $viewModel.password,
                        isValid: $viewModel.isValid,
                        focusedField: _focusedField,
                        onCommit: { viewModel.onDoneTap() }
                    )
                    .padding(.top, Spacing.medium)
                    Group {
                        if !viewModel.isValid {
                            Localizable.Transaction.EnterPassword.Label.invalidPassword.text
                                .foregroundColor(Asset.accentRed300.swiftUIColor)
                        }
                        Localizable.Transaction.EnterPassword.Label.explanation.text
                            .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
                    }
                    .font(PrimaryFont.captionM.font)
                    .padding(.top, Spacing.extraSmall)
                    .padding(.bottom, Spacing.small)
                }
                .padding(.horizontal, Spacing.large)
            }
            .background(Asset.backgroundTertiary.swiftUIColor)
            .onAppear {
                focusedField = .secure
            }
        }
    }

    @ViewBuilder
    func keyComponent() -> some View {
        HStack {
            VStack(alignment: .leading, spacing: Spacing.extraExtraSmall) {
                renderablePath
                    .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
                    .font(PrimaryFont.captionM.font)
                Text(viewModel.dataModel.authorInfo.address.seedName)
                    .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                    .font(PrimaryFont.bodyM.font)
                HStack {
                    Text(
                        viewModel.dataModel.authorInfo.base58
                            .truncateMiddle()
                    )
                    .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
                    .font(PrimaryFont.bodyM.font)
                }
            }
            Spacer()
            NetworkIdenticon(
                identicon: viewModel.dataModel.authorInfo.address.identicon,
                network: viewModel.dataModel.networkInfo?.networkLogo,
                background: Asset.accentRed300Overlay.swiftUIColor,
                size: Heights.identiconInCell
            )
        }
        .padding(Spacing.small)
        .containerBackground(state: .error)
    }

    /// Manual string interpolation for `lock` `SFSymbol`
    private var renderablePath: Text {
        Text(
            // swiftlint:disable:next line_length
            "\(viewModel.dataModel.authorInfo.address.path)\(Localizable.Shared.Label.passwordedPathDelimeter.string)\(Image(.lock))"
        )
    }
}

extension SignSpecEnterPasswordModal {
    final class ViewModel: ObservableObject {
        private let navigation: NavigationCoordinator
        @Binding var isPresented: Bool
        @Binding var shouldPresentError: Bool
        @Binding var dataModel: MEnterPassword
        @Binding var detailsContent: MSufficientCryptoReady?

        @Published var password: String = ""
        @Published var isActionDisabled: Bool = true
        @Published var isValid: Bool = true
        private var cancelBag = CancelBag()

        init(
            isPresented: Binding<Bool>,
            shouldPresentError: Binding<Bool>,
            dataModel: Binding<MEnterPassword>,
            detailsContent: Binding<MSufficientCryptoReady?>,
            navigation: NavigationCoordinator = NavigationCoordinator()
        ) {
            _isPresented = isPresented
            _shouldPresentError = shouldPresentError
            _dataModel = dataModel
            _detailsContent = detailsContent
            self.navigation = navigation
            subscribeToUpdates()
        }

        func onCancelTap() {
            navigation.performFake(navigation: .init(action: .goBack))
            isPresented = false
        }

        func onErrorDismiss() {
            isPresented = false
        }

        func onDoneTap() {
            let actionResult = navigation.performFake(navigation: .init(action: .goForward, details: password))
            switch actionResult?.modalData {
            case let .enterPassword(value):
                dataModel = value
                isValid = false
            case let .sufficientCryptoReady(value):
                detailsContent = value
                isPresented = false
                shouldPresentError = false
            default:
                navigation.performFake(navigation: .init(action: .goBack))
                proceedtoErrorState()
            }
        }

        private func proceedtoErrorState() {
            isPresented = false
            shouldPresentError = true
        }

        private func subscribeToUpdates() {
            $password.sink { newValue in
                self.isActionDisabled = newValue.isEmpty
            }
            .store(in: cancelBag)
        }
    }
}

struct SignSpecEnterPasswordModal_Previews: PreviewProvider {
    static var previews: some View {
        SignSpecEnterPasswordModal(
            viewModel: .init(
                isPresented: Binding<Bool>.constant(true),
                shouldPresentError: Binding<Bool>.constant(false),
                dataModel: Binding<MEnterPassword>.constant(
                    .init(
                        authorInfo: .init(
                            base58: PreviewData.base58,
                            addressKey: "01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e",
                            address: .init(
                                path: "//polkadot",
                                hasPwd: true,
                                identicon: .svg(image: PreviewData.exampleIdenticon),
                                seedName: "Parity Keys",
                                secretExposed: true
                            )
                        ),
                        networkInfo: MscNetworkInfo(
                            networkTitle: "Polkadot",
                            networkLogo: "polkadot",
                            networkSpecsKey: "sr25519"
                        ),
                        counter: 2
                    )
                ),
                detailsContent: .constant(nil)
            )
        )
    }
}

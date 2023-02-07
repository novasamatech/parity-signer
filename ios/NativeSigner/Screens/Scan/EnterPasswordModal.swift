//
//  EnterPasswordModal.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 24/11/2022.
//

import SwiftUI

struct EnterPasswordModal: View {
    @EnvironmentObject private var navigation: NavigationCoordinator
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
                viewModel.use(navigation: navigation)
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

extension EnterPasswordModal {
    final class ViewModel: ObservableObject {
        private weak var navigation: NavigationCoordinator!
        @Binding var isPresented: Bool
        @Binding var isErrorPresented: Bool
        @Binding var dataModel: MEnterPassword
        @Binding var signature: MSignatureReady?

        @Published var password: String = ""
        @Published var isActionDisabled: Bool = true
        @Published var isValid: Bool = true
        private var cancelBag = CancelBag()

        init(
            isPresented: Binding<Bool>,
            isErrorPresented: Binding<Bool>,
            dataModel: Binding<MEnterPassword>,
            signature: Binding<MSignatureReady?>
        ) {
            _isPresented = isPresented
            _isErrorPresented = isErrorPresented
            _dataModel = dataModel
            _signature = signature
            subscribeToUpdates()
        }

        func use(navigation: NavigationCoordinator) {
            self.navigation = navigation
        }

        func onCancelTap() {
            // Dismissing password modal goes to `Log` screen
            navigation.performFake(navigation: .init(action: .goBack))
            // Pretending to navigate back to `Scan` so navigation states for new QR code scan will work
            navigation.performFake(navigation: .init(action: .navbarScan))
            isPresented = false
        }

        func onErrorDismiss() {
            isPresented = false
        }

        func onDoneTap() {
            let actionResult = navigation.performFake(navigation: .init(action: .goForward, details: password))
            // If navigation returned `enterPassword`, it means password is invalid
            if case let .enterPassword(value) = actionResult.modalData {
                if value.counter > 3 {
                    proceedtoErrorState()
                    return
                }
                dataModel = value
                isValid = false
            }
            // If we got signature from navigation, we should return to camera view and there check for further
            // navigation to Transaction Details
            if case let .signatureReady(value) = actionResult.modalData {
                navigation.performFake(navigation: .init(action: .goBack))
                isPresented = false
                isErrorPresented = false
                // This needs to trigger navigation to Transaction Details in parent camera view via Binding
                signature = value
                return
            }
            // If we got `Log`, we need to hide password modal, "navigate" to camera view and present
            if case .log = actionResult.screenData {
                proceedtoErrorState()
            }
        }

        private func proceedtoErrorState() {
            navigation.performFake(navigation: .init(action: .goBack))
            // Inform parent camera view to present error for too many failed attempts at password
            isPresented = false
            isErrorPresented = true
            // Fake navigation to camera, as were brought back to `Log` screen on navstate error handling
            navigation.performFake(navigation: .init(action: .navbarScan))
        }

        private func subscribeToUpdates() {
            $password.sink { newValue in
                self.isActionDisabled = newValue.isEmpty
            }
            .store(in: cancelBag)
        }
    }
}

struct EnterPasswordModal_Previews: PreviewProvider {
    static var previews: some View {
        EnterPasswordModal(
            viewModel: .init(
                isPresented: Binding<Bool>.constant(true),
                isErrorPresented: Binding<Bool>.constant(false),
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
                signature: Binding<MSignatureReady?>.constant(nil)
            )
        )
        .environmentObject(NavigationCoordinator())
//        .preferredColorScheme(.dark)
    }
}

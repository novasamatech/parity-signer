//
//  EnterPasswordModal.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 24/11/2022.
//

import SwiftUI

struct EnterPasswordModal: View {
    @EnvironmentObject private var navigation: NavigationCoordinator
//    @StateObject var keyboardOffsetAdapter = KeyboardOffsetAdapter()
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
                        title: Localizable.Transaction.EnterPassword.Action.done.string
                    )
                }
                .padding(.top, -Spacing.extraSmall)
                .padding(.horizontal, Spacing.extraSmall)
                VStack(alignment: .leading, spacing: 0) {
                    Localizable.Transaction.EnterPassword.Label.title.text
                        .font(Fontstyle.titleL.base)
                        .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                        .padding(.bottom, Spacing.extraSmall)
                    keyComponent()
                        .padding(.top, Spacing.medium)
                    SecurePrimaryTextField(
                        placeholder: "",
                        text: $viewModel.password,
                        isValid: $viewModel.isValid,
                        focusedField: _focusedField
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
                    .font(Fontstyle.captionM.base)
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
                Text(viewModel.dataModel.authorInfo.address.path)
                    .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
                    .font(Fontstyle.captionM.base)
                Text(viewModel.dataModel.authorInfo.address.seedName)
                    .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                    .font(Fontstyle.bodyM.base)
                HStack {
                    Text(
                        viewModel.dataModel.authorInfo.base58
                            .truncateMiddle()
                    )
                    .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
                    .font(Fontstyle.bodyM.base)
                }
            }
            Spacer()
            Identicon(identicon: viewModel.dataModel.authorInfo.address.identicon, rowHeight: Heights.identiconInCell)
        }
        .padding(Spacing.small)
        .containerBackground(isTinted: true)
    }
}

extension EnterPasswordModal {
    final class ViewModel: ObservableObject {
        private weak var navigation: NavigationCoordinator!

        @Binding var isPresented: Bool
        @Binding var dataModel: MEnterPassword
        @Binding var signature: MSignatureReady?
        @Published var password: String = ""
        @State var isValid: Bool = true

        init(
            isPresented: Binding<Bool>,
            dataModel: Binding<MEnterPassword>,
            signature: Binding<MSignatureReady?>
        ) {
            _isPresented = isPresented
            _dataModel = dataModel
            _signature = signature
        }

        func use(navigation: NavigationCoordinator) {
            self.navigation = navigation
        }

        func onCancelTap() {
            navigation.performFake(navigation: .init(action: .goBack)) // going back to MTransaction
            isPresented.toggle()
        }

        func onDoneTap() {
            let actionResult = navigation.performFake(navigation: .init(action: .goForward, details: password))
            // If navigation returned `enterPassword`, it means password is invalid
            if case let .enterPassword(value) = actionResult.modalData {
                dataModel = value
                isValid = value.counter == 0
            }
            // If we got signature from navigation, we should return to camera view and there check for further
            // navigation to Transaction Details
            if case let .signatureReady(value) = actionResult.modalData {
                signature = value
                isPresented = false
            }
        }
    }
}

struct EnterPasswordModal_Previews: PreviewProvider {
    static var previews: some View {
        EnterPasswordModal(
            viewModel: .init(
                isPresented: Binding<Bool>.constant(true),
                dataModel: Binding<MEnterPassword>.constant(
                    .init(
                        authorInfo: .init(
                            base58: PreviewData.base58,
                            address: .init(
                                path: "//polkadot///",
                                hasPwd: true,
                                identicon: PreviewData.exampleIdenticon,
                                seedName: "Parity Keys",
                                secretExposed: true
                            ),
                            multiselect: nil
                        ),
                        counter: 2
                    )
                ),
                signature: Binding<MSignatureReady?>.constant(nil)
            )
        )
        .environmentObject(NavigationCoordinator())
        .preferredColorScheme(.dark)
    }
}

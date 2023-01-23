//
//  VerfierCertificateView.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 12/12/2022.
//

import SwiftUI

struct VerfierCertificateView: View {
    @StateObject var viewModel: ViewModel
    @EnvironmentObject private var navigation: NavigationCoordinator
    @EnvironmentObject private var data: SignerDataModel

    var body: some View {
        VStack(alignment: .leading, spacing: 0) {
            NavigationBarView(
                viewModel: NavigationBarViewModel(
                    title: Localizable.VerifierCertificate.Label.title.string,
                    leftButton: .arrow,
                    rightButton: .empty
                )
            )
            VStack {
                VStack(spacing: Spacing.small) {
                    VStack(alignment: .leading, spacing: Spacing.extraSmall) {
                        Localizable.Transaction.Verifier.Label.key.text
                            .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
                        Text(viewModel.content.publicKey)
                            .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                    }
                    Divider()
                    VStack(alignment: .leading) {
                        HStack {
                            Localizable.Transaction.Verifier.Label.crypto.text
                                .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
                            Spacer()
                            Text(viewModel.content.encryption)
                                .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                        }
                    }
                }
                .padding(Spacing.medium)
            }
            .containerBackground()
            .padding(.bottom, Spacing.extraSmall)
            .padding(.horizontal, Spacing.extraSmall)
            HStack {
                Text(Localizable.VerifierCertificate.Action.remove.string)
                    .font(PrimaryFont.titleS.font)
                    .foregroundColor(Asset.accentRed400.swiftUIColor)
                Spacer()
            }
            .contentShape(Rectangle())
            .onTapGesture {
                viewModel.onRemoveTap()
            }
            .frame(height: Heights.verifierCertificateActionHeight)
            .padding(.horizontal, Spacing.large)
            Spacer()
        }
        .background(Asset.backgroundPrimary.swiftUIColor)
        .onAppear {
            viewModel.use(data: data)
        }
        .fullScreenCover(isPresented: $viewModel.isPresentingRemoveConfirmation) {
            VerticalActionsBottomModal(
                viewModel: .removeGeneralVerifier,
                mainAction: viewModel.onRemoveConfirmationTap(),
                isShowingBottomAlert: $viewModel.isPresentingRemoveConfirmation
            )
            .clearModalBackground()
        }
    }
}

extension VerfierCertificateView {
    final class ViewModel: ObservableObject {
        @Published var renderable: SettingsViewRenderable = .init()
        @Published var isPresentingRemoveConfirmation = false

        let content: MVerifierDetails

        private weak var data: SignerDataModel!

        init(content: MVerifierDetails) {
            self.content = content
        }

        func use(data: SignerDataModel) {
            self.data = data
        }

        func loadData() {
            renderable = SettingsViewRenderable()
        }

        func onRemoveTap() {
            isPresentingRemoveConfirmation = true
        }

        func onRemoveConfirmationTap() {
            data.removeGeneralVerifier()
        }
    }
}

#if DEBUG
    struct VerfierCertificateView_Previews: PreviewProvider {
        static var previews: some View {
            VerfierCertificateView(viewModel: .init(content: .init(
                publicKey: "public key",
                identicon: .svg(image: PreviewData.exampleIdenticon),
                encryption: "fdsfds"
            )))
        }
    }
#endif

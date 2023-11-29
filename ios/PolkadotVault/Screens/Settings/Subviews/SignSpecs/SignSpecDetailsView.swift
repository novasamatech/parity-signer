//
//  SignSpecDetailsView.swift
//  PolkadotVault
//
//  Created by Krzysztof Rodak on 22/03/2023.
//

import Combine
import SwiftUI

struct SignSpecDetails: View {
    @StateObject var viewModel: ViewModel
    @State var isShowingFullAddress: Bool = false
    @Environment(\.presentationMode) var presentationMode

    var body: some View {
        VStack {
            NavigationBarView(
                viewModel: NavigationBarViewModel(
                    title: .title(viewModel.title),
                    leftButtons: [.init(
                        type: .arrow,
                        action: viewModel.onBackTap
                    )],
                    backgroundColor: .backgroundPrimary
                )
            )
            ScrollView {
                VStack(alignment: .leading, spacing: 0) {
                    signatureSection()
                        .padding(.horizontal, Spacing.medium)
                        .padding(.top, Spacing.extraSmall)
                        .padding(.bottom, Spacing.medium)
                    Text(viewModel.qrCodeSectionTitle)
                        .foregroundColor(.textAndIconsPrimary)
                        .font(PrimaryFont.bodyL.font)
                        .padding(.horizontal, Spacing.large)
                        .padding(.bottom, Spacing.extraSmall)
                    AnimatedQRCodeView(
                        viewModel: Binding<AnimatedQRCodeViewModel>.constant(
                            .init(
                                qrCodes: [viewModel.content.sufficient]
                            )
                        )
                    )
                    .padding(.horizontal, Spacing.large)
                    ActionButton(
                        action: viewModel.onBackTap,
                        text: Localizable.SignSpecsDetails.Action.done.key,
                        style: .secondary()
                    )
                    .padding(.horizontal, Spacing.large)
                    .padding(.top, Spacing.extraExtraLarge)
                    .padding(.bottom, Spacing.large)
                }
            }
        }
        .background(.backgroundPrimary)
        .onReceive(viewModel.dismissViewRequest) { _ in
            presentationMode.wrappedValue.dismiss()
        }
    }

    @ViewBuilder
    func signatureSection() -> some View {
        VStack(alignment: .leading, spacing: Spacing.small) {
            VStack(alignment: .leading, spacing: 0) {
                Group {
                    switch viewModel.content.content {
                    case let .addSpecs(network):
                        Text(
                            Localizable.SignSpecsDetails.Label
                                .networkSpecSignature(network.networkTitle, network.networkSpecsKey)
                        )
                    case .loadTypes:
                        Localizable.SignSpecsDetails.Label.typesSignature.text
                    case let .loadMetadata(name: name, version: version):
                        Text(Localizable.SignSpecsDetails.Label.networkMetadataSignature(name, String(version)))
                    }
                }
                .foregroundColor(.textAndIconsPrimary)
                .font(PrimaryFont.bodyM.font)
            }
            Divider()
            VStack(alignment: .leading, spacing: Spacing.extraSmall) {
                Localizable.SignSpecsDetails.Label.sign.text
                    .foregroundColor(.textAndIconsTertiary)
                    .font(PrimaryFont.captionM.font)
                HStack {
                    signatureDetails()
                    Spacer()
                    NetworkIdenticon(
                        identicon: viewModel.content.authorInfo.address.identicon,
                        network: viewModel.content.networkLogo,
                        background: .fill6Solid,
                        size: Heights.identiconInCell
                    )
                }
            }
        }
        .padding(Spacing.medium)
        .containerBackground(CornerRadius.small, state: .standard)
    }

    @ViewBuilder
    private func signatureDetails() -> some View {
        VStack(alignment: .leading, spacing: Spacing.minimal) {
            if !viewModel.content.authorInfo.address.displayablePath.isEmpty {
                Text(viewModel.content.authorInfo.address.displayablePath)
                    .foregroundColor(.textAndIconsTertiary)
                    .font(PrimaryFont.captionM.font)
            }
            Text(viewModel.content.authorInfo.address.seedName)
                .foregroundColor(.textAndIconsPrimary)
                .font(PrimaryFont.bodyM.font)
            HStack {
                Text(
                    isShowingFullAddress ? viewModel.content.authorInfo.base58 : viewModel.content.authorInfo.base58
                        .truncateMiddle()
                )
                .foregroundColor(.textAndIconsTertiary)
                .font(PrimaryFont.bodyM.font)
                if !isShowingFullAddress {
                    Image(.chevronDown)
                        .foregroundColor(.textAndIconsTertiary)
                        .padding(.leading, Spacing.extraExtraSmall)
                }
            }
            .contentShape(Rectangle())
            .onTapGesture {
                withAnimation {
                    isShowingFullAddress = true
                }
            }
        }
    }
}

extension SignSpecDetails {
    final class ViewModel: ObservableObject {
        private let type: SpecSignType
        let content: MSufficientCryptoReady
        var dismissViewRequest: AnyPublisher<Void, Never> {
            dismissRequest.eraseToAnyPublisher()
        }

        private let dismissRequest = PassthroughSubject<Void, Never>()

        init(
            content: MSufficientCryptoReady,
            type: SpecSignType
        ) {
            self.content = content
            self.type = type
        }

        func onBackTap() {
            dismissRequest.send()
        }

        var title: String {
            switch type {
            case .metadata:
                Localizable.SignSpecsDetails.Label.Title.metadata.string
            case .network:
                Localizable.SignSpecsDetails.Label.Title.specs.string
            }
        }

        var qrCodeSectionTitle: String {
            switch type {
            case .metadata:
                Localizable.SignSpecsDetails.Label.ScanQRCode.metadata.string
            case .network:
                Localizable.SignSpecsDetails.Label.ScanQRCode.specs.string
            }
        }
    }
}

//
//  SignSpecDetails.swift
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
                    title: .title(Localizable.SignSpecsDetails.Label.title.string),
                    leftButtons: [.init(
                        type: .arrow,
                        action: viewModel.onBackTap
                    )],
                    backgroundColor: Asset.backgroundPrimary.swiftUIColor
                )
            )
            ScrollView {
                VStack(alignment: .leading, spacing: 0) {
                    signatureSection()
                        .padding(.horizontal, Spacing.medium)
                        .padding(.top, Spacing.extraSmall)
                        .padding(.bottom, Spacing.medium)
                    Localizable.SignSpecsDetails.Label.scanQRCode.text
                        .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
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
                    SecondaryButton(
                        action: viewModel.onBackTap(),
                        text: Localizable.SignSpecsDetails.Action.done.key
                    )
                    .padding(.horizontal, Spacing.large)
                    .padding(.top, Spacing.extraExtraLarge)
                    .padding(.bottom, Spacing.large)
                }
            }
        }
        .background(Asset.backgroundPrimary.swiftUIColor)
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
                .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                .font(PrimaryFont.bodyM.font)
            }
            Divider()
            VStack(alignment: .leading, spacing: Spacing.extraSmall) {
                Localizable.SignSpecsDetails.Label.sign.text
                    .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
                    .font(PrimaryFont.captionM.font)
                HStack {
                    signatureDetails()
                    Spacer()
                    NetworkIdenticon(
                        identicon: viewModel.content.authorInfo.address.identicon,
                        network: viewModel.content.networkLogo,
                        background: Asset.fill6Solid.swiftUIColor,
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
                    .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
                    .font(PrimaryFont.captionM.font)
            }
            Text(viewModel.content.authorInfo.address.seedName)
                .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                .font(PrimaryFont.bodyM.font)
            HStack {
                Text(
                    isShowingFullAddress ? viewModel.content.authorInfo.base58 : viewModel.content.authorInfo.base58
                        .truncateMiddle()
                )
                .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
                .font(PrimaryFont.bodyM.font)
                if !isShowingFullAddress {
                    Asset.chevronDown.swiftUIImage
                        .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
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
        var content: MSufficientCryptoReady
        private let onComplete: () -> Void
        var dismissViewRequest: AnyPublisher<Void, Never> {
            dismissRequest.eraseToAnyPublisher()
        }

        private let dismissRequest = PassthroughSubject<Void, Never>()

        init(
            content: MSufficientCryptoReady,
            onComplete: @escaping () -> Void
        ) {
            self.content = content
            self.onComplete = onComplete
        }

        func onBackTap() {
            dismissRequest.send()
            onComplete()
        }
    }
}

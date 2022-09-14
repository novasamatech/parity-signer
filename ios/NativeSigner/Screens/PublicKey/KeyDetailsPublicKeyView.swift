//
//  KeyDetailsPublicKeyView.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 13/09/2022.
//

import SwiftUI

struct KeyDetailsPublicKeyViewModel: Equatable {
    let qrCode: QRCodeContainerViewModel
    let addressFooter: QRCodeAddressFooterViewModel?
    let rootFooter: QRCodeRootFooterViewModel?
    let isKeyExposed: Bool
    let isRootKey: Bool

    init(_ keyDetails: MKeyDetails) {
        qrCode = .init(qrCode: keyDetails.qr)
        rootFooter = keyDetails.isRootKey ? .init(
            keyName: keyDetails.address.seedName,
            base58: keyDetails.address.base58
        ) : nil
        addressFooter = keyDetails.isRootKey ? nil : .init(
            identicon: keyDetails.address.identicon,
            path: [keyDetails.address.seedName, keyDetails.address.path].joined(separator: " "),
            network: keyDetails.networkInfo.networkTitle,
            base58: keyDetails.address.base58
        )
        isKeyExposed = keyDetails.address.secretExposed
        isRootKey = keyDetails.isRootKey
    }

    init(
        qrCode: QRCodeContainerViewModel,
        addressFooter: QRCodeAddressFooterViewModel?,
        rootFooter: QRCodeRootFooterViewModel?,
        isKeyExposed: Bool,
        isRootKey: Bool
    ) {
        self.qrCode = qrCode
        self.addressFooter = addressFooter
        self.rootFooter = rootFooter
        self.isKeyExposed = isKeyExposed
        self.isRootKey = isRootKey
    }
}

struct KeyDetailsPublicKeyView: View {
    @ObservedObject private var navigation: NavigationCoordinator
    private let viewModel: KeyDetailsPublicKeyViewModel

    init(
        navigation: NavigationCoordinator,
        viewModel: KeyDetailsPublicKeyViewModel
    ) {
        self.navigation = navigation
        self.viewModel = viewModel
    }

    var body: some View {
        VStack(spacing: 0) {
            // Navigation bar
            NavigationBarView(
                navigation: navigation,
                viewModel: .init(
                    title: Localizable.PublicKeyDetails.Label.title.string,
                    subtitle: viewModel.isRootKey ? nil : Localizable.PublicKeyDetails.Label.subtitle.string,
                    leftButton: .xmark,
                    rightButton: .more
                )
            )
            ScrollView {
                VStack {
                    VStack(spacing: 0) {
                        QRCodeContainerView(viewModel: viewModel.qrCode)
                        if let addressFooter = viewModel.addressFooter {
                            QRCodeAddressFooterView(viewModel: addressFooter)
                        }
                        if let rootFooter = viewModel.rootFooter {
                            QRCodeRootFooterView(viewModel: rootFooter)
                        }
                    }
                    .background(
                        RoundedRectangle(cornerRadius: CornerRadius.medium)
                            .stroke(Asset.fill12.swiftUIColor, lineWidth: 1)
                            .background(Asset.fill6.swiftUIColor)
                            .cornerRadius(CornerRadius.medium)
                    )
                    // Exposed key alert
                    if viewModel.isKeyExposed {
                        HStack {
                            Localizable.KeyScreen.Label.hotkey.text
                                .frame(maxWidth: .infinity, alignment: .leading)
                            Spacer().frame(maxWidth: Spacing.medium)
                            Asset.exclamationRed.swiftUIImage
                        }
                        .padding()
                        .foregroundColor(Asset.accentRed300.swiftUIColor)
                        .font(Fontstyle.bodyM.base)
                        .background(
                            RoundedRectangle(cornerRadius: CornerRadius.small)
                                .stroke(Asset.fill12.swiftUIColor, lineWidth: 1)
                                .background(Asset.accentRed300.swiftUIColor.opacity(0.12))
                                .cornerRadius(CornerRadius.small)
                        )
                    }
                }
                .padding([.leading, .trailing], Spacing.large)
                .padding([.top, .bottom], 60)
                .background(Asset.backgroundSolidSystem.swiftUIColor)
            }
            .background(Asset.backgroundSolidSystem.swiftUIColor)
        }
    }
}

struct KeyDetailsPublicKeyView_Previews: PreviewProvider {
    static var previews: some View {
        HStack {
            VStack {
                KeyDetailsPublicKeyView(
                    navigation: NavigationCoordinator(),
                    viewModel: PreviewData.exampleKeyDetailsPublicKey()
                )
            }
            VStack {
                KeyDetailsPublicKeyView(
                    navigation: NavigationCoordinator(),
                    viewModel: PreviewData.exampleKeyDetailsPublicKey(isKeyExposed: false)
                )
            }
            VStack {
                KeyDetailsPublicKeyView(
                    navigation: NavigationCoordinator(),
                    viewModel: PreviewData.exampleKeyDetailsPublicKey(isRootKey: false)
                )
            }
            VStack {
                KeyDetailsPublicKeyView(
                    navigation: NavigationCoordinator(),
                    viewModel: PreviewData.exampleKeyDetailsPublicKey(isKeyExposed: false, isRootKey: false)
                )
            }
        }
        .previewLayout(.sizeThatFits)
        .preferredColorScheme(.dark)
    }
}

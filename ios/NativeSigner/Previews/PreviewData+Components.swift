//
//  PreviewData+Components.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 13/09/2022.
//

import Foundation

extension PreviewData {
    static let qrCodeContainerViewModel = QRCodeContainerViewModel(
        qrCode: PreviewData.exampleQRCode
    )

    static let qrCodeAddressFooterViewModel = QRCodeAddressFooterViewModel(
        identicon: PreviewData.exampleIdenticon,
        path: "//polkadot//path",
        network: "Polkadot",
        base58: "5F3sa2TJAWMqDhXG6jhV4N8ko9SxwGy8TpaNS1repo5EYjQX"
    )

    static let qrCodeRootFooterViewModel = QRCodeRootFooterViewModel(
        keyName: "Staking",
        base58: "5F3sa2TJAWMqDhXG6jhV4N8ko9SxwGy8TpaNS1repo5EYjQX"
    )
}

extension PreviewData {
    static let exampleExportPrivateKey = ExportPrivateKeyViewModel(
        qrCode: qrCodeContainerViewModel,
        addressFooter: qrCodeAddressFooterViewModel
    )

    static func exampleKeyDetailsPublicKey(
        isKeyExposed: Bool = true,
        isRootKey: Bool = true
    ) -> KeyDetailsPublicKeyViewModel {
        KeyDetailsPublicKeyViewModel(
            qrCode: qrCodeContainerViewModel,
            addressFooter: isRootKey ? nil : qrCodeAddressFooterViewModel,
            rootFooter: isRootKey ? qrCodeRootFooterViewModel : nil,
            isKeyExposed: isKeyExposed,
            isRootKey: isRootKey
        )
    }
}

extension PreviewData {
    static let seedPhraseViewModel = SeedPhraseViewModel(
        seedPhrase: """
        awesome change room lottery song useless hurdle dry educate type debate
         season give exact gift push bid rich atom system pig put welcome exit
        """
    )
}

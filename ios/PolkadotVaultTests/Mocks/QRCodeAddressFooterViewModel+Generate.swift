//
//  QRCodeAddressFooterViewModel+Generate.swift
//  PolkadotVaultTests
//
//  Created by Krzysztof Rodak on 08/12/2023.
//

import Foundation
@testable import PolkadotVault

extension ExportPrivateKeyViewModel {
    static func generate(
        qrCode: QrData = QrData.generate(),
        addressFooter: QRCodeAddressFooterViewModel = QRCodeAddressFooterViewModel.generate()
    ) -> ExportPrivateKeyViewModel {
        ExportPrivateKeyViewModel(
            qrCode: qrCode,
            addressFooter: addressFooter
        )
    }
}

extension QRCodeAddressFooterViewModel {
    static func generate(
        identicon: Identicon = Identicon.generate(),
        networkLogo: String = "defaultNetworkLogo.png",
        base58: String = "defaultBase58"
    ) -> QRCodeAddressFooterViewModel {
        QRCodeAddressFooterViewModel(
            identicon: identicon,
            networkLogo: networkLogo,
            base58: base58
        )
    }
}

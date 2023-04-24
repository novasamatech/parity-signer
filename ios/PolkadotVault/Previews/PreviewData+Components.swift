//
//  PreviewData+Components.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 13/09/2022.
//

import Foundation

extension PreviewData {
    static let qrCodeContainerViewModel = QrData.regular(data: PreviewData.exampleQRCode)

    static let animatedQrCodeViewModel = AnimatedQRCodeViewModel(
        qrCodes: [PreviewData.exampleQRCode, PreviewData.exampleQRCode]
    )

    static let qrCodeAddressFooterViewModel = QRCodeAddressFooterViewModel(
        identicon: .svg(image: PreviewData.exampleIdenticon),
        networkLogo: "polkadot",
        base58: "5F3sa2TJAWMqDhXG6jhV4N8ko9SxwGy8TpaNS1repo5EYjQX"
    )

    static let qrCodeAddressFooterViewModelNoPath = QRCodeAddressFooterViewModel(
        identicon: .svg(image: PreviewData.exampleIdenticon),
        networkLogo: "polkadot",
        base58: "5F3sa2TJAWMqDhXG6jhV4N8ko9SxwGy8TpaNS1repo5EYjQX"
    )

    static let qrCodeAddressFooterViewModelVeryLongPath = QRCodeAddressFooterViewModel(
        identicon: .svg(image: PreviewData.exampleIdenticon),
        networkLogo: "polkadot",
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
        .init(
            qrCodes: [],
            footer: PreviewData.qrCodeAddressFooterViewModel,
            isKeyExposed: isKeyExposed,
            isRootKey: isRootKey,
            networkTitle: "Polkadot",
            networkLogo: "polkadot",
            keySetName: "My Key Set",
            path: "//polkadot",
            hasPassword: true
        )
    }
}

extension PreviewData {
    static let seedPhraseViewModel = SeedPhraseViewModel(
        seedPhrase: """
        awesome change room lottery song useless elephant dry educate type debate
         season give exact gift push bid rich atom system pig put welcome exit
        """
    )
}

extension PreviewData {
    static let exampleKeySummary = KeySummaryViewModel(
        keyName: "Main Polkadot",
        base58: "15322Gsc678...0HA04H0A"
    )

    static let exampleDerivedKeyOverview = DerivedKeyOverviewViewModel(
        identicon: .svg(image: PreviewData.exampleIdenticon),
        path: "//polkadot",
        hasPassword: false,
        network: "Polkadot",
        networkLogo: "polkadot"
    )

    static let exampleDerivedKeyOverviews: [DerivedKeyOverviewViewModel] = [
        DerivedKeyOverviewViewModel(
            identicon: .svg(image: PreviewData.exampleIdenticon),
            path: "",
            hasPassword: false,
            network: "Kusama",
            networkLogo: "kusama"
        ),
        DerivedKeyOverviewViewModel(
            identicon: .svg(image: PreviewData.exampleIdenticon),
            path: "//polkadot",
            hasPassword: false,
            network: "Polkadot",
            networkLogo: "polkadot"
        ),
        DerivedKeyOverviewViewModel(
            identicon: .svg(image: PreviewData.exampleIdenticon),
            path: "//astar",
            hasPassword: false,
            network: "Astar",
            networkLogo: "astar"
        ),
        DerivedKeyOverviewViewModel(
            identicon: .svg(image: PreviewData.exampleIdenticon),
            path: "//kusama",
            hasPassword: true,
            network: "Kusama",
            networkLogo: "kusama"
        ),
        DerivedKeyOverviewViewModel(
            identicon: .svg(image: PreviewData.exampleIdenticon),
            path: "//kusama//verylongpathsolongthatmightbemultilineandhaspasswordtoo",
            hasPassword: true,
            network: "Kusama",
            networkLogo: "kusama"
        )
    ]

    static let exampleBackupViewModel = BackupModalViewModel(
        header: exampleKeySummary,
        derivedKeys: exampleDerivedKeyOverviews,
        seedPhrase: seedPhraseViewModel
    )

    static let exampleSettingsBackupViewModel = SettingsBackupViewModel(
        keyName: "Main Polkadot",
        seedPhrase: seedPhraseViewModel
    )
}

extension PreviewData {
    static let exampleExportMultipleKeysModal = ExportMultipleKeysModalViewModel(
        selectedItems: .keySets(KeySetListViewModelBuilder().build(for: PreviewData.mseeds).list),
        count: mseeds.seedNameCards.count
    )
}

extension PreviewData {
    static let transactionSummary: TransactionSummaryModel = .init(
        pallet: "Balances",
        method: "transfer_keep_alive",
        destination: "1219xC79CXV31543DDXoQMjuA",
        value: "0.2 WND"
    )

    static let transactionSignature: TransactionSignatureRenderable = .init(
        path: "//polkadot//1",
        name: "Parity Keys",
        network: "polkadot",
        base58: "1219xC79CXV31543DDXoQMjuA",
        identicon: .svg(image: PreviewData.exampleIdenticon)
    )
}

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

    static let animatedQrCodeViewModel = AnimatedQRCodeViewModel(
        qrCodes: [PreviewData.exampleQRCode, PreviewData.exampleQRCode]
    )

    static let qrCodeAddressFooterViewModel = QRCodeAddressFooterViewModel(
        identicon: PreviewData.exampleIdenticon,
        rootKeyName: "Dotsama parachains",
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
            footer: isRootKey ? .root(qrCodeRootFooterViewModel) : .address(qrCodeAddressFooterViewModel),
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

extension PreviewData {
    static let exampleKeySummary = KeySummaryViewModel(
        keyName: "Main Polkadot",
        base58: "15322Gsc678...0HA04H0A"
    )

    static let exampleDerivedKeyOverview = DerivedKeyOverviewViewModel(
        identicon: PreviewData.exampleIdenticon,
        path: "// polkadot",
        hasPassword: false
    )

    static let exampleDerivedKeyOverviews: [DerivedKeyOverviewViewModel] = [
        DerivedKeyOverviewViewModel(
            identicon: PreviewData.exampleIdenticon,
            path: "// polkadot",
            hasPassword: false
        ),
        DerivedKeyOverviewViewModel(
            identicon: PreviewData.exampleIdenticon,
            path: "// kusama",
            hasPassword: false
        ),
        DerivedKeyOverviewViewModel(
            identicon: PreviewData.exampleIdenticon,
            path: "// astar",
            hasPassword: true
        )
    ]

    static let exampleBackupViewModel = BackupModalViewModel(
        header: exampleKeySummary,
        derivedKeys: exampleDerivedKeyOverviews,
        seedPhrase: seedPhraseViewModel,
        qrCode: qrCodeContainerViewModel
    )
}

extension PreviewData {
    static let exampleExportMultipleKeysModal = ExportMultipleKeysModalViewModel(
        selectedItems: .keySets(KeySetListViewModelBuilder().build(for: PreviewData.mseeds).list),
        seedNames: mseeds.seedNameCards.map(\.seedName)
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
        base58: "1219xC79CXV31543DDXoQMjuA",
        identicon: PreviewData.exampleIdenticon
    )
}

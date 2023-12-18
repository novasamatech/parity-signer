//
//  ViewModels+Stubs.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 10/05/2023.
//

import Foundation

extension AnimatedQRCodeViewModel {
    static let stub: AnimatedQRCodeViewModel = .init(
        qrCodes: [
            Stubs.stubQRCode,
            Stubs.stubQRCode
        ]
    )
}

extension QRCodeAddressFooterViewModel {
    static let stub: QRCodeAddressFooterViewModel = .init(
        identicon: .stubIdenticon,
        networkLogo: "polkadot",
        base58: "5F3sa2TJAWMqDhXG6jhV4N8ko9SxwGy8TpaNS1repo5EYjQX"
    )
}

extension QRCodeRootFooterViewModel {
    static let stub: QRCodeRootFooterViewModel = .init(
        identicon: .stubJdenticon,
        base58: "5F3sa2TJAWMqDhXG6jhV4N8ko9SxwGy8TpaNS1repo5EYjQX"
    )
}

extension ExportPrivateKeyViewModel {
    static let stub: ExportPrivateKeyViewModel = .init(
        qrCode: .stubRegular,
        addressFooter: .stub
    )
}

extension KeyDetailsPublicKeyViewRenderable {
    static let stub: KeyDetailsPublicKeyViewRenderable = .init(
        qrCodes: [Stubs.stubQRCode],
        footer: .stub,
        isKeyExposed: false,
        isRootKey: false,
        networkTitle: "Polkadot",
        networkLogo: "polkadot",
        keySetName: "My Key Set",
        path: "//polkadot",
        hasPassword: true
    )
}

extension SeedPhraseViewModel {
    static let stub: SeedPhraseViewModel = .init(
        seedPhrase: """
        awesome change room lottery song useless elephant dry educate type debate
         season give exact gift push bid rich atom system pig put welcome exit
        """
    )
}

extension KeySummaryViewModel {
    static let stub: KeySummaryViewModel = .init(
        keyName: "Main Polkadot",
        base58: "15322Gsc678...0HA04H0A"
    )
}

extension DerivedKeyOverviewViewModel {
    static let stub: DerivedKeyOverviewViewModel = .init(
        identicon: .stubIdenticon,
        path: "//polkadot",
        hasPassword: false,
        network: "Polkadot",
        networkLogo: "polkadot"
    )

    static let stubs: [DerivedKeyOverviewViewModel] = [
        .init(
            identicon: .stubIdenticon,
            path: "",
            hasPassword: false,
            network: "Kusama",
            networkLogo: "kusama"
        ),
        .init(
            identicon: .stubIdenticon,
            path: "//polkadot",
            hasPassword: false,
            network: "Polkadot",
            networkLogo: "polkadot"
        ),
        .init(
            identicon: .stubIdenticon,
            path: "//astar",
            hasPassword: false,
            network: "Astar",
            networkLogo: "astar"
        ),
        .init(
            identicon: .stubIdenticon,
            path: "//kusama",
            hasPassword: true,
            network: "Kusama",
            networkLogo: "kusama"
        ),
        .init(
            identicon: .stubIdenticon,
            path: "//kusama//verylongpathsolongthatmightbemultilineandhaspasswordtoo",
            hasPassword: true,
            network: "Kusama",
            networkLogo: "kusama"
        )
    ]
}

extension BackupModalViewModel {
    static let stub: BackupModalViewModel = .init(
        header: .stub,
        derivedKeys: DerivedKeyOverviewViewModel.stubs,
        seedPhrase: .stub
    )
}

extension SettingsBackupViewModel {
    static let stub: SettingsBackupViewModel = .init(
        keyName: "Main Polkadot",
        seedPhrase: .stub
    )
}

extension ExportMultipleKeysModalViewModel {
    static let stub: ExportMultipleKeysModalViewModel = .init(
        keyName: "key",
        key: .stub,
        derivedKeys: [],
        count: 3
    )
}

extension TransactionSummaryModel {
    static let stub: TransactionSummaryModel = .init(
        pallet: "Balances",
        method: "transfer_keep_alive",
        destination: "1219xC79CXV31543DDXoQMjuA",
        value: "0.2 WND"
    )
}

extension TransactionSignatureRenderable {
    static let stub: TransactionSignatureRenderable = .init(
        path: "//polkadot//1",
        name: "Parity Keys",
        network: "polkadot",
        base58: "1219xC79CXV31543DDXoQMjuA",
        identicon: .stubIdenticon
    )
}

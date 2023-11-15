//
//  Stubs+Signer.swift
//  PolkadotVault
//
//  Created by Krzysztof Rodak on 10/05/2023.
//

import Foundation

extension Address {
    static let stub: Address = .init(
        path: "//polkadot",
        hasPwd: true,
        identicon: .stubIdenticon,
        seedName: "Polkadot Vault",
        secretExposed: false
    )
}

extension MKeysCard {
    static let stub: MKeysCard = .init(
        address: .stub,
        addressKey: "",
        base58: "",
        swiped: false
    )
}

extension MscNetworkInfo {
    static let stub: MscNetworkInfo = .init(
        networkTitle: "Polkadot",
        networkLogo: "polkadot",
        networkSpecsKey: "polkadot"
    )
}

extension MKeyAndNetworkCard {
    static let stub: MKeyAndNetworkCard = .init(
        key: .stub,
        network: .stub
    )

    static let stubs: [MKeyAndNetworkCard] = [.stub, .stub]
}

extension MAddressCard {
    static let stub: MAddressCard = .init(
        base58: "",
        addressKey: "01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e",
        address: .stub
    )
}

extension MKeysNew {
    static let stub: MKeysNew = .init(
        root: .stub,
        set: MKeyAndNetworkCard.stubs
    )
}

extension MKeyDetails {
    static let stub: MKeyDetails = .init(
        qr: .stubRegular,
        pubkey: "",
        networkInfo: .stub,
        address: .stub,
        base58: ""
    )
}

extension SeedNameCard {
    static let stub: SeedNameCard = .init(
        seedName: "name",
        identicon: .stubIdenticon,
        usedInNetworks: ["polkadot", "kusama", "westend"],
        derivedKeysCount: 3
    )

    static let stubs: [SeedNameCard] = [
        .init(
            seedName: "aaaa",
            identicon: .stubIdenticon,
            usedInNetworks: ["polkadot", "westend"],
            derivedKeysCount: 3
        ),
        .init(
            seedName: "bbbb",
            identicon: .stubIdenticon,
            usedInNetworks: ["polkadot", "kusama"],
            derivedKeysCount: 0
        ),
        .init(
            seedName: "cccc",
            identicon: .stubIdenticon,
            usedInNetworks: ["kusama", "westend"],
            derivedKeysCount: 1
        ),
        .init(
            seedName: "dddd",
            identicon: .stubIdenticon,
            usedInNetworks: ["polkadot", "kusama", "westend"],
            derivedKeysCount: 4
        ),
        .init(
            seedName: "eeee",
            identicon: .stubIdenticon,
            usedInNetworks: ["polkadot", "kusama", "westend"],
            derivedKeysCount: 15
        ),
        .init(
            seedName: "ffff",
            identicon: .stubIdenticon,
            usedInNetworks: ["polkadot", "kusama", "westend"],
            derivedKeysCount: 1
        ),
        .init(
            seedName: "Really long name that probably shouldn't fit into single line",
            identicon: .stubIdenticon,
            usedInNetworks: ["polkadot", "westend"],
            derivedKeysCount: 0
        )
    ]
}

extension MSeeds {
    static let stub: MSeeds = .init(
        seedNameCards: SeedNameCard.stubs
    )
}

extension MTransaction {
    static let stub: MTransaction = .init(
        content: .stub,
        ttype: .sign,
        authorInfo: .stub,
        networkInfo: .stub
    )
}

extension TransactionCardSet {
    static let stub: TransactionCardSet = .init(
        author: nil,
        error: nil,
        extensions: TransactionCard.stubsExtensions,
        importingDerivations: nil,
        message: nil,
        meta: nil,
        method: TransactionCard.stubsMethod,
        newSpecs: nil,
        verifier: nil,
        warning: nil,
        typesInfo: nil
    )
}

extension TransactionCard {
    static let stub: TransactionCard = .init(
        index: 9,
        indent: 0,
        card: .tipCard(f: .init(amount: "0", units: "pWND"))
    )

    static let stubsExtensions: [TransactionCard] = [
        .init(index: 7, indent: 0, card: .eraMortalCard(f: .init(era: "Mortal", phase: "1", period: "64"))),
        .init(index: 8, indent: 0, card: .nonceCard(f: "0")),
        .init(index: 9, indent: 0, card: .tipCard(f: .init(amount: "0", units: "pWND"))),
        .init(index: 10, indent: 0, card: .nameVersionCard(f: .init(name: "westend", version: "9320"))),
        .init(index: 11, indent: 0, card: .txSpecCard(f: "14")),
        .init(
            index: 12,
            indent: 0,
            card: .blockHashCard(f: "66721224f39e6c5250b5ab65e8b0ce334f354358244afb89c68fc2c05e1db38b")
        )
    ]

    static let stubsMethod: [TransactionCard] = [
        .init(index: 0, indent: 0, card: .palletCard(f: "Balance")),
        .init(
            index: 1,
            indent: 1,
            card: .callCard(f: .init(
                methodName: "transfer_keep_alive",
                docs: Stubs.stubMarkdownDocs
            ))
        ),
        .init(
            index: 2,
            indent: 2,
            card: .fieldNameCard(f: .init(
                name: "dest",
                docsFieldName: "",
                pathType: "sp_runtime >> multiaddress >> MultiAddress",
                docsType: ""
            ))
        ),
        .init(index: 3, indent: 3, card: .enumVariantNameCard(f: .init(name: "Id", docsEnumVariant: ""))),
        .init(
            index: 4,
            indent: 4,
            card: .idCard(f: .init(
                base58: "5FeSzkpTHV9N86kj61QLVaYU7pndHuCD7Cjj3zyzUhxxKZ5i",
                identicon: .stubIdenticon
            ))
        ),
        .init(
            index: 5,
            indent: 2,
            card: .fieldNameCard(f: .init(name: "value", docsFieldName: "", pathType: "", docsType: ""))
        ),
        .init(index: 6, indent: 3, card: .balanceCard(f: .init(amount: "2.0", units: "WND")))
    ]
}

extension MmNetwork {
    static let stub: MmNetwork = .init(
        key: "polkadot",
        title: "Polkadot",
        logo: "polkadot",
        order: 1,
        pathId: "//polkadot"
    )

    static let stubList: [MmNetwork] = [
        MmNetwork(key: "polkadot", title: "Polkadot", logo: "polkadot", order: 1, pathId: "//polkadot"),
        MmNetwork(key: "kusama", title: "Kusama", logo: "kusama", order: 2, pathId: "//kusama"),
        MmNetwork(key: "westend", title: "Westend", logo: "westend", order: 3, pathId: "//westend"),
        MmNetwork(key: "astar", title: "Astar", logo: "astar", order: 4, pathId: "//astar")
    ]
}

extension MEnterPassword {
    static let stub: MEnterPassword = .init(
        authorInfo: .stub,
        networkInfo: .stub,
        counter: 2
    )
}

extension MSignatureReady {
    static let stub: MSignatureReady = .init(
        signatures: [.stubRegular]
    )
}

extension MVerifierDetails {
    static let stub: MVerifierDetails = .init(
        publicKey: "publicKey",
        identicon: .stubIdenticon,
        encryption: "sh29919"
    )
}

extension MscEnumVariantName {
    static let stub: MscEnumVariantName = .init(
        name: "Name",
        docsEnumVariant: Stubs.stubMarkdownDocs
    )
}

extension MscCall {
    static let stub: MscCall = .init(
        methodName: "Method name",
        docs: Stubs.stubMarkdownDocs
    )
}

extension MscId {
    static let stub: MscId = .init(
        base58: "base58",
        identicon: .stubIdenticon
    )
}

extension MscFieldNumber {
    static let stub: MscFieldNumber = .init(
        number: "number",
        docsFieldNumber: "docsFieldNumber",
        pathType: "pathType",
        docsType: Stubs.stubMarkdownDocs
    )
}

extension SeedKeysPreview {
    static let stub: SeedKeysPreview = .init(
        name: "Derivation 1",
        multisigner: ["long address", "encryption"],
        derivedKeys: DerivedKeyPreview.stubList
    )

    static let stubList: [SeedKeysPreview] = [
        .stub
    ]
}

extension DerivedKeyPreview {
    static let stub: DerivedKeyPreview = .init(
        address: "address",
        derivationPath: "//kusama",
        encryption: .ed25519,
        genesisHash: .init([3, 4, 5]),
        identicon: .stubIdenticon,
        hasPwd: nil,
        networkTitle: "Kusama",
        status: .alreadyExists
    )

    static let stubList: [DerivedKeyPreview] = [
        .init(
            address: "address",
            derivationPath: "//kusama",
            encryption: .ed25519,
            genesisHash: .init([3, 4, 5]),
            identicon: .stubIdenticon,
            hasPwd: nil,
            networkTitle: "Kusama",
            status: .alreadyExists
        ),
        .init(
            address: "GD5434gFGFD543Dgdf",
            derivationPath: "//westendMain",
            encryption: .ed25519,
            genesisHash: .init([3, 4, 5]),
            identicon: .stubIdenticon,
            hasPwd: true,
            networkTitle: "Westend",
            status: .invalid(errors: [.badFormat])
        ),
        .init(
            address: "address",
            derivationPath: "//polka",
            encryption: .ed25519,
            genesisHash: .init([3, 4, 5]),
            identicon: .stubIdenticon,
            hasPwd: false,
            networkTitle: "Polkadot",
            status: .importable
        ),
        .init(
            address: "address",
            derivationPath: "//polkadot//parachains",
            encryption: .ed25519,
            genesisHash: .init([3, 4, 5]),
            identicon: .stubIdenticon,
            hasPwd: true,
            networkTitle: "Polkadot",
            status: .importable
        ),
        .init(
            address: "address",
            derivationPath: "",
            encryption: .ed25519,
            genesisHash: .init([3, 4, 5]),
            identicon: .stubIdenticon,
            hasPwd: false,
            networkTitle: nil,
            status: .importable
        ),
        .init(
            address: "address",
            derivationPath: "//kusama//verylongpathsolongitrequirestwolinesoftextormaybeevenmoremaybethree",
            encryption: .ed25519,
            genesisHash: .init([3, 4, 5]),
            identicon: .stubIdenticon,
            hasPwd: true,
            networkTitle: nil,
            status: .importable
        )
    ]
}

extension MTypesInfo {
    static let stub: MTypesInfo = .init(
        typesOnFile: false,
        typesHash: "typesHas",
        typesIdPic: .stubIdenticon
    )
}

extension NetworkSpecs {
    static let stub: NetworkSpecs = .init(
        base58prefix: 231,
        color: "black",
        decimals: 4,
        encryption: .sr25519,
        genesisHash: H256(repeating: 3, count: 4),
        logo: "polkadot",
        name: "polkadot",
        pathId: "1",
        secondaryColor: "pink",
        title: "Polka",
        unit: "DOT"
    )
}

extension MMetadataRecord {
    static let stub: MMetadataRecord = .init(
        specname: "Westend",
        specsVersion: "9230",
        metaHash: "5DCmwXp8XLzSMUyE4uhJMKV4vwvsWqqBYFKJq38CW53VHEVq",
        metaIdPic: .stubIdenticon
    )
}

extension MscTxSpecPlain {
    static let stub: MscTxSpecPlain = .init(
        networkGenesisHash: .init([3, 4, 5]),
        version: "9230",
        txVersion: "tx9230"
    )
}

extension DdDetail {
    static let stub: DdDetail = .init(
        base58: "5FeSzkpTHV9N86kj61QLVaYU7pndHuCD7Cjj3zyzUhxxKZ5i",
        path: "//polkadot",
        networkLogo: "polkadot",
        networkSpecsKey: "5DCmwXp8XLzSMUyE4uhJMKV4vwvsWqqBYFKJq38CW53VHEVq",
        identicon: .stubIdenticon
    )
}

extension DdKeySet {
    static let stub: DdKeySet = .init(seedName: "seed name", derivations: [.stub])
}

extension DdPreview {
    static let stub: DdPreview = .init(
        qr: [.stubRegular],
        keySet: .stub,
        isSomeAlreadyImported: true,
        isSomeNetworkMissing: true
    )
}

extension MRawKey {
    static let stub: MRawKey = .init(
        address: .stub,
        addressKey: "addressKey",
        publicKey: "publicKey",
        networkLogo: "polkadot"
    )
}

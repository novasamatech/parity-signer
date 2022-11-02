//
//  PreviewData+Rust.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 26/09/2022.
//

import Foundation

extension PreviewData {
    static let mkeys = MKeys(
        set: [],
        root: .init(
            address: .init(
                path: "",
                hasPwd: false,
                identicon: [],
                seedName: "",
                secretExposed: false
            ),
            addressKey: "",
            base58: "",
            swiped: false,
            multiselect: false
        ),
        network: .init(title: "", logo: ""),
        multiselectMode: false,
        multiselectCount: ""
    )

    static let mKeyAndNetworkCard = MKeyAndNetworkCard(key: .init(
        address: .init(
            path: "",
            hasPwd: false,
            identicon: [],
            seedName: "",
            secretExposed: false
        ),
        addressKey: "",
        base58: "",
        swiped: false,
        multiselect: false
    ), network: .init(networkTitle: "", networkLogo: "", networkSpecsKey: ""))

    static let mKeyNew = MKeysNew(
        root: MAddressCard(
            base58: "",
            address: .init(
                path: "",
                hasPwd: false,
                identicon: PreviewData.exampleIdenticon,
                seedName: "",
                secretExposed: false
            ),
            multiselect: false
        ),
        set: []
    )

    static let mkeyDetails = MKeyDetails(
        qr: [],
        pubkey: "",
        networkInfo: .init(networkTitle: "", networkLogo: "", networkSpecsKey: ""),
        address: .init(
            path: "",
            hasPwd: false,
            identicon: [],
            seedName: "",
            secretExposed: false
        ),
        base58: "",
        multiselect: nil
    )

    static let exampleErrorMessage =
        // swiftlint:disable:next line_length
        "The Westend network current metadata does not correspond to the one you use in the app. Please update it to sign the transaction."

    static let seedNameCard = SeedNameCard(
        seedName: "aaaa",
        identicon: PreviewData.exampleIdenticon,
        derivedKeysCount: 3
    )

    static let mseeds = MSeeds(
        seedNameCards: [
            SeedNameCard(
                seedName: "aaaa",
                identicon: PreviewData.exampleIdenticon,
                derivedKeysCount: 3
            ),
            SeedNameCard(
                seedName: "bbbb",
                identicon: PreviewData.exampleIdenticon,
                derivedKeysCount: 0
            ),
            SeedNameCard(
                seedName: "cccc",
                identicon: PreviewData.exampleIdenticon,
                derivedKeysCount: 1
            ),
            SeedNameCard(
                seedName: "dddd",
                identicon: PreviewData.exampleIdenticon,
                derivedKeysCount: 4
            ),
            SeedNameCard(
                seedName: "eeee",
                identicon: PreviewData.exampleIdenticon,
                derivedKeysCount: 15
            ),
            SeedNameCard(
                seedName: "ffff",
                identicon: PreviewData.exampleIdenticon,
                derivedKeysCount: 1
            ),
            SeedNameCard(
                seedName: "gggg",
                identicon: PreviewData.exampleIdenticon,
                derivedKeysCount: 0
            )
        ]
    )

    static let networks: [Network] = [
        Network(key: "polkadot", logo: "polkadot", order: 1, selected: false, title: "Polkadot"),
        Network(key: "kusama", logo: "kusama", order: 2, selected: false, title: "Kusama"),
        Network(key: "westend", logo: "westend", order: 3, selected: false, title: "Westend"),
        Network(key: "astar", logo: "astar", order: 4, selected: false, title: "Astar")
    ]
}

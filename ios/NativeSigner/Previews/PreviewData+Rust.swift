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
            seedName: "",
            identicon: [],
            addressKey: "",
            base58: "",
            swiped: false,
            multiselect: false,
            secretExposed: false
        ),
        network: .init(title: "", logo: ""),
        multiselectMode: false,
        multiselectCount: ""
    )
    static let mkeyDetails = MKeyDetails(
        qr: [],
        pubkey: "",
        networkInfo: .init(networkTitle: "", networkLogo: "", networkSpecsKey: ""),
        address: .init(
            base58: "",
            path: "",
            hasPwd: false,
            identicon: [],
            seedName: "",
            multiselect: nil,
            secretExposed: false
        )
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
}

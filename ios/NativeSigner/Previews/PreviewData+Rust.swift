//
//  PreviewData+Rust.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 26/09/2022.
//

import Foundation

extension PreviewData {
    static let address: Address = .init(
        path: "// polkadot",
        hasPwd: true,
        identicon: .svg(image: PreviewData.exampleIdenticon),
        seedName: "main account",
        secretExposed: false
    )
    static let base58 = "5F3sa2TJAWMqDhXG6jhV4N8ko9SxwGy8TpaNS1repo5EYjQX"
    static let publicKey = "15Gsc678654FDSG0HA04H0A"
    static let mkeys = MKeys(
        set: [],
        root: .init(
            address: .init(
                path: "",
                hasPwd: false,
                identicon: .svg(image: PreviewData.exampleIdenticon),
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
            identicon: .svg(image: PreviewData.exampleIdenticon),
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
                identicon: .svg(image: PreviewData.exampleIdenticon),
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
            identicon: .svg(image: PreviewData.exampleIdenticon),
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
        identicon: .svg(image: PreviewData.exampleIdenticon),
        derivedKeysCount: 3
    )

    static let mseeds = MSeeds(
        seedNameCards: [
            SeedNameCard(
                seedName: "aaaa",
                identicon: .svg(image: PreviewData.exampleIdenticon),
                derivedKeysCount: 3
            ),
            SeedNameCard(
                seedName: "bbbb",
                identicon: .svg(image: PreviewData.exampleIdenticon),
                derivedKeysCount: 0
            ),
            SeedNameCard(
                seedName: "cccc",
                identicon: .svg(image: PreviewData.exampleIdenticon),
                derivedKeysCount: 1
            ),
            SeedNameCard(
                seedName: "dddd",
                identicon: .svg(image: PreviewData.exampleIdenticon),
                derivedKeysCount: 4
            ),
            SeedNameCard(
                seedName: "eeee",
                identicon: .svg(image: PreviewData.exampleIdenticon),
                derivedKeysCount: 15
            ),
            SeedNameCard(
                seedName: "ffff",
                identicon: .svg(image: PreviewData.exampleIdenticon),
                derivedKeysCount: 1
            ),
            SeedNameCard(
                seedName: "gggg",
                identicon: .svg(image: PreviewData.exampleIdenticon),
                derivedKeysCount: 0
            )
        ]
    )
}

extension PreviewData {
    static let exampleMarkdownDocs: String =
        // swiftlint:disable:next line_length
        "53616d6520617320746865205b607472616e73666572605d2063616c6c2c206275742077697468206120636865636b207468617420746865207472616e736665722077696c6c206e6f74206b696c6c207468650a6f726967696e206163636f756e742e0a0a393925206f66207468652074696d6520796f752077616e74205b607472616e73666572605d20696e73746561642e0a0a5b607472616e73666572605d3a207374727563742e50616c6c65742e68746d6c236d6574686f642e7472616e73666572"

    static let signTransaction = MTransaction(
        content: .init(
            author: nil,
            error: nil,
            extensions: [
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
            ],
            importingDerivations: nil,
            message: nil,
            meta: nil,
            method: [
                .init(index: 0, indent: 0, card: .palletCard(f: "Balance")),
                .init(
                    index: 1,
                    indent: 1,
                    card: .callCard(f: .init(
                        methodName: "transfer_keep_alive",
                        docs: PreviewData.exampleMarkdownDocs
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
                        identicon: .svg(image: PreviewData.exampleIdenticon)
                    ))
                ),
                .init(
                    index: 5,
                    indent: 2,
                    card: .fieldNameCard(f: .init(name: "value", docsFieldName: "", pathType: "", docsType: ""))
                ),
                .init(index: 6, indent: 3, card: .balanceCard(f: .init(amount: "2.0", units: "WND")))

            ],
            newSpecs: nil,
            verifier: nil,
            warning: nil,
            typesInfo: nil
        ),
        ttype: .sign,
        authorInfo: .init(
            base58: "5ELtQSR8igkgpwCNGKkoGiepCWS6m558T9mchaMax7zwVWUz",
            address: .init(
                path: "",
                hasPwd: true,
                identicon: .svg(image: PreviewData.exampleIdenticon),
                seedName: "Seed name",
                secretExposed: false
            ), multiselect: nil
        ),
        networkInfo: .init(
            networkTitle: "Westend",
            networkLogo: "westend",
            networkSpecsKey: "01e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e"
        )
    )
}

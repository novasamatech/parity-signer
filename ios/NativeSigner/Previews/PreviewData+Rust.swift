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
}

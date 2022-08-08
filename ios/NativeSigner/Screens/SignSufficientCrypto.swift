//
//  SignSufficientCrypto.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 21.12.2021.
//

import SwiftUI

struct SignSufficientCrypto: View {
    let content: MSignSufficientCrypto
    let navigationRequest: NavigationRequest
    let getSeed: (String) -> String
    var body: some View {
        VStack {
            Text("Select key for signing")
            ScrollView {
                LazyVStack {
                    ForEach(content.identities, id: \.addressKey) { keyrecord in
                        Button(
                            action: {
                                let seedPhrase = getSeed(keyrecord.seedName)
                                if !seedPhrase.isEmpty {
                                    navigationRequest(.init(
                                        action: .goForward,
                                        details: keyrecord.addressKey,
                                        seedPhrase: seedPhrase
                                    ))
                                }
                            },
                            label: {
                                AddressCard(
                                    address: Address(
                                        base58: keyrecord.publicKey,
                                        path: keyrecord.path,
                                        hasPwd: keyrecord.hasPwd,
                                        identicon: keyrecord.identicon,
                                        seedName: keyrecord.seedName,
                                        multiselect: nil
                                    )
                                )
                            }
                        )
                    }
                }
            }
        }
    }
}

// struct SignSufficientCrypto_Previews: PreviewProvider {
// static var previews: some View {
// SignSufficientCrypto()
// }
// }

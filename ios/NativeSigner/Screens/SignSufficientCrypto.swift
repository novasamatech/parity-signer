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
            Localizable.selectKeyForSigning.text
            ScrollView {
                LazyVStack {
                    ForEach(content.identities, id: \.addressKey) { keyrecord in
                        Button(
                            action: {
                                let seedPhrase = getSeed(keyrecord.address.seedName)
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
                                    card: MAddressCard(
                                        base58: keyrecord.publicKey,
                                        address: keyrecord.address,
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

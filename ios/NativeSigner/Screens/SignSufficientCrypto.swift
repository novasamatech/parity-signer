//
//  SignSufficientCrypto.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 21.12.2021.
//

import SwiftUI

struct SignSufficientCrypto: View {
    let content: MSignSufficientCrypto
    @EnvironmentObject var navigation: NavigationCoordinator
    let seedsMediator: SeedsMediating = ServiceLocator.seedsMediator
    var body: some View {
        VStack {
            Localizable.selectKeyForSigning.text
            ScrollView {
                LazyVStack {
                    ForEach(content.identities, id: \.addressKey) { keyrecord in
                        Button(
                            action: {
                                let seedPhrase = seedsMediator.getSeed(seedName: keyrecord.address.seedName)
                                if !seedPhrase.isEmpty {
                                    navigation.perform(navigation: .init(
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
                                        addressKey: keyrecord.addressKey,
                                        address: keyrecord.address
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

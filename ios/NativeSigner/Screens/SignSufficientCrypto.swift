//
//  SignSufficientCrypto.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 21.12.2021.
//

import SwiftUI

struct SignSufficientCrypto: View {
    @EnvironmentObject var data: SignerDataModel
    var content: MSignSufficientCrypto
    var body: some View {
        VStack {
            Text("Select key for signing")
            ScrollView {
                
                LazyVStack {
                    ForEach(content.identities, id: \.addressKey) {keyrecord in
                        Button(action: {
                            let seedPhrase = data.getSeed(seedName: keyrecord.seedName)
                            if seedPhrase != "" {
                                data.pushButton(action: .goForward, details: keyrecord.addressKey, seedPhrase: seedPhrase)
                            }
                        }) {
                            AddressCard(address: Address(base58: keyrecord.publicKey, path: keyrecord.path, hasPwd: keyrecord.hasPwd, identicon: keyrecord.identicon, seedName: keyrecord.seedName, multiselect: nil))
                        }
                    }
                }
            }
        }
    }
}

/*
 struct SignSufficientCrypto_Previews: PreviewProvider {
 static var previews: some View {
 SignSufficientCrypto()
 }
 }
 */

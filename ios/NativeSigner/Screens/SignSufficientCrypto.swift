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
                    ForEach(content.getSortedKeys(), id: \.address_key) {keyrecord in
                        Button(action: {
                            let seedPhrase = data.getSeed(seedName: keyrecord.seed_name)
                            if seedPhrase != "" {
                                data.pushButton(buttonID: .GoForward, details: keyrecord.address_key, seedPhrase: seedPhrase)
                            }
                        }) {
                            AddressCard(address: keyrecord.intoAddress())
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

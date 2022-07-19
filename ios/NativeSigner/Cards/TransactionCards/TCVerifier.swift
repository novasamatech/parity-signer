//
//  TCVerifier.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 21.10.2021.
//

import SwiftUI

struct TCVerifier: View {
    var value: MVerifierDetails
    var body: some View {
        VStack {
            Text("VERIFIER CERTIFICATE").foregroundColor(Color("Text600"))
            HStack {
                Identicon(identicon: value.identicon)
                VStack(alignment: .leading) {
                    HStack {
                        Text("key:")
                            .foregroundColor(Color("Text600"))
                        Text(value.publicKey)
                            .foregroundColor(Color("Crypto400"))
                    }
                    HStack {
                        Text("crypto:")
                            .foregroundColor(Color("Text600"))
                        Text(value.encryption)
                            .foregroundColor(Color("Crypto400"))
                    }
                }
            }
        }
    }
}

/*
 struct TCVerifier_Previews: PreviewProvider {
 static var previews: some View {
 TCVerifier()
 }
 }*/

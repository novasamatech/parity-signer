//
//  TCVerifier.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 21.10.2021.
//

import SwiftUI

struct TCVerifier: View {
    var value: Verifier
    var body: some View {
        VStack {
            Text("VERIFIER CERTIFICATE").foregroundColor(Color("textMainColor"))
            VStack(alignment: .leading) {
                HStack {
                    Text("key:")
                        .foregroundColor(Color("textMainColor"))
                    Text(value.hex)
                        .foregroundColor(Color("cryptoColor"))
                }
                HStack {
                    Text("crypto:")
                        .foregroundColor(Color("textMainColor"))
                    Text(value.encryption)
                        .foregroundColor(Color("cryptoColor"))
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

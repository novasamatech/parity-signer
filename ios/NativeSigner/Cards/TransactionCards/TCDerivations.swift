//
//  TCDerivations.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 6.1.2022.
//

import SwiftUI

struct TCDerivations: View {
    let value: [String]
    var body: some View {
        HStack {
            VStack {
                Text("Importing derivations:").font(FBase(style: .h1)).foregroundColor(Color("Text600"))
                ForEach(value, id: \.self) {derivation in
                    HStack {
                        Text(derivation).font(FCrypto(style: .body2)).foregroundColor(Color("Crypto400"))
                        Spacer()
                    }
                }
            }
        }
    }
}

/*
 struct TCDerivations_Previews: PreviewProvider {
 static var previews: some View {
 TCDerivations()
 }
 }
 */

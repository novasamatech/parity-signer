//
//  TCEraMortalNonce.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 16.8.2021.
//

import SwiftUI

struct TCEraMortal: View {
    var eraMortal: EraMortal
    var body: some View {
        HStack {
            Spacer()
            VStack {
                Text("phase")
                    .foregroundColor(Color("Text400"))
                Text(eraMortal.phase)
                    .foregroundColor(Color("Text600"))
            }
            Spacer()
            VStack {
                Text("period")
                    .foregroundColor(Color("Text400"))
                Text(eraMortal.period)
                    .foregroundColor(Color("Text600"))
            }
            Spacer()
        }
    }
}

/*
struct TCEraMortalNonce_Previews: PreviewProvider {
    static var previews: some View {
        TCEraMortalNonce()
    }
}
*/

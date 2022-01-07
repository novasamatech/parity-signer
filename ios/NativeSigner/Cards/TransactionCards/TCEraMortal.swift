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
        VStack {
            TCNameValueTemplate(name: "phase", value: eraMortal.phase)
            TCNameValueTemplate(name: "period", value: eraMortal.period)
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

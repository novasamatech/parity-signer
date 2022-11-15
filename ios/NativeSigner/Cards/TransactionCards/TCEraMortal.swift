//
//  TCEraMortalNonce.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 16.8.2021.
//

import SwiftUI

struct TCEraMortal: View {
    var eraMortal: MscEraMortal
    var body: some View {
        VStack {
            TCNamedValueCard(name: Localizable.TCName.phase.string, value: eraMortal.phase)
            TCNamedValueCard(name: Localizable.TCName.period.string, value: eraMortal.period)
        }
    }
}

// struct TCEraMortalNonce_Previews: PreviewProvider {
// static var previews: some View {
// TCEraMortalNonce()
// }
// }

//
//  TCEraMortalNonce.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 16.8.2021.
//

import SwiftUI

struct TCEraMortal: View {
    var content: MscEraMortal
    var body: some View {
        VStack {
            TCNamedValueCard(name: Localizable.TCName.phase.string, value: content.phase)
            TCNamedValueCard(name: Localizable.TCName.period.string, value: content.period)
        }
    }
}

struct TCEraMortal_Previews: PreviewProvider {
    static var previews: some View {
        TCEraMortal(content: MscEraMortal(era: "era", phase: "phase", period: "period"))
    }
}

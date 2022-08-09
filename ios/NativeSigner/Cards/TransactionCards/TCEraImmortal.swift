//
//  TCEraImmortalNonce.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 16.8.2021.
//

import SwiftUI

struct TCEraImmortal: View {
    var body: some View {
        HStack {
            Localizable.immortalTransaction.text
                .foregroundColor(Asset.text400.swiftUIColor)
            Spacer()
        }
    }
}

// struct TCEraImmortalNonce_Previews: PreviewProvider {
// static var previews: some View {
// TCEraImmortalNonce()
// }
// }

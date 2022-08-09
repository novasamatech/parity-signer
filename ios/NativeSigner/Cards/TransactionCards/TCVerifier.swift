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
            Localizable.verifierCertificateUppercase.text
                .foregroundColor(Asset.text600.swiftUIColor)
            HStack {
                Identicon(identicon: value.identicon)
                VStack(alignment: .leading) {
                    HStack {
                        Localizable.key.text
                            .foregroundColor(Asset.text600.swiftUIColor)
                        Text(value.publicKey)
                            .foregroundColor(Asset.crypto400.swiftUIColor)
                    }
                    HStack {
                        Localizable.crypto.text
                            .foregroundColor(Asset.text600.swiftUIColor)
                        Text(value.encryption)
                            .foregroundColor(Asset.crypto400.swiftUIColor)
                    }
                }
            }
        }
    }
}

// struct TCVerifier_Previews: PreviewProvider {
// static var previews: some View {
// TCVerifier()
// }
// }

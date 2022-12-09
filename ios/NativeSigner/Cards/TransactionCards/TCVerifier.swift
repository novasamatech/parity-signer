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
        VStack(alignment: .leading, spacing: 0) {
            Localizable.verifierCertificate.text
                .font(PrimaryFont.bodyL.font)
                .foregroundColor(Asset.textAndIconsSecondary.swiftUIColor)
                .padding(.leading, Spacing.medium)
                .padding(.bottom, Spacing.extraExtraSmall)
            VStack {
                VStack(spacing: Spacing.small) {
                    VStack(alignment: .leading, spacing: Spacing.extraSmall) {
                        Localizable.Transaction.Verifier.Label.key.text
                            .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
                        Text(value.publicKey)
                            .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                    }
                    Divider()
                    VStack(alignment: .leading) {
                        HStack {
                            Localizable.Transaction.Verifier.Label.crypto.text
                                .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
                            Spacer()
                            Text(value.encryption)
                                .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                        }
                    }
                }
                .padding(Spacing.medium)
            }
            .background(Asset.fill6.swiftUIColor)
            .cornerRadius(CornerRadius.medium)
            .padding(.bottom, Spacing.extraSmall)
        }
    }
}

struct TCVerifier_Previews: PreviewProvider {
    static var previews: some View {
        TCVerifier(
            value: MVerifierDetails(
                publicKey: "5DCmwXp8XLzSMUyE4uhJMKV4vwvsWqqBYFKJq38CW53VHEVq",
                identicon: .svg(image: PreviewData.exampleIdenticon),
                encryption: "sr25519"
            )
        )
    }
}

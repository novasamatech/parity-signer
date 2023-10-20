//
//  TCVerifier.swift
//  Polkadot Vault
//
//  Created by Alexander Slesarev on 21.10.2021.
//

import SwiftUI

struct TCVerifier: View {
    var value: MVerifierDetails

    var body: some View {
        VStack(alignment: .leading, spacing: 0) {
            Localizable.Transaction.Verifier.Label.header.text
                .font(PrimaryFont.bodyL.font)
                .foregroundColor(.textAndIconsSecondary)
                .padding(.leading, Spacing.medium)
                .padding(.bottom, Spacing.extraExtraSmall)
            VStack {
                VStack(spacing: Spacing.small) {
                    VStack(alignment: .leading, spacing: Spacing.extraSmall) {
                        Localizable.Transaction.Verifier.Label.key.text
                            .foregroundColor(.textAndIconsTertiary)
                        Text(value.publicKey)
                            .foregroundColor(.textAndIconsPrimary)
                    }
                    Divider()
                    VStack(alignment: .leading) {
                        HStack {
                            Localizable.Transaction.Verifier.Label.crypto.text
                                .foregroundColor(.textAndIconsTertiary)
                            Spacer()
                            Text(value.encryption)
                                .foregroundColor(.textAndIconsPrimary)
                        }
                    }
                }
                .padding(Spacing.medium)
            }
            .background(.fill6)
            .cornerRadius(CornerRadius.medium)
            .padding(.bottom, Spacing.extraSmall)
        }
    }
}

#if DEBUG
    struct TCVerifier_Previews: PreviewProvider {
        static var previews: some View {
            TCVerifier(
                value: .stub
            )
        }
    }
#endif

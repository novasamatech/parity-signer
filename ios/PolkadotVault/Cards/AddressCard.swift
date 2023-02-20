//
//  IdentityCard.swift
//  Polkadot Vault
//
//  Created by Alexander Slesarev on 3.8.2021.
//

import SwiftUI

/// Card for showing any address.
/// Accepts Address object
/// Deprecated, used in: SufficientCryptoReady, SignSufficientCrypto
struct AddressCard: View {
    @EnvironmentObject private var data: SharedDataModel
    var card: MAddressCard
    @GestureState private var dragOffset = CGSize.zero
    let rowHeight: CGFloat = 28
    var body: some View {
        ZStack {
            RoundedRectangle(cornerRadius: 4).foregroundColor(Asset.backgroundSecondary.swiftUIColor).frame(height: 44)
            HStack {
                ZStack {
                    Identicon(identicon: card.address.identicon)
                }
                .frame(width: 30, height: 30)
                VStack(alignment: .leading) {
                    HStack {
                        Text(card.address.seedName).foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                            .font(PrimaryFont.labelM.font)
                        Text(card.address.path)
                        if card.address.hasPwd {
                            Localizable.Shared.Label.passwordedPathDelimeter.text
                                .foregroundColor(Asset.accentPink300.swiftUIColor)
                                .font(PrimaryFont.captionM.font)
                            Image(.lock)
                                .foregroundColor(Asset.accentPink300.swiftUIColor)
                                .font(PrimaryFont.captionM.font)
                        }
                    }.foregroundColor(Asset.accentPink300.swiftUIColor)
                        .font(PrimaryFont.captionM.font)
                    // Here we could have shortened base58 address when buttons are shown, but we don't need to
                    Text(card.base58.truncateMiddle(length: 8)).foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
                        .font(PrimaryFont.captionM.font)
                }
                Spacer()
            }.padding(.horizontal, 8)
        }
    }
}

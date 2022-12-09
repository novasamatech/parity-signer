//
//  SeedKeyCard.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 9.12.2021.
//

import SwiftUI

struct SeedKeyCard: View {
    @EnvironmentObject private var data: SignerDataModel
    var seedCard: MKeysCard
    var multiselectMode: Bool = false
    var body: some View {
        ZStack {
            RoundedRectangle(cornerRadius: 4)
                .foregroundColor(Asset.backgroundSecondary.swiftUIColor)
                .frame(height: 47)
            HStack {
                ZStack {
                    Identicon(identicon: seedCard.address.identicon)
                    if multiselectMode, !seedCard.base58.isEmpty {
                        VStack {
                            Spacer()
                            HStack {
                                Spacer()
                                (seedCard.multiselect ? Image(.checkmark, variants: [.circle, .fill]) : Image(.circle))
                                    .imageScale(.large)
                            }
                        }
                    }
                }.frame(width: 30, height: 30)
                VStack(alignment: .leading) {
                    Text(seedCard.address.seedName)
                        .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                        .font(PrimaryFont.labelM.font)
                    Text(seedCard.base58.truncateMiddle(length: 8))
                        .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
                        .font(PrimaryFont.captionM.font)
                }
                Spacer()
            }
            .padding(8)
        }
    }
}

// struct SeedKeyCard_Previews: PreviewProvider {
// static var previews: some View {
// SeedKeyCard()
// }
// }

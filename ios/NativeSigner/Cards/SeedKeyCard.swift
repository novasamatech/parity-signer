//
//  SeedKeyCard.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 9.12.2021.
//

import SwiftUI

struct SeedKeyCard: View {
    @EnvironmentObject private var data: SignerDataModel
    var seedCard: MSeedKeyCard
    var multiselectMode: Bool = false
    var body: some View {
        ZStack {
            RoundedRectangle(cornerRadius: 4)
                .foregroundColor(Asset.bg200.swiftUIColor)
                .frame(height: 47)
            HStack {
                ZStack {
                    Identicon(identicon: seedCard.identicon)
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
                    Text(seedCard.seedName)
                        .foregroundColor(Asset.text600.swiftUIColor)
                        .font(Fontstyle.subtitle1.base)
                    Text(seedCard.base58.truncateMiddle(length: 8))
                        .foregroundColor(Asset.text400.swiftUIColor)
                        .font(Fontstyle.body1.crypto)
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

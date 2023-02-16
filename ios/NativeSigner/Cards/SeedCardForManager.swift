//
//  AddressCardSelector.swift
//  Polkadot Vault
//
//  Created by Alexander Slesarev on 14.10.2021.
//

import SwiftUI

struct SeedCardForManager: View {
    @EnvironmentObject private var data: SharedDataModel
    var seedNameCard: SeedNameCard
    var body: some View {
        ZStack {
            RoundedRectangle(cornerRadius: 4)
                .foregroundColor(Asset.backgroundSecondary.swiftUIColor)
                .frame(height: 47)
            HStack {
                Identicon(identicon: seedNameCard.identicon)
                VStack(alignment: .leading) {
                    Text(seedNameCard.seedName)
                        .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                        .font(PrimaryFont.labelM.font)
                }
                Spacer()
            }
            .padding(8)
        }
    }
}

// struct AddressCardSelector_Previews: PreviewProvider {
// static var previews: some View {
// SeedCardForManager()
// }
// }

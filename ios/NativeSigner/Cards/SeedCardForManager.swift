//
//  AddressCardSelector.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 14.10.2021.
//

import SwiftUI

struct SeedCardForManager: View {
    @EnvironmentObject var data: SignerDataModel
    var seedNameCard: SeedNameCard
    var body: some View {
        ZStack {
            RoundedRectangle(cornerRadius: 4).foregroundColor(Asset.bg200.swiftUIColor).frame(height: 47)
            HStack {
                Identicon(identicon: seedNameCard.identicon)
                VStack(alignment: .leading) {
                    Text(seedNameCard.seedName)
                        .foregroundColor(Asset.text600.swiftUIColor)
                        .font(Fontstyle.subtitle1.base)
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

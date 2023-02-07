//
//  NetworkCard.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 19.7.2021.
//

import SwiftUI

struct NetworkCard: View {
    let title: String
    let logo: String
    var fancy: Bool = false
    var body: some View {
        ZStack {
            if fancy {
                RoundedRectangle(cornerRadius: 4)
                    .foregroundColor(Asset.backgroundSecondary.swiftUIColor)
                    .frame(height: 47)
            }
            HStack {
                NetworkLogoIcon(networkName: logo)
                Text(title).font(PrimaryFont.labelM.font)
                if fancy { Spacer() }
            }
            .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
            .frame(height: 36)
            .padding(.horizontal)
        }
    }
}

struct NetworkCard_Previews: PreviewProvider {
    static var previews: some View {
        NetworkCard(
            title: "Polkadot",
            logo: "polkadot",
            fancy: true
        )
        .previewLayout(.sizeThatFits)
    }
}

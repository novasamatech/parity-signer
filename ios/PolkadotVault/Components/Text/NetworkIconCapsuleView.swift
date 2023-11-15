//
//  NetworkIconCapsuleView.swift
//  PolkadotVault
//
//  Created by Krzysztof Rodak on 21/04/2023.
//

import SwiftUI

struct NetworkIconCapsuleView: View {
    let networkLogo: String
    let networkTitle: String

    var body: some View {
        HStack(spacing: 0) {
            NetworkLogoIcon(networkName: networkLogo, size: Heights.networkLogoInCapsule)
                .padding(.trailing, Spacing.extraSmall)
            Text(networkTitle.capitalized)
                .foregroundColor(.textAndIconsSecondary)
                .font(PrimaryFont.bodyM.font)
                .padding(.trailing, Spacing.extraSmall)
        }
        .padding(Spacing.minimal)
        .background(.fill12)
        .clipShape(Capsule())
    }
}

#if DEBUG
    struct NetworkIconCapsuleView_Previews: PreviewProvider {
        static var previews: some View {
            NetworkIconCapsuleView(networkLogo: "polkadot", networkTitle: "Polkadot")
        }
    }
#endif

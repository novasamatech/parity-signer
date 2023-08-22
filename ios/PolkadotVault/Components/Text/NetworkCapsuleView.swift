//
//  NetworkCapsuleView.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 30/12/2022.
//

import SwiftUI

struct NetworkCapsuleView: View {
    let network: String

    var body: some View {
        Text(network.capitalized)
            .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
            .font(PrimaryFont.captionM.font)
            .padding([.top, .bottom], Spacing.extraExtraSmall)
            .padding(.horizontal, Spacing.extraSmall)
            .background(Asset.fill12.swiftUIColor)
            .clipShape(Capsule())
    }
}

#if DEBUG
    struct NetworkCapsuleView_Previews: PreviewProvider {
        static var previews: some View {
            NetworkCapsuleView(network: "Polkadot")
        }
    }
#endif

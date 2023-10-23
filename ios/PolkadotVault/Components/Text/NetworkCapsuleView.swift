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
            .foregroundColor(.textAndIconsTertiary)
            .font(PrimaryFont.captionM.font)
            .padding([.top, .bottom], Spacing.extraExtraSmall)
            .padding(.horizontal, Spacing.extraSmall)
            .background(.fill12)
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

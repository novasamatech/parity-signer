//
//  NetworkLogoIcon.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 12/12/2022.
//

import SwiftUI

struct NetworkLogoIcon: View {
    let logo: String

    var body: some View {
        Text(logo)
            .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
            .font(.web3)
            .frame(width: Heights.networkLogoInCell, height: Heights.networkLogoInCell)
            .background(Circle().foregroundColor(Asset.fill12.swiftUIColor))
    }
}

#if DEBUG
    struct NetworkLogoIcon_Previews: PreviewProvider {
        static var previews: some View {
            VStack {
                NetworkLogoIcon(logo: "polkadot")
                NetworkLogoIcon(logo: "kusama")
                NetworkLogoIcon(logo: "westend")
                NetworkLogoIcon(logo: "acala")
                NetworkLogoIcon(logo: "moonbeam")
                NetworkLogoIcon(logo: "astar")
            }
            .previewLayout(.sizeThatFits)
        }
    }
#endif

//
//  NetworkLogoIcon.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 12/12/2022.
//

import SwiftUI

struct NetworkLogoIcon: View {
    let networkName: String
    let size: CGFloat

    init(
        networkName: String,
        size: CGFloat = Heights.networkLogoInCell
    ) {
        self.networkName = networkName
        self.size = size
    }

    var body: some View {
        let image = UIImage(named: networkName)
        Group {
            if let image {
                Image(uiImage: image)
                    .resizable()
                    .aspectRatio(contentMode: .fit)
                    .frame(width: size, height: size)
                    .clipShape(Circle())
            } else {
                UnknownNetworkIcon(
                    networkName: networkName,
                    size: size
                )
            }
        }
    }
}

#if DEBUG
    struct NetworkLogoIcon_Previews: PreviewProvider {
        static var previews: some View {
            VStack {
                NetworkLogoIcon(networkName: "polkadot")
                NetworkLogoIcon(networkName: "kusama")
                NetworkLogoIcon(networkName: "westend")
                NetworkLogoIcon(networkName: "acala")
                NetworkLogoIcon(networkName: "frequency")
                NetworkLogoIcon(networkName: "moonbeam")
            }
            VStack {
                NetworkLogoIcon(networkName: "astar")
                NetworkLogoIcon(networkName: "moonriver")
                NetworkLogoIcon(networkName: "BigNewNetwork")
                NetworkLogoIcon(networkName: "Random")
                NetworkLogoIcon(networkName: "GLMR")
            }
            .previewLayout(.sizeThatFits)
        }
    }
#endif

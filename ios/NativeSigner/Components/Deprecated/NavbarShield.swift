//
//  NavbarShield.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 27.7.2021.
//

import Network
import SwiftUI

struct NavbarShield: View {
    let isConnectivityOn: Bool
    let alert: Bool
    @ObservedObject var navigation: NavigationCoordinator

    var body: some View {
        Button(
            action: {
                navigation.perform(navigation: .init(action: .shield))
            },
            label: {
                if isConnectivityOn {
                    Image(.shield, variant: .slash)
                        .imageScale(.large)
                        .foregroundColor(Asset.signalDanger.swiftUIColor)
                } else {
                    if alert {
                        Image(.exclamationmark, variant: .shield)
                            .imageScale(.large)
                            .foregroundColor(Asset.signalWarning.swiftUIColor)
                    } else {
                        Image(.lock, variants: [.shield, .fill])
                            .imageScale(.large)
                            .foregroundColor(Asset.crypto400.swiftUIColor)
                    }
                }
            }
        )
    }
}

// struct NavbarShield_Previews: PreviewProvider {
// static var previews: some View {
// NavbarShield().previewLayout(.sizeThatFits)
// }
// }

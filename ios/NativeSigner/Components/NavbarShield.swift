//
//  NavbarShield.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 27.7.2021.
//

import Network
import SwiftUI

struct NavbarShield: View {
    let canaryDead: Bool
    let alert: Bool
    let pushButton: (Action, String, String) -> Void
    var body: some View {
        Button(
            action: {
                pushButton(.shield, "", "")
            },
            label: {
                if canaryDead /* bluetooth detector: `|| data.bsDetector.canaryDead` */ {
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

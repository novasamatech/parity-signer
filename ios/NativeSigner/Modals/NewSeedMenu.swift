//
//  NewSeedMenu.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 7.12.2021.
//

import SwiftUI

struct NewSeedMenu: View {
    let alert: Bool
    let alertShow: () -> Void
    let navigationRequest: NavigationRequest
    var body: some View {
        VStack {
            Spacer()
            VStack {
                HeaderBar(line1: "ADD SEED", line2: "Select seed addition method")
                MenuButtonsStack {
                    BigButton(
                        text: "New seed",
                        action: {
                            if alert { alertShow() } else {
                                navigationRequest(.init(action: .newSeed))
                            }
                        }
                    )
                    BigButton(
                        text: "Recover seed",
                        isShaded: true,
                        action: {
                            if alert { alertShow() } else {
                                navigationRequest(.init(action: .recoverSeed))
                            }
                        }
                    )
                }
            }
            .padding([.leading, .trailing, .top])
            .padding(.bottom, 24)
            .background(Asset.bg000.swiftUIColor)
        }
    }
}

// struct NewSeedMenu_Previews: PreviewProvider {
// static var previews: some View {
// NewSeedMenu()
// }
// }

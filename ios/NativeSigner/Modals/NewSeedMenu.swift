//
//  NewSeedMenu.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 7.12.2021.
//

import SwiftUI

struct NewSeedMenu: View {
    @Binding var isShowingNewSeedMenu: Bool
    @ObservedObject var navigation: NavigationCoordinator

    var body: some View {
        VStack {
            Spacer()
                .frame(idealHeight: .infinity)
            HeaderBar(
                line1: Localizable.AddSeed.uppercased.key,
                line2: Localizable.selectSeedAdditionMethod.key
            )
            MenuButtonsStack {
                BigButton(
                    text: Localizable.newSeed.key,
                    action: {
                        isShowingNewSeedMenu.toggle()
                        navigation.perform(navigation: .init(action: .newSeed))
                    }
                )
                BigButton(
                    text: Localizable.recoverSeed.key,
                    isShaded: true,
                    action: {
                        isShowingNewSeedMenu.toggle()
                        navigation.perform(navigation: .init(action: .recoverSeed))
                    }
                )
            }
        }
        .padding([.leading, .trailing, .top])
        .padding(.bottom, 24)
        .background(Asset.bg000.swiftUIColor)
    }
}

// struct NewSeedMenu_Previews: PreviewProvider {
// static var previews: some View {
// NewSeedMenu()
// }
// }

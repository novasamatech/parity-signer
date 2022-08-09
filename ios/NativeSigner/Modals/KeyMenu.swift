//
//  KeyMenu.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 21.12.2021.
//

import SwiftUI

struct KeyMenu: View {
    @State private var removeConfirm = false

    let navigationRequest: NavigationRequest
    var body: some View {
        MenuStack {
            HeaderBar(line1: Localizable.keyMenu.key, line2: Localizable.selectAction.key).padding(.top, 10)
            MenuButtonsStack {
                BigButton(
                    text: Localizable.forgetThisKeyForever.key,
                    isShaded: true,
                    isDangerous: true,
                    action: {
                        removeConfirm = true
                    }
                )
            }
        }
        .alert(isPresented: $removeConfirm, content: {
            Alert(
                title: Localizable.forgetThisKey.text,
                message: Localizable.ThisKeyWillBeRemovedForThisNetwork.areYouSure.text,
                primaryButton: .cancel(Localizable.cancel.text),
                secondaryButton: .destructive(
                    Localizable.removeKey.text,
                    action: { navigationRequest(.init(action: .removeKey)) }
                )
            )
        })
    }
}

// struct KeyMenu_Previews: PreviewProvider {
// static var previews: some View {
// KeyMenu()
// }
// }

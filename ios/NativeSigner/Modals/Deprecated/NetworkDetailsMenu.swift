//
//  NetworkDetailsMenu.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 20.12.2021.
//

import SwiftUI

struct NetworkDetailsMenu: View {
    @State private var removeNetworkAlert = false

    let navigationRequest: NavigationRequest
    var body: some View {
        MenuStack {
            HeaderBar(line1: Localizable.manageNetwork.key, line2: Localizable.selectAction.key).padding(.top, 10)
            MenuButtonsStack {
                BigButton(
                    text: Localizable.signNetworkSpecs.key,
                    isShaded: true,
                    isCrypto: true,
                    action: { navigationRequest(.init(action: .signNetworkSpecs)) }
                )
                BigButton(
                    text: Localizable.deleteNetwork.key,
                    isShaded: true,
                    isDangerous: true,
                    action: { removeNetworkAlert = true }
                )
            }
        }
        .gesture(DragGesture().onEnded { drag in
            if drag.translation.height > 40 {
                navigationRequest(.init(action: .goBack))
            }
        })
        .alert(isPresented: $removeNetworkAlert, content: {
            Alert(
                title: Localizable.removeNetworkQuestion.text,
                message: Localizable.thisNetworkWillBeRemovedForWholeDevice.text,
                primaryButton: .cancel(Localizable.cancel.text),
                secondaryButton: .destructive(
                    Localizable.removeNetwork.text,
                    action: { navigationRequest(.init(action: .removeNetwork)) }
                )
            )
        })
    }
}

// struct NetworkDetailsMenu_Previews: PreviewProvider {
// static var previews: some View {
// NetworkDetailsMenu()
// }
// }

//
//  TypesMenu.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 23.12.2021.
//

import SwiftUI

struct TypesMenu: View {
    var content: MTypesInfo

    let navigationRequest: NavigationRequest
    @State private var removeTypesAlert = false
    var body: some View {
        MenuStack {
            HeaderBar(line1: Localizable.manageTypes.key, line2: Localizable.selectAction.key).padding(.top, 10)
            if content.typesOnFile {
                HStack {
                    Identicon(identicon: content.typesIdPic?.svgPayload ?? []) // this is potentially different image
                }
                Text(content.typesHash ?? Localizable.none.string)
            } else {
                Localizable.preV14TypesNotInstalled.text
            }
            MenuButtonsStack {
                BigButton(
                    text: Localizable.signTypes.key,
                    isShaded: true,
                    isCrypto: true,
                    action: { navigationRequest(.init(action: .signTypes)) }
                )
                BigButton(
                    text: Localizable.deleteTypes.key,
                    isShaded: true,
                    isDangerous: true,
                    action: { removeTypesAlert = true }
                )
            }
        }
        .gesture(DragGesture().onEnded { drag in
            if drag.translation.height > 40 {
                navigationRequest(.init(action: .goBack))
            }
        })
        .alert(isPresented: $removeTypesAlert, content: {
            Alert(
                title: Localizable.removeTypesQuestion.text,
                message: Localizable.TypesInformationNeededForSupportOfPreV14MetadataWillBeRemoved.areYouSure.text,
                primaryButton: .cancel(Localizable.cancel.text),
                secondaryButton: .destructive(
                    Localizable.removeTypes.text,
                    action: { navigationRequest(.init(action: .removeTypes)) }
                )
            )
        })
    }
}

// struct TypesMenu_Previews: PreviewProvider {
// static var previews: some View {
// TypesMenu()
// }
// }

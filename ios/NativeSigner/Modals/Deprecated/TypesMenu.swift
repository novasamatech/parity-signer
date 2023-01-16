//
//  TypesMenu.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 23.12.2021.
//

import SwiftUI

struct TypesMenu: View {
    var content: MTypesInfo

    @EnvironmentObject var navigation: NavigationCoordinator
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
                PrimaryButton(
                    action: {
                        navigation.perform(navigation: .init(action: .signTypes))
                    },
                    text: Localizable.signTypes.key,
                    style: .primary()
                )
                PrimaryButton(
                    action: {
                        removeTypesAlert = true
                    },
                    text: Localizable.deleteTypes.key,
                    style: .primaryDestructive()
                )
            }
        }
        .gesture(DragGesture().onEnded { drag in
            if drag.translation.height > 40 {
                navigation.perform(navigation: .init(action: .goBack))
            }
        })
        .alert(isPresented: $removeTypesAlert, content: {
            Alert(
                title: Localizable.removeTypesQuestion.text,
                message: Localizable.TypesInformationNeededForSupportOfPreV14MetadataWillBeRemoved.areYouSure.text,
                primaryButton: .cancel(Localizable.cancel.text),
                secondaryButton: .destructive(
                    Localizable.removeTypes.text,
                    action: { self.navigation.perform(navigation: .init(action: .removeTypes)) }
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

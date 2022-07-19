//
//  TypesMenu.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 23.12.2021.
//

import SwiftUI

struct TypesMenu: View {
    var content: MTypesInfo

    let pushButton: (Action, String, String) -> Void
    @State var removeTypesAlert = false
    var body: some View {
        MenuStack {
            HeaderBar(line1: "MANAGE TYPES", line2: "Select action").padding(.top, 10)
            if content.typesOnFile {
                HStack {
                    Identicon(identicon: content.typesIdPic ?? []) // this is potentially different image
                }
                Text(content.typesHash ?? "none")
            } else {
                Text("Pre-v14 types not installed")
            }
            MenuButtonsStack {
                BigButton(
                    text: "Sign types",
                    isShaded: true,
                    isCrypto: true,
                    action: {pushButton(.signTypes, "", "")}
                )
                BigButton(
                    text: "Delete types",
                    isShaded: true,
                    isDangerous: true,
                    action: {removeTypesAlert = true}
                )
            }
        }
        .gesture(DragGesture().onEnded {drag in
            if drag.translation.height > 40 {
                pushButton(.goBack, "", "")
            }
        })
        .alert(isPresented: $removeTypesAlert, content: {
            Alert(title: Text("Remove types?"),
                  message: Text(
                    "Types information needed for support of pre-v14 metadata will be removed. Are you sure?"
                  ),
                  primaryButton: .cancel(Text("Cancel")),
                  secondaryButton: .destructive(Text("Remove types"),
                                                action: {pushButton(.removeTypes, "", "")}))
        })
    }
}

/*
 struct TypesMenu_Previews: PreviewProvider {
 static var previews: some View {
 TypesMenu()
 }
 }
 */

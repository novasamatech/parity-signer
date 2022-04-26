//
//  TypesMenu.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 23.12.2021.
//

import SwiftUI

struct TypesMenu: View {
    @EnvironmentObject var data: SignerDataModel
    var content: MTypesInfo
    @State var removeTypesAlert = false
    var body: some View {
        MenuStack {
            HeaderBar(line1: "MANAGE TYPES", line2: "Select action").padding(.top, 10)
            if content.typesOnFile {
                HStack {
                    Image(uiImage: UIImage(data: Data(fromHexEncodedString: content.typesIdPic ?? "") ?? Data()) ?? UIImage())
                        .resizable(resizingMode: .stretch)
                        .frame(width: 28, height: 28)
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
                    action:{data.pushButton(action: .signTypes)}
                )
                BigButton(
                    text: "Delete types",
                    isShaded: true,
                    isDangerous: true,
                    action: {removeTypesAlert = true}
                )
            }
        }
        .gesture(DragGesture().onEnded{drag in
            if drag.translation.height > 40 {
                data.pushButton(action: .goBack)
            }
        })
        .alert(isPresented: $removeTypesAlert, content: {
            Alert(title: Text("Remove types?"),
                  message: Text("Types information needed for support of pre-v14 metadata will be removed. Are you sure?"),
                  primaryButton: .cancel(Text("Cancel")),
                  secondaryButton: .destructive(Text("Remove types"),
                                                action: {data.pushButton(action: .removeTypes)}))
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

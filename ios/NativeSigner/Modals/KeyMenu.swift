//
//  KeyMenu.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 21.12.2021.
//

import SwiftUI

struct KeyMenu: View {
    @EnvironmentObject var data: SignerDataModel
    @State var removeConfirm = false
    var body: some View {
        MenuStack {
            HeaderBar(line1: "KEY MENU", line2: "Select action").padding(.top, 10)
            MenuButtonsStack {
                BigButton(
                    text: "Forget this key forever",
                    isShaded: true,
                    isDangerous: true,
                    action: {
                        removeConfirm = true
                    }
                )
            }
        }
        .alert(isPresented: $removeConfirm, content: {
            Alert(title: Text("Forget this key?"), message: Text("This key will be removed for this network. Are you sure?"), primaryButton: .cancel(Text("Cancel")), secondaryButton: .destructive(Text("Remove key"), action: {data.pushButton(buttonID: .RemoveKey)}))
        })
    }
}

/*
struct KeyMenu_Previews: PreviewProvider {
    static var previews: some View {
        KeyMenu()
    }
}
*/

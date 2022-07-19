//
//  SeedMenu.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 10.12.2021.
//

import SwiftUI

struct SeedMenu: View {
    @State var removeConfirm = false
    let content: MSeedMenu
    let alert: Bool
    let alertShow: () -> Void
    let removeSeed: (String) -> Void
    let pushButton: (Action, String, String) -> Void
    var body: some View {
        MenuStack {
            HeaderBar(line1: "SEED MENU", line2: "Select action").padding(.top, 10)
            MenuButtonsStack {
                BigButton(
                    text: "Backup",
                    action: {
                        pushButton(.backupSeed, "", "")
                    }
                )
                BigButton(
                    text: "Derive new key",
                    isShaded: true,
                    isCrypto: true,
                    action: {
                        if alert { alertShow() } else {
                            pushButton(.newKey, "", "")
                        }
                    }
                )
                BigButton(
                    text: "Forget this seed forever",
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
                title: Text("Forget this seed?"),
                message: Text("This seed will be removed for all networks. This is not reversible. Are you sure?"),
                primaryButton: .cancel(Text("Cancel")),
                secondaryButton: .destructive(
                    Text("Remove seed"),
                    action: {removeSeed(content.seed)}
                )
            )
        })
    }
}

/*
 struct SeedMenu_Previews: PreviewProvider {
 static var previews: some View {
 SeedMenu()
 }
 }
 */

//
//  SeedMenu.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 10.12.2021.
//

import SwiftUI

struct SeedMenu: View {
    @EnvironmentObject var data: SignerDataModel
    @State var removeConfirm = false
    let content: MSeedMenu
    var body: some View {
        MenuStack {
            HeaderBar(line1: "SEED MENU", line2: "Select action").padding(.top, 10)
            MenuButtonsStack {
                BigButton(
                    text: "Backup",
                    action: {
                        data.pushButton(buttonID: .BackupSeed)
                    }
                )
                BigButton(
                    text: "Derive new key",
                    isShaded: true,
                    isCrypto: true,
                    action:{
                        if data.alert { data.alertShow = true } else {
                            data.pushButton(buttonID: .NewKey)
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
            Alert(title: Text("Forget this seed?"), message: Text("This seed will be removed for all networks. This is not reversible. Are you sure?"), primaryButton: .cancel(Text("Cancel")), secondaryButton: .destructive(Text("Remove seed"), action: {data.removeSeed(seedName: content.seed)}))
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

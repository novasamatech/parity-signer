//
//  SeedMenu.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 10.12.2021.
//

import SwiftUI

struct SeedMenu: View {
    @EnvironmentObject var data: SignerDataModel
    var body: some View {
        MenuStack {
            HeaderBar(line1: "SEED MENU", line2: "Select action").padding(.top, 10)
            MenuButtonsStack {
                BigButton(
                    text: "Backup",
                    action: {data.pushButton(buttonID: .BackupSeed)}
                )
                BigButton(
                    text: "Derive new key",
                    isShaded: true,
                    isCrypto: true,
                    action:{data.pushButton(buttonID: .NewKey)}
                )
                BigButton(
                    text: "Forget this seed forever",
                    isShaded: true,
                    isDangerous: true,
                    action: {}
                )
            }
            
        }
    }
}

/*
struct SeedMenu_Previews: PreviewProvider {
    static var previews: some View {
        SeedMenu()
    }
}
*/

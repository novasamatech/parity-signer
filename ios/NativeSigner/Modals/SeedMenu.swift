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
        VStack {
                Spacer().frame(height: UIScreen.main.bounds.height/2)
            ZStack {
                RoundedRectangle(cornerRadius: 20.0).foregroundColor(Color("Bg000"))
                VStack {
                    Spacer()
                    Rectangle().foregroundColor(Color("Bg000")).frame(height: 25)
                }
                VStack {
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
    }
}

/*
struct SeedMenu_Previews: PreviewProvider {
    static var previews: some View {
        SeedMenu()
    }
}
*/

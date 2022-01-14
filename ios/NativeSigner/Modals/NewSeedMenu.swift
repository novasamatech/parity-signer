//
//  NewSeedMenu.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 7.12.2021.
//

import SwiftUI

struct NewSeedMenu: View {
    @EnvironmentObject var data: SignerDataModel
    var body: some View {
        VStack {
            Spacer()
            VStack {
                HeaderBar(line1: "ADD SEED", line2: "Select seed addition method" )
                MenuButtonsStack {
                    BigButton(
                        text: "New seed",
                        action: {
                            if data.alert { data.alertShow = true } else {
                                data.pushButton(buttonID: .NewSeed)
                            }
                        }
                    )
                    BigButton(
                        text: "Recover seed",
                        isShaded: true,
                        action: {
                            if data.alert { data.alertShow = true } else {
                                data.pushButton(buttonID: .RecoverSeed)
                            }
                        }
                    )
                }
            }
            .padding([.leading, .trailing, .top])
            .padding(.bottom, 24)
            .background(Color("Bg000"))
        }
    }
}

/*
 struct NewSeedMenu_Previews: PreviewProvider {
 static var previews: some View {
 NewSeedMenu()
 }
 }
 */

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
                VStack (spacing: 16) {
                    BigButton(
                        text: "New seed",
                        action: {
                            data.pushButton(buttonID: .NewSeed)
                        }
                    )
                    BigButton(
                        text: "Recover seed",
                        isShaded: true,
                        action: {
                            data.pushButton(buttonID: .RecoverSeed)
                        }
                    )
                }
                .padding(.top, 12)
            }
            .padding([.leading, .trailing, .top], 16)
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

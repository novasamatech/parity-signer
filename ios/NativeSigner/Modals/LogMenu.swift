//
//  LogMenu.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 17.12.2021.
//

import SwiftUI

struct LogMenu: View {
    @EnvironmentObject var data: SignerDataModel
    var body: some View {
        VStack {
            Spacer()
            VStack {
                HeaderBar(line1: "LOG", line2: "Manage log" )
                MenuButtonsStack {
                    BigButton(
                        text: "Erase log",
                        action: {
                            data.pushButton(buttonID: .ClearLog)
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
 struct LogMenu_Previews: PreviewProvider {
 static var previews: some View {
 LogMenu()
 }
 }
 */

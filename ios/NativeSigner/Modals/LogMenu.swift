//
//  LogMenu.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 17.12.2021.
//

import SwiftUI

struct LogMenu: View {
    @EnvironmentObject var data: SignerDataModel
    @State var clearConfirm = false
    var body: some View {
        VStack {
            Spacer()
            VStack {
                HeaderBar(line1: "LOG", line2: "Manage log" )
                MenuButtonsStack {
                    BigButton(
                        text: "Clear log",
                        action: {
                            clearConfirm = true
                        }
                    )
                }
            }
            .padding([.leading, .trailing, .top])
            .padding(.bottom, 24)
            .background(Color("Bg000"))
            .alert(isPresented: $clearConfirm, content: {
                Alert(title: Text("Clear log?"), message: Text("Do you want this Signer to forget all logged events? This is not reversible."), primaryButton: .cancel(Text("Cancel")), secondaryButton: .destructive(Text("Clear log"), action: {data.pushButton(buttonID: .ClearLog)}))
            })
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

//
//  LogMenu.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 17.12.2021.
//

import SwiftUI

struct LogMenu: View {
    @State var clearConfirm = false
    var content: MLogRight
    let pushButton: (Action, String, String) -> Void
    var body: some View {
        VStack {
            Spacer()
            VStack {
                HeaderBar(line1: "LOG", line2: "Checksum: " + content.checksum)
                MenuButtonsStack {
                    BigButton(
                        text: "Add note",
                        action: {
                            pushButton(.createLogComment, "", "")
                        }
                    )
                    BigButton(
                        text: "Clear log",
                        isShaded: true,
                        isDangerous: true,
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
                Alert(
                    title: Text("Clear log?"),
                    message: Text("Do you want this Signer to forget all logged events? This is not reversible."),
                    primaryButton: .cancel(Text("Cancel")),
                    secondaryButton: .destructive(
                        Text("Clear log"),
                        action: {pushButton(.clearLog, "", "")})
                )
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

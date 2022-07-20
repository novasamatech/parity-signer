//
//  NetworkDetailsMenu.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 20.12.2021.
//

import SwiftUI

struct NetworkDetailsMenu: View {
    @State var removeNetworkAlert = false

    let pushButton: (Action, String, String) -> Void
    var body: some View {
        MenuStack {
            HeaderBar(line1: "MANAGE NETWORK", line2: "Select action").padding(.top, 10)
            MenuButtonsStack {
                BigButton(
                    text: "Sign network specs",
                    isShaded: true,
                    isCrypto: true,
                    action: {pushButton(.signNetworkSpecs, "", "")}
                )
                BigButton(
                    text: "Delete network",
                    isShaded: true,
                    isDangerous: true,
                    action: {removeNetworkAlert = true}
                )
            }

        }
        .gesture(DragGesture().onEnded {drag in
            if drag.translation.height > 40 {
                pushButton(.goBack, "", "")
            }
        })
        .alert(isPresented: $removeNetworkAlert, content: {
            Alert(
                title: Text("Remove network?"),
                message: Text("This network will be removed for whole device"),
                primaryButton: .cancel(Text("Cancel")),
                secondaryButton: .destructive(
                    Text("Remove network"),
                    action: {pushButton(.removeNetwork, "", "")}
                )
            )
        })
    }
}

/*
 struct NetworkDetailsMenu_Previews: PreviewProvider {
 static var previews: some View {
 NetworkDetailsMenu()
 }
 }
 */

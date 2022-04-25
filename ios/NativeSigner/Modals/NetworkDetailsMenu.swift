//
//  NetworkDetailsMenu.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 20.12.2021.
//

import SwiftUI

struct NetworkDetailsMenu: View {
    @EnvironmentObject var data: SignerDataModel
    @State var removeNetworkAlert = false
    var body: some View {
        MenuStack {
            HeaderBar(line1: "MANAGE NETWORK", line2: "Select action").padding(.top, 10)
            MenuButtonsStack {
                BigButton(
                    text: "Sign network specs",
                    isShaded: true,
                    isCrypto: true,
                    action:{data.pushButton(action: .signNetworkSpecs)}
                )
                BigButton(
                    text: "Delete network",
                    isShaded: true,
                    isDangerous: true,
                    action: {removeNetworkAlert = true}
                )
            }
            
        }
        .gesture(DragGesture().onEnded{drag in
            if drag.translation.height > 40 {
                data.pushButton(buttonID: .GoBack)
            }
        })
        .alert(isPresented: $removeNetworkAlert, content: {
            Alert(title: Text("Remove network?"), message: Text("This network will be removed for whole device"), primaryButton: .cancel(Text("Cancel")), secondaryButton: .destructive(Text("Remove network"), action: {data.pushButton(buttonID: .RemoveNetwork)}))
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

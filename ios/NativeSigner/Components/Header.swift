//
//  Header.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 13.9.2021.
//

import SwiftUI

struct Header: View {
    @EnvironmentObject var data: SignerDataModel
    var body: some View {
        HStack {
            /*
            if false{
                Button(action: {
                    //TODO: buttonpush
                }) {
                    SmallButton(text: "Cancel")
                }
            } else {
                if data.actionResult.back {
                    Button(action: {
                        data.pushButton(buttonID: .GoBack)
                    }) {
                        Image(systemName: "chevron.left").imageScale(.large)
                    }
                }
            }*/
            Spacer()
            Text(data.actionResult.screenLabel)
            Spacer()
            /*
            if false {
                Button(action: {
                    //TODO: Buttonpush
                }) {
                    SmallButton(text: "Select all")
                }
            }*/
            if (true) {
                Button(action: {
                    data.keyManagerModal = .newSeed
                }) {
                    Image(systemName: "plus.circle")
                        .imageScale(.large)
                }
            }
            NavbarShield()
        }
        .padding().background(/*@START_MENU_TOKEN@*//*@PLACEHOLDER=View@*/Color("backgroundColor")/*@END_MENU_TOKEN@*/)
    }
}

/*
 struct Header_Previews: PreviewProvider {
 static var previews: some View {
 Header().previewLayout(.sizeThatFits)
 }
 }
 */

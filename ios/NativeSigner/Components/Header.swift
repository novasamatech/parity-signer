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
            if data.actionResult.back {
                Button(action: {
                    data.pushButton(buttonID: .GoBack)
                }) {
                    Image(systemName: "chevron.left").imageScale(.large).foregroundColor(Color("Text500"))
                }
            }
            /*if data.actionResult.back {
             Button(action: {
             data.pushButton(buttonID: .GoBack)
             }) {
             SmallButton(text: "Cancel")
             }
             } else {*/
            Spacer()
            Text(data.actionResult.screenLabel).foregroundColor(Color("Text600"))
                .font(data.actionResult.screenNameType == "h1" ? FBase(style: .h1) : FBase(style: .h4))
            Spacer()
            /*
             if false {
             Button(action: {
             //TODO: Buttonpush
             }) {
             SmallButton(text: "Select all")
             }
             }*/
            Button(action: {
                data.pushButton(buttonID: .RightButton)
            }) {
                switch(data.actionResult.rightButton) {
                case "NewSeed":
                    Image(systemName: "plus.circle").imageScale(.large).foregroundColor(Color("Action400"))
                case "Backup":
                    Image(systemName: "ellipsis").imageScale(.large).foregroundColor(Color("Action400"))
                default:
                    EmptyView()
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

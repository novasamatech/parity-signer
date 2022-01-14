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
        VStack {
            Spacer()
            HStack {
                HStack(spacing: 8.0) {
                    if data.actionResult.back {
                        Button(action: {
                            data.pushButton(buttonID: .GoBack)
                        }) {
                            Image(systemName: data.actionResult.rightButton == "MultiSelect" ? "xmark" : "chevron.left")
                                .imageScale(.large)
                                .foregroundColor(Color("Text500"))
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
                }
                .frame(width: 72.0)
                
                Spacer()
                Text(data.actionResult.screenLabel)
                    .foregroundColor(Color("Text600"))
                    .font(data.actionResult.screenNameType == "h1" ? FBase(style: .h2) : FBase(style: .h4))
                    .tracking(0.1)
                
                if data.actionResult.rightButton == "MultiSelect" {
                    Button(action: {
                        data.pushButton(buttonID: .SelectAll)
                    }) {
                        SmallButton(text: "Select all")
                    }
                }
                Spacer()
                
                HStack(spacing: 8.0) {
                    Spacer()
                    Button(action: {
                        if data.alert && data.actionResult.rightButton == "NewSeed" {
                            data.alertShow = true
                        } else {
                            data.pushButton(buttonID: .RightButton)
                        }
                    }) {
                        switch(
                            data.actionResult.rightButton
                        ) {
                        case "NewSeed":
                            Image(systemName: "plus.circle")
                                .imageScale(.large)
                                .foregroundColor(Color("Action400"))
                        case "Backup":
                            Image(systemName: "ellipsis")
                                .imageScale(.large)
                                .foregroundColor(Color("Action400"))
                        case "LogRight":
                            Image(systemName: "ellipsis")
                                .imageScale(.large)
                                .foregroundColor(Color("Action400"))
                        case "MultiSelect":
                            EmptyView()
                        case "None":
                            EmptyView()
                        default:
                            Image(systemName: "ellipsis")
                                .imageScale(.large)
                                .foregroundColor(Color("Action400"))
                        }
                    }
                    NavbarShield()
                }
                .frame(width: 72.0)
            }
        }
        .frame(height: 32.0)
        .padding(.all, 8.0)
    }
}

/*
 struct Header_Previews: PreviewProvider {
 static var previews: some View {
 Header().previewLayout(.sizeThatFits)
 }
 }
 */

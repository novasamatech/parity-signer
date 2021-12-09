//
//  Footer.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 26.7.2021.
//

import SwiftUI

struct KeySymbol: View {
    var body: some View {
        VStack{
            Text("accounts").font(Font.custom("Web3-Regular", size: 20))
        }
    }
}

struct WrenchSymbol: View {
    var body: some View {
        VStack{
            Image(systemName: "gearshape.fill").imageScale(.medium)
        }
    }
}

struct Footer: View {
    @EnvironmentObject var data: SignerDataModel
    var body: some View {
        HStack {
            Button(action: {
                data.pushButton(buttonID: ButtonID.NavbarLog)
            }) {
                VStack(alignment: .center) {
                    Image(systemName: "rectangle.grid.1x2.fill").imageScale(.medium)
                        .padding(.top, 4.0)
                        .padding(.bottom, 1.0)
                    Text("Log")
                        
                }
                .foregroundColor(buttonColor(active: data.actionResult.footerButton == "Log"))
            }
            Spacer()
            Button(action: {
                data.pushButton(buttonID: ButtonID.NavbarScan)
            }) {
                VStack {
                    Image(systemName: "viewfinder").imageScale(.medium)
                        .padding(.top, 4.0)
                        .padding(.bottom, 1.0)
                    Text("Scanner")
                }
                .foregroundColor(buttonColor(active: data.actionResult.footerButton == "Scan"))
            }
            Spacer()
            Button(action: {
                data.pushButton(buttonID: ButtonID.NavbarKeys)
            }) {
                VStack{
                    KeySymbol()
                    Text("Keys")
                }
                .foregroundColor(buttonColor(active: data.actionResult.footerButton == "Keys"))
            }
            Spacer()
            Button(action: {
                data.pushButton(buttonID: ButtonID.NavbarSettings)
            }) {
                VStack {
                    WrenchSymbol()
                        .padding(.top, 4.0)
                        .padding(.bottom, 1.0)
                    Text("Settings")
                }
                .foregroundColor(buttonColor(active: data.actionResult.footerButton == "Settings"))
            }
        }.font(.footnote)
    }
}

func buttonColor(active: Bool) -> Color {
    return active ? Color("Text600") : Color("Text300")
}

/*
func buttonLabelColor(active: Bool) -> Color {
    return active ? Color("Text600") : Color("Text300")
}
 */

/*
 struct Footer_Previews: PreviewProvider {
 static var previews: some View {
 Footer().previewLayout(.sizeThatFits)
 }
 }
 */

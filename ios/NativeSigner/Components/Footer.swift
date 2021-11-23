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
            Image(systemName: "circle.hexagongrid.fill").imageScale(.medium)
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
                data.refreshUI()
                //data.signerScreen = .Log
                data.pushButton(buttonID: ButtonID.NavbarLog)
            }) {
                VStack(alignment: .center) {
                    Image(systemName: "rectangle.grid.1x2.fill").imageScale(.medium).foregroundColor(data.signerScreen == .Log ? Color("buttonActive") : Color("buttonPassiveImage"))
                    Text("Log").foregroundColor(data.signerScreen == .Log ? Color("buttonActive") : Color("buttonPassiveText"))
                }
            }
            Spacer()
            Button(action: {
                data.refreshUI()
                //data.signerScreen = .Scan
                data.pushButton(buttonID: ButtonID.NavbarScan)

            }) {
                VStack {
                    Image(systemName: "viewfinder").imageScale(.medium).foregroundColor(data.signerScreen == .Scan ? Color("buttonActive") : Color("buttonPassiveImage"))
                    Text("Scanner").foregroundColor(data.signerScreen == .Scan ? Color("buttonActive") : Color("buttonPassiveText"))
                }
            }
            Spacer()
            Button(action: {
                data.refreshUI()
                //data.signerScreen = .Keys
                data.pushButton(buttonID: ButtonID.NavbarKeys)
            }) {
                VStack{
                    KeySymbol().foregroundColor(data.signerScreen == .Keys ? Color("buttonActive") : Color("buttonPassiveImage"))
                    Text("Keys").foregroundColor(data.signerScreen == .Keys ? Color("buttonActive") : Color("buttonPassiveText"))
                }
            }
            Spacer()
            Button(action: {
                data.refreshUI()
                data.networkSettings = nil
                //data.signerScreen = .Settings
                data.pushButton(buttonID: ButtonID.NavbarSettings)
            }) {
                VStack {
                    WrenchSymbol().foregroundColor(data.signerScreen == .Settings ? Color("buttonActive") : Color("buttonPassiveImage"))
                    Text("Settings").foregroundColor(data.signerScreen == .Settings ? Color("buttonActive") : Color("buttonPassiveText"))
                }
            }
        }.font(.footnote)
    }
}

/*
 struct Footer_Previews: PreviewProvider {
 static var previews: some View {
 Footer().previewLayout(.sizeThatFits)
 }
 }
 */

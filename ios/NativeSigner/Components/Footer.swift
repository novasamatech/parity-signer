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
                data.pushButton(buttonID: ButtonID.NavbarLog)
            }) {
                VStack(alignment: .center) {
                    Image(systemName: "rectangle.grid.1x2.fill").imageScale(.medium).foregroundColor(data.actionResult.screen == .Log ? Color("buttonActive") : Color("buttonPassiveImage"))
                    Text("Log").foregroundColor(data.actionResult.screen == .Log ? Color("buttonActive") : Color("buttonPassiveText"))
                }
            }
            Spacer()
            Button(action: {
                data.pushButton(buttonID: ButtonID.NavbarScan)
            }) {
                VStack {
                    Image(systemName: "viewfinder").imageScale(.medium).foregroundColor(data.actionResult.screen == .Scan ? Color("buttonActive") : Color("buttonPassiveImage"))
                    Text("Scanner").foregroundColor(data.actionResult.screen == .Scan ? Color("buttonActive") : Color("buttonPassiveText"))
                }
            }
            Spacer()
            Button(action: {
                data.pushButton(buttonID: ButtonID.NavbarKeys)
            }) {
                VStack{
                    KeySymbol().foregroundColor(data.actionResult.screen == .Keys ? Color("buttonActive") : Color("buttonPassiveImage"))
                    Text("Keys").foregroundColor(data.actionResult.screen == .Keys ? Color("buttonActive") : Color("buttonPassiveText"))
                }
            }
            Spacer()
            Button(action: {
                data.pushButton(buttonID: ButtonID.NavbarSettings)
            }) {
                VStack {
                    WrenchSymbol().foregroundColor(data.actionResult.screen == .Settings ? Color("buttonActive") : Color("buttonPassiveImage"))
                    Text("Settings").foregroundColor(data.actionResult.screen == .Settings ? Color("buttonActive") : Color("buttonPassiveText"))
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

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
            Image(systemName: "key").imageScale(.large)
        }
    }
}

struct WrenchSymbol: View {
    var body: some View {
        VStack{
            Image(systemName: "wrench").imageScale(.large)
        }
    }
}

struct Footer: View {
    @EnvironmentObject var data: SignerDataModel
    var body: some View {
        VStack {
            Button(action: {
                data.totalRefresh()
                data.signerScreen = .home
            }) {
                Image("HomeButton").offset(y: -30).padding(.bottom, -30)
            }
            HStack {
                Button(action: {
                    data.totalRefresh()
                    data.signerScreen = .keys
                }) {
                    KeySymbol()
                }
                Spacer()
                Button(action: {
                    data.totalRefresh()
                    data.networkSettings = nil
                    data.signerScreen = .settings
                }) {
                    WrenchSymbol()
                }
            }
        }.padding().background(/*@START_MENU_TOKEN@*//*@PLACEHOLDER=View@*/Color("backgroundColor")/*@END_MENU_TOKEN@*/)
    }
}

struct Footer_Previews: PreviewProvider {
    static var previews: some View {
        Footer().previewLayout(.sizeThatFits)
    }
}

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
    @Environment(\.presentationMode) var presentationMode: Binding<PresentationMode>
    @EnvironmentObject var data: SignerDataModel
    var caller: String
    var body: some View {
        VStack {
            Button(action: {presentationMode.wrappedValue.dismiss()}) {
                Image("HomeButton").offset(y: -30).padding(.bottom, -30)
            }
            HStack {
                if caller == "KeyManager" {
                    Button(action: {
                        data.totalRefresh()
                    }) {
                        KeySymbol()
                    }
                } else {
                    NavigationLink(
                    destination: KeyManager()) {
                        KeySymbol()
                    }
                }
                Spacer()
                if caller == "Settings" {
                    Button(action: {
                        data.totalRefresh()
                        data.networkSettings = nil
                    }) {
                        WrenchSymbol()
                    }
                } else {
                    NavigationLink(destination: SettingsScreen()) {
                        WrenchSymbol()
                    }
                }
            }
        }.padding().background(/*@START_MENU_TOKEN@*//*@PLACEHOLDER=View@*/Color("backgroundColor")/*@END_MENU_TOKEN@*/)
    }
}

struct Footer_Previews: PreviewProvider {
    static var previews: some View {
        Footer(caller: "home").previewLayout(.sizeThatFits)
    }
}

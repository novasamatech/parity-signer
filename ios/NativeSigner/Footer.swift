//
//  Footer.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 26.7.2021.
//

import SwiftUI

struct Footer: View {
    var body: some View {
        VStack {
            NavigationLink(destination: MainButtonScreen()) {
                Image("HomeButton").offset(y: -30).padding(.bottom, -30)
            }
            HStack {
                NavigationLink(
                    destination: NetworkList()) {
                    VStack{
                        Image(systemName: "key")
                        Text("Manage accounts")
                    }
                }
                Spacer()
                Button(action: {}) {
                    VStack {
                        Image(systemName: "wrench")
                        Text("Settings")
                    }
                }
            }.padding()
        }.background(/*@START_MENU_TOKEN@*//*@PLACEHOLDER=View@*/Color("backgroundColor")/*@END_MENU_TOKEN@*/)
    }
}

struct Footer_Previews: PreviewProvider {
    static var previews: some View {
        Footer().previewLayout(.sizeThatFits)
    }
}

//
//  SettingsScreen.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 29.7.2021.
//

import SwiftUI

struct SettingsScreen: View {
    var body: some View {
        VStack {
            NavigationLink(destination: Text("")) {
                Text("Manage seeds")
            }
            Spacer()
            NavigationLink(destination: Text("temp")) {
                Text("Network settings")
            }
            Spacer()
            Button(action: {}) {
                HStack{
                    Image(systemName: "warning")
                    Text("Wipe all data")
                    Image(systemName: "warning")
                }
            }
            Spacer()
            Footer()
        }.padding().background(/*@START_MENU_TOKEN@*//*@PLACEHOLDER=View@*/Color("backgroundColor")/*@END_MENU_TOKEN@*/)
    }
}

struct SettingsScreen_Previews: PreviewProvider {
    static var previews: some View {
        NavigationView {
            SettingsScreen()
        }
    }
}

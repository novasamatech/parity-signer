//
//  SettingsScreen.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 29.7.2021.
//

import SwiftUI

struct SettingsScreen: View {
    @EnvironmentObject var data: SignerDataModel
    @Environment(\.presentationMode) var presentationMode: Binding<PresentationMode>
    var body: some View {
        ZStack {
        VStack {
            Button(action: {}) {
                Text("Show log")
            }.padding()
            NavigationLink(destination: Text("")) {
                Text("Manage seeds")
            }.padding()
            NavigationLink(destination: Text("temp")) {
                Text("Network settings")
            }.padding()
            Button(action: {
                //TODO: add some alerts to make sure the operation was successful
                do {
                    var destination = try FileManager.default.url(for: .documentDirectory, in: .userDomainMask, appropriateFor: nil, create: false)
                    print(destination)
                    print(destination.appendPathComponent("Database"))
                    print(destination)
                    try FileManager.default.removeItem(at: destination)
                } catch {
                    print("FileManager failed to delete db")
                    return
                }
                let query = [
                    kSecClass as String: kSecClassGenericPassword
                ] as CFDictionary
                SecItemDelete(query)
                data.onboardingDone = false
                presentationMode.wrappedValue.dismiss()
            }) {
                HStack{
                    Image(systemName: "exclamationmark.triangle.fill").imageScale(.large)
                    Text("Wipe all data")
                    Image(systemName: "exclamationmark.triangle.fill").imageScale(.large)
                }
            }.padding()
            NavigationLink(destination: Text("About")) {
                Text("About")
            }.padding()
            NavigationLink(destination: Text("ToC")) {
                Text("Terms and conditions")
            }.padding()
            NavigationLink(destination: Text("Privacy statement")) {
                Text("Privacy statement")
            }.padding()
            Spacer()
        }
            VStack {
                Spacer()
                Footer(caller: "Settings")
            }
        }
        .navigationTitle("Manage identities").navigationBarTitleDisplayMode(.inline).toolbar {
            ToolbarItem(placement: .navigationBarTrailing) {
                NavbarShield()
            }
        }
        .background(/*@START_MENU_TOKEN@*//*@PLACEHOLDER=View@*/Color("backgroundColor")/*@END_MENU_TOKEN@*/)
    }
}

struct SettingsScreen_Previews: PreviewProvider {
    static var previews: some View {
        NavigationView {
            SettingsScreen()
        }
    }
}

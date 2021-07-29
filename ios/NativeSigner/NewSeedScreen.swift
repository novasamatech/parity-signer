//
//  NewSeedScreen.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 28.7.2021.
//

import SwiftUI

struct NewSeedScreen: View {
    @State private var seedName: String = ""
    @State private var seedPhrase: String = ""
    @State private var recover: Bool = false
    init() {
        UITextView.appearance().backgroundColor = .clear
    }
    let seeds = Seeds()
    var body: some View {
        VStack {
            VStack {
                Text("Seed name").font(.title).foregroundColor(Color("textMainColor"))
                TextField("Seed name", text: $seedName).font(/*@START_MENU_TOKEN@*/.title/*@END_MENU_TOKEN@*/)
                    .foregroundColor(/*@START_MENU_TOKEN@*/Color("textEntryColor")/*@END_MENU_TOKEN@*/)
                .background(/*@START_MENU_TOKEN@*//*@PLACEHOLDER=View@*/Color("textFieldColor")/*@END_MENU_TOKEN@*/).border(/*@START_MENU_TOKEN@*/Color("borderSignalColor")/*@END_MENU_TOKEN@*/, width: /*@START_MENU_TOKEN@*/1/*@END_MENU_TOKEN@*/)
                Toggle(isOn: $recover) {
                    Text("Recover").font(.headline).foregroundColor(Color("textMainColor"))
                }
                if (recover) {
                    Text("Seed phrase").font(.title).foregroundColor(Color("textMainColor"))
                    TextEditor(text: $seedPhrase)
                .frame(height: 150.0)
                .background(/*@START_MENU_TOKEN@*//*@PLACEHOLDER=View@*/Color("textFieldColor")/*@END_MENU_TOKEN@*/)
                .foregroundColor(/*@START_MENU_TOKEN@*/Color("textEntryColor")/*@END_MENU_TOKEN@*/).background(/*@START_MENU_TOKEN@*//*@PLACEHOLDER=View@*/Color("textFieldColor")/*@END_MENU_TOKEN@*/).border(/*@START_MENU_TOKEN@*/Color("borderSignalColor")/*@END_MENU_TOKEN@*/, width: /*@START_MENU_TOKEN@*/1/*@END_MENU_TOKEN@*/)
                }
                Button(action: {
                    seeds.add(seedName: seedName, seedPhrase: seedPhrase)
                }) {
                    Text("Create")
                        .font(.largeTitle)
                }
            }.padding()
            Spacer()
            Footer()
        }
        .background(/*@START_MENU_TOKEN@*//*@PLACEHOLDER=View@*/Color("backgroundColor")/*@END_MENU_TOKEN@*/)
        .navigationTitle("New seed").navigationBarTitleDisplayMode(.inline).toolbar {
            ToolbarItem(placement: .navigationBarTrailing) {
                NavbarShield()
            }
        }.background(/*@START_MENU_TOKEN@*//*@PLACEHOLDER=View@*/Color("backgroundColor")/*@END_MENU_TOKEN@*/)
    }
}

struct NewSeedScreen_Previews: PreviewProvider {
    static var previews: some View {
        NavigationView {
            NewSeedScreen()
        }
    }
}

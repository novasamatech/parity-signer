//
//  NewSeedScreen.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 28.7.2021.
//

import SwiftUI

struct NewSeedScreen: View {
    @EnvironmentObject var data: SignerDataModel
    @State private var seedName: String = ""
    @State private var seedPhrase: String = ""
    @State private var recover: Bool = false
    init() {
        UITextView.appearance().backgroundColor = .clear
    }
    var body: some View {
        ZStack{
            RoundedRectangle(cornerRadius: 50).foregroundColor(/*@START_MENU_TOKEN@*/Color("backgroundCard")/*@END_MENU_TOKEN@*/)
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
                HStack{
                    Button(action: {
                        data.newSeed = false
                    }) {
                        Text("Cancel").font(.largeTitle)
                    }
                    Spacer()
                    Button(action: {
                        seedPhrase = data.addSeed(seedName: seedName, seedPhrase: seedPhrase)
                        data.newSeed = false
                    }) {
                        Text("Create")
                            .font(.largeTitle)
                    }
                }
            }.padding()
            Spacer()
        }
        }
        .background(/*@START_MENU_TOKEN@*//*@PLACEHOLDER=View@*/Color("backgroundColor")/*@END_MENU_TOKEN@*/).padding(.bottom, 100)
    }
}

struct NewSeedScreen_Previews: PreviewProvider {
    static var previews: some View {
        NewSeedScreen().previewLayout(.sizeThatFits)
    }
}

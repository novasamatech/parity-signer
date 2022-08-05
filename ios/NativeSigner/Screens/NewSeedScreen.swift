//
//  NewSeedScreen.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 28.7.2021.
//

import SwiftUI

struct NewSeedScreen: View {
    @State private var seedName: String = ""
    @FocusState private var nameFocused: Bool
    let content: MNewSeed
    let checkSeedCollision: (String) -> Bool
    let pushButton: (Action, String, String) -> Void

    var body: some View {
        VStack(alignment: .leading) {
            Text("DISPLAY NAME").font(FBase(style: .overline)).foregroundColor(Color("Text500"))
            ZStack {
                RoundedRectangle(cornerRadius: 8)
                    .stroke(Color("Border400"))
                    .frame(height: 39)
                TextField("Seed", text: $seedName, prompt: Text("Seed name"))
                    .focused($nameFocused)
                    .foregroundColor(Color("Text600"))
                    .font(FBase(style: .body2))
                    .disableAutocorrection(true)
                    .keyboardType(.asciiCapable)
                    .submitLabel(.done)
                    .onSubmit {
                        nameFocused = false
                        if (seedName != "") && !checkSeedCollision(seedName) {
                            pushButton(.goForward, seedName, "")
                        }
                    }
                    .onAppear(perform: {
                        nameFocused = content.keyboard
                    })
                    .padding(.horizontal, 8)
            }
            Text("Display name is visible only on this device").font(.callout)
            Spacer()
            BigButton(
                text: "Generate seed phrase",
                action: {
                    nameFocused = false
                    pushButton(.goForward, seedName, "")
                },
                isDisabled: (seedName == "") || checkSeedCollision(seedName)
            )
            Spacer()
        }.padding()
    }
}

/*
 struct NewSeedScreen_Previews: PreviewProvider {
 static var previews: some View {
 NewSeedScreen().previewLayout(.sizeThatFits)
 }
 }
 */

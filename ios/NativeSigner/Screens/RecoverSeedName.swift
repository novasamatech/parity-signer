//
//  RecoverSeedName.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 8.12.2021.
//

import SwiftUI

struct RecoverSeedName: View {
    @State private var seedName: String = ""
    @FocusState private var nameFocused: Bool
    let content: MRecoverSeedName
    let checkSeedCollision: (String) -> Bool
    let pushButton: (Action, String, String) -> Void

    var body: some View {
        VStack(alignment: .leading) {
            Text("DISPLAY NAME").font(FBase(style: .overline))
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
                        if (seedName != "") && !checkSeedCollision(seedName) {
                            pushButton(.goForward, seedName, "")
                        }
                    }
                    .onAppear(perform: {
                        seedName = content.seedName
                        nameFocused = content.keyboard
                    })
                    .padding(.horizontal, 8)
            }
            Text("Display name visible only to you").font(.callout)
            Spacer()
            BigButton(
                text: "Next",
                action: {
                    pushButton(.goForward, seedName, "")
                },
                isDisabled: (seedName == "") || checkSeedCollision(seedName)
            )
            Spacer()
        }.padding()
    }
}

/*
 struct RecoverSeedName_Previews: PreviewProvider {
 static var previews: some View {
 RecoverSeedName()
 }
 }
 */

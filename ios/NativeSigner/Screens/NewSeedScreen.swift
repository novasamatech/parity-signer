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
            Text("DISPLAY NAME").font(Fontstyle.overline.base).foregroundColor(Asset.text500.swiftUIColor)
            ZStack {
                RoundedRectangle(cornerRadius: 8)
                    .stroke(Asset.border400.swiftUIColor)
                    .frame(height: 39)
                TextField("Seed", text: $seedName, prompt: Text("Seed name"))
                    .focused($nameFocused)
                    .foregroundColor(Asset.text600.swiftUIColor)
                    .font(Fontstyle.body2.base)
                    .disableAutocorrection(true)
                    .keyboardType(.asciiCapable)
                    .submitLabel(.done)
                    .onSubmit {
                        nameFocused = false
                        if !seedName.isEmpty, !checkSeedCollision(seedName) {
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
                isDisabled: (seedName.isEmpty) || checkSeedCollision(seedName)
            )
            Spacer()
        }.padding()
    }
}

// struct NewSeedScreen_Previews: PreviewProvider {
// static var previews: some View {
// NewSeedScreen().previewLayout(.sizeThatFits)
// }
// }

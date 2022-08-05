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
    let navigationRequest: NavigationRequest

    var body: some View {
        VStack(alignment: .leading) {
            Text("DISPLAY NAME").font(Fontstyle.overline.base)
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
                        if !seedName.isEmpty, !checkSeedCollision(seedName) {
                            navigationRequest(.init(action: .goForward, details: seedName))
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
                    navigationRequest(.init(action: .goForward, details: seedName))
                },
                isDisabled: seedName.isEmpty || checkSeedCollision(seedName)
            )
            Spacer()
        }.padding()
    }
}

// struct RecoverSeedName_Previews: PreviewProvider {
// static var previews: some View {
// RecoverSeedName()
// }
// }

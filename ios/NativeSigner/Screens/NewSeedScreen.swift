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
    let navigationRequest: NavigationRequest

    var body: some View {
        VStack(alignment: .leading) {
            Localizable.displayName.text
                .font(PrimaryFont.labelS.font)
                .foregroundColor(Asset.text500.swiftUIColor)
            ZStack {
                RoundedRectangle(cornerRadius: 8)
                    .stroke(Asset.border400.swiftUIColor)
                    .frame(height: 39)
                TextField(Localizable.seed.string, text: $seedName, prompt: Localizable.seedName.text)
                    .focused($nameFocused)
                    .foregroundColor(Asset.text600.swiftUIColor)
                    .font(PrimaryFont.bodyL.font)
                    .disableAutocorrection(true)
                    .keyboardType(.asciiCapable)
                    .submitLabel(.done)
                    .onSubmit {
                        nameFocused = false
                        if !seedName.isEmpty, !checkSeedCollision(seedName) {
                            navigationRequest(.init(action: .goForward, details: seedName))
                        }
                    }
                    .onAppear(perform: {
                        nameFocused = content.keyboard
                    })
                    .padding(.horizontal, 8)
            }
            Localizable.displayNameIsVisibleOnlyOnThisDevice.text.font(.callout)
            Spacer()
            BigButton(
                text: Localizable.NewSeed.generate.key,
                action: {
                    nameFocused = false
                    navigationRequest(.init(action: .goForward, details: seedName))
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

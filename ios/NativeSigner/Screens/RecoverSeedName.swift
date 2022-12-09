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
            Localizable.displayName.text.font(PrimaryFont.labelS.font)
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
            Localizable.displayNameVisibleOnlyToYou.text.font(.callout)
            Spacer()
            BigButton(
                text: Localizable.next.key,
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

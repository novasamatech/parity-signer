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
    @EnvironmentObject var navigation: NavigationCoordinator

    var body: some View {
        VStack(alignment: .leading) {
            Localizable.displayName.text.font(PrimaryFont.labelS.font)
            ZStack {
                RoundedRectangle(cornerRadius: 8)
                    .stroke(Asset.fill12.swiftUIColor)
                    .frame(height: 39)
                TextField(Localizable.seed.string, text: $seedName, prompt: Localizable.seedName.text)
                    .focused($nameFocused)
                    .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                    .font(PrimaryFont.bodyL.font)
                    .disableAutocorrection(true)
                    .keyboardType(.asciiCapable)
                    .submitLabel(.done)
                    .onSubmit {
                        if !seedName.isEmpty, !ServiceLocator.seedsMediator.checkSeedCollision(seedName: seedName) {
                            navigation.perform(navigation: .init(action: .goForward, details: seedName))
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
            PrimaryButton(
                action: {
                    navigation.perform(navigation: .init(action: .goForward, details: seedName))
                },
                text: Localizable.next.key,
                style: .primary(isDisabled: .constant(
                    (seedName.isEmpty) || ServiceLocator.seedsMediator
                        .checkSeedCollision(seedName: seedName)
                ))
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

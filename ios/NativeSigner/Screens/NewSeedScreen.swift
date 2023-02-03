//
//  NewSeedScreen.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 28.7.2021.
//

import SwiftUI

struct NewSeedScreen: View {
    let content: MNewSeed
    @State private var seedName: String = ""
    @FocusState private var nameFocused: Bool
    @EnvironmentObject var navigation: NavigationCoordinator

    var body: some View {
        VStack(alignment: .leading, spacing: 0) {
            NavigationBarView(
                viewModel: .init(
                    title: nil,
                    leftButtons: [.init(
                        type: .xmark,
                        action: { navigation.perform(navigation: .init(action: .goBack)) }
                    )],
                    rightButtons: [.init(
                        type: .activeAction(
                            Localizable.NewSeed.Name.Action.next.key,
                            .constant(
                                (seedName.isEmpty) || ServiceLocator.seedsMediator
                                    .checkSeedCollision(seedName: seedName)
                            )
                        ),
                        action: {
                            self.nameFocused = false
                            self.navigation.perform(navigation: .init(action: .goForward, details: seedName))
                        }
                    )]
                )
            )
            VStack(alignment: .leading, spacing: 0) {
                Localizable.NewSeed.Name.Label.title.text
                    .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                    .font(PrimaryFont.titleL.font)
                    .padding(.top, Spacing.extraSmall)
                Localizable.NewSeed.Name.Label.header.text
                    .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                    .font(PrimaryFont.bodyL.font)
                    .padding(.vertical, Spacing.extraSmall)
                TextField("", text: $seedName)
                    .primaryTextFieldStyle(
                        Localizable.NewSeed.Name.Label.placeholder.string,
                        text: $seedName
                    )
                    .focused($nameFocused)
                    .disableAutocorrection(true)
                    .keyboardType(.asciiCapable)
                    .submitLabel(.done)
                    .onSubmit {
                        nameFocused = false
                        if !seedName.isEmpty, !ServiceLocator.seedsMediator.checkSeedCollision(seedName: seedName) {
                            navigation.perform(navigation: .init(action: .goForward, details: seedName))
                        }
                    }
                    .onAppear(perform: {
                        nameFocused = content.keyboard
                    })
                    .padding(.vertical, Spacing.medium)
                Localizable.NewSeed.Name.Label.footer.text
                    .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
                    .font(PrimaryFont.captionM.font)
                Spacer()
            }
            .padding(.horizontal, Spacing.large)
        }
    }
}

// struct NewSeedScreen_Previews: PreviewProvider {
// static var previews: some View {
// NewSeedScreen().previewLayout(.sizeThatFits)
// }
// }

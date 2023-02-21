//
//  RecoverSeedName.swift
//  Polkadot Vault
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
                            Localizable.RecoverSeedName.Action.next.key,
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
                Localizable.RecoverSeedName.Label.title.text
                    .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                    .font(PrimaryFont.titleL.font)
                    .padding(.top, Spacing.extraSmall)
                Localizable.RecoverSeedName.Label.content.text
                    .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                    .font(PrimaryFont.bodyL.font)
                    .padding(.vertical, Spacing.extraSmall)
                TextField("", text: $seedName)
                    .submitLabel(.done)
                    .primaryTextFieldStyle(
                        Localizable.seedName.string,
                        text: $seedName
                    )
                    .focused($nameFocused)
                    .onSubmit {
                        if !seedName.isEmpty, !ServiceLocator.seedsMediator.checkSeedCollision(seedName: seedName) {
                            navigation.perform(navigation: .init(action: .goForward, details: seedName))
                        }
                    }
                    .onAppear {
                        seedName = content.seedName
                        nameFocused = content.keyboard
                    }
                    .padding(.vertical, Spacing.medium)
                Localizable.RecoverSeedName.Label.footer.text
                    .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
                    .font(PrimaryFont.captionM.font)
                Spacer()
            }
            .padding(.horizontal, Spacing.large)
        }
    }
}

// struct RecoverSeedName_Previews: PreviewProvider {
// static var previews: some View {
// RecoverSeedName()
// }
// }

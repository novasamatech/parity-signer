//
//  SettingsRowView.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 12/12/2022.
//

import SwiftUI

struct SettingsRowView: View {
    let renderable: SettingsRowRenderable

    var body: some View {
        HStack(alignment: .center) {
            Text(renderable.title)
                .font(PrimaryFont.titleS.font)
                .foregroundColor(
                    renderable.isDestructive ? .accentRed400 : .textAndIconsPrimary
                )
            Spacer()
            if renderable.hasDetails {
                Image(.chevronRight)
                    .foregroundColor(.textAndIconsTertiary)
            }
        }
        .padding(.horizontal, Spacing.large)
        .background(.clear)
        .frame(height: Heights.settingsEntryHeight)
    }
}

struct SettingsRowRenderable: Equatable, Hashable {
    let item: SettingsItem
    let title: String
    let isDestructive: Bool
    let hasDetails: Bool
}

#if DEBUG
    struct SettingsRowView_Previews: PreviewProvider {
        static var previews: some View {
            VStack(spacing: 0) {
                SettingsRowView(renderable: .init(
                    item: .networks,
                    title: "Networks",
                    isDestructive: false,
                    hasDetails: true
                ))
                SettingsRowView(renderable: .init(
                    item: .wipe,
                    title: "Wipe All data",
                    isDestructive: true,
                    hasDetails: false
                ))
            }
        }
    }
#endif

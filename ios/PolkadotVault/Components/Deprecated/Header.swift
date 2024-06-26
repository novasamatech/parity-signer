//
//  Header.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 13.9.2021.
//

import SwiftUI

struct Header: View {
    let back: Bool
    let screenLabel: String
    let screenNameType: ScreenNameType?
    let rightButton: RightButton?
    @EnvironmentObject private var navigation: NavigationCoordinator

    var body: some View {
        VStack {
            Spacer()
            HStack {
                HStack(spacing: 8.0) {
                    if back {
                        Button(
                            action: {
                                navigation.perform(navigation: .init(action: .goBack))
                                // Temporar fix for Private Key Export - clears intermediate state for Key Details
                                navigation.currentKeyDetails = nil
                            },
                            label: {
                                Image(
                                    rightButton == .multiSelect ? .xmark : .chevron,
                                    variant: rightButton == .multiSelect ? nil : .left
                                )
                                .imageScale(.large)
                                .foregroundColor(Asset.textAndIconsSecondary.swiftUIColor)
                            }
                        )
                    }
                    Spacer()
                }
                .frame(width: 72.0)
                Spacer()
                Text(screenLabel)
                    .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                    .font(screenNameType == .h1 ? PrimaryFont.titleM.font : PrimaryFont.labelM.font)
                    .tracking(0.1)
                Spacer()
                HStack(spacing: 8.0) {
                    Spacer()
                    Button(
                        action: {
                            navigation.perform(navigation: .init(action: .rightButtonAction))
                        },
                        label: {
                            switch rightButton {
                            case .none,
                                 .multiSelect:
                                EmptyView()
                            default:
                                Image(.ellipsis)
                                    .imageScale(.large)
                                    .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                            }
                        }
                    )
                }
                .frame(width: 72.0)
            }
        }
        .frame(height: 32.0)
        .padding(Spacing.extraSmall)
    }
}

// struct Header_Previews: PreviewProvider {
// static var previews: some View {
// Header().previewLayout(.sizeThatFits)
// }
// }

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
    let alert: Bool
    let alertShow: () -> Void
    @EnvironmentObject private var navigation: NavigationCoordinator
    @EnvironmentObject private var data: SignerDataModel

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
                                .foregroundColor(Asset.text500.swiftUIColor)
                            }
                        )
                    }
                    Spacer()
                }
                .frame(width: 72.0)
                Spacer()
                Text(screenLabel)
                    .foregroundColor(Asset.text600.swiftUIColor)
                    .font(screenNameType == .h1 ? PrimaryFont.titleM.font : PrimaryFont.labelM.font)
                    .tracking(0.1)
                if rightButton == .multiSelect {
                    Button(
                        action: {
                            navigation.perform(navigation: .init(action: .selectAll))
                        },
                        label: {
                            SmallButton(text: Localizable.selectAll.key)
                        }
                    )
                }
                Spacer()
                HStack(spacing: 8.0) {
                    Spacer()
                    Button(
                        action: {
                            navigation.perform(navigation: .init(action: .rightButtonAction))
                        },
                        label: {
                            switch rightButton {
                            case .backup:
                                Image(.ellipsis)
                                    .imageScale(.large)
                                    .foregroundColor(Asset.action400.swiftUIColor)
                            case .logRight:
                                Image(.ellipsis)
                                    .imageScale(.large)
                                    .foregroundColor(Asset.action400.swiftUIColor)
                            case .multiSelect:
                                EmptyView()
                            case .none:
                                EmptyView()
                            default:
                                Image(.ellipsis)
                                    .imageScale(.large)
                                    .foregroundColor(Asset.action400.swiftUIColor)
                            }
                        }
                    )
                    NavbarShield(
                        alert: alert,
                        resetWarningAction: ResetConnectivtyWarningsAction(alert: $data.alert)
                    )
                }
                .frame(width: 72.0)
            }
        }
        .frame(height: 32.0)
        .padding(.all, 8.0)
    }
}

// struct Header_Previews: PreviewProvider {
// static var previews: some View {
// Header().previewLayout(.sizeThatFits)
// }
// }

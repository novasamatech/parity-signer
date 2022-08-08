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
    let canaryDead: Bool
    let alertShow: () -> Void
    let navigationRequest: NavigationRequest
    var body: some View {
        VStack {
            Spacer()
            HStack {
                HStack(spacing: 8.0) {
                    if back {
                        Button(
                            action: {
                                navigationRequest(.init(action: .goBack))
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
                    .font(screenNameType == .h1 ? Fontstyle.header2.base : Fontstyle.header4.base)
                    .tracking(0.1)
                if rightButton == .multiSelect {
                    Button(
                        action: {
                            navigationRequest(.init(action: .selectAll))
                        },
                        label: {
                            SmallButton(text: "Select all")
                        }
                    )
                }
                Spacer()
                HStack(spacing: 8.0) {
                    Spacer()
                    Button(
                        action: {
                            if alert, rightButton == .newSeed {
                                alertShow()
                            } else {
                                navigationRequest(.init(action: .rightButtonAction))
                            }
                        },
                        label: {
                            switch rightButton {
                            case .newSeed:
                                Image(.plus, variant: .circle)
                                    .imageScale(.large)
                                    .foregroundColor(Asset.action400.swiftUIColor)
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
                        canaryDead: canaryDead,
                        alert: alert,
                        navigationRequest: navigationRequest
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

//
//  Footer.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 26.7.2021.
//

import SwiftUI

struct KeySymbol: View {
    var body: some View {
        VStack {
            Text("accounts").font(FontFamily.Web3.regular.swiftUIFont(size: 20))
        }
    }
}

struct WrenchSymbol: View {
    var body: some View {
        VStack {
            Image(.gearshape, variant: .fill).imageScale(.medium)
        }
    }
}

struct Footer: View {
    let footerButton: FooterButton?
    let navigationRequest: NavigationRequest
    var body: some View {
        HStack {
            Button(
                action: {
                    navigationRequest(.init(action: .navbarLog))
                },
                label: {
                    VStack(alignment: .center) {
                        Image(.rectangle, variants: [.grid, .oneByTwo, .fill]).imageScale(.medium)
                            .padding(.top, 4.0)
                            .padding(.bottom, 1.0)
                        Text("Log")
                    }
                    .foregroundColor(buttonColor(active: footerButton == .log))
                }
            )
            Spacer()
            Button(
                action: {
                    navigationRequest(.init(action: .navbarScan))
                },
                label: {
                    VStack {
                        Image(.viewfinder).imageScale(.medium)
                            .padding(.top, 4.0)
                            .padding(.bottom, 1.0)
                        Text("Scanner")
                    }
                    .foregroundColor(buttonColor(active: footerButton == .scan))
                }
            )
            Spacer()
            Button(
                action: {
                    navigationRequest(.init(action: .navbarKeys))
                },
                label: {
                    VStack {
                        KeySymbol()
                        Text("Keys")
                    }
                    .foregroundColor(buttonColor(active: footerButton == .keys))
                }
            )
            Spacer()
            Button(
                action: {
                    navigationRequest(.init(action: .navbarSettings))
                },
                label: {
                    VStack {
                        WrenchSymbol()
                            .padding(.top, 4.0)
                            .padding(.bottom, 1.0)
                        Text("Settings")
                    }
                    .foregroundColor(buttonColor(active: footerButton == .settings))
                }
            )
        }.font(.footnote)
    }
}

func buttonColor(active: Bool) -> Color {
    active ? Asset.text600.swiftUIColor : Asset.text300.swiftUIColor
}

// func buttonLabelColor(active: Bool) -> Color {
// return active ? Asset.text600.swiftUIColor : Asset.text300.swiftUIColor
// }

// struct Footer_Previews: PreviewProvider {
// static var previews: some View {
// Footer().previewLayout(.sizeThatFits)
// }
// }

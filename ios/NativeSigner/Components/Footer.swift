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
            Text("accounts").font(Font.custom("Web3-Regular", size: 20))
        }
    }
}

struct WrenchSymbol: View {
    var body: some View {
        VStack {
            Image(systemName: "gearshape.fill").imageScale(.medium)
        }
    }
}

struct Footer: View {
    let footerButton: FooterButton?
    let pushButton: (Action, String, String) -> Void
    var body: some View {
        HStack {
            Button(
                action: {
                    pushButton(.navbarLog, "", "")
                },
                label: {
                    VStack(alignment: .center) {
                        Image(systemName: "rectangle.grid.1x2.fill").imageScale(.medium)
                            .padding(.top, 4.0)
                            .padding(.bottom, 1.0)
                        Text("Log")
                    }
                    .foregroundColor(buttonColor(active: footerButton == .log))
                })
            Spacer()
            Button(
                action: {
                    pushButton(.navbarScan, "", "")
                },
                label: {
                    VStack {
                        Image(systemName: "viewfinder").imageScale(.medium)
                            .padding(.top, 4.0)
                            .padding(.bottom, 1.0)
                        Text("Scanner")
                    }
                    .foregroundColor(buttonColor(active: footerButton == .scan))
                })
            Spacer()
            Button(
                action: {
                    pushButton(.navbarKeys, "", "")
                },
                label: {
                    VStack {
                        KeySymbol()
                        Text("Keys")
                    }
                    .foregroundColor(buttonColor(active: footerButton == .keys))
                })
            Spacer()
            Button(
                action: {
                    pushButton(.navbarSettings, "", "")
                },
                label: {
                    VStack {
                        WrenchSymbol()
                            .padding(.top, 4.0)
                            .padding(.bottom, 1.0)
                        Text("Settings")
                    }
                    .foregroundColor(buttonColor(active: footerButton == .settings))
                })
        }.font(.footnote)
    }
}

func buttonColor(active: Bool) -> Color {
    return active ? Color("Text600") : Color("Text300")
}

/*
 func buttonLabelColor(active: Bool) -> Color {
 return active ? Color("Text600") : Color("Text300")
 }
 */

/*
 struct Footer_Previews: PreviewProvider {
 static var previews: some View {
 Footer().previewLayout(.sizeThatFits)
 }
 }
 */

//
//  LogMenu.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 17.12.2021.
//

import SwiftUI

struct LogMenu: View {
    @State private var clearConfirm = false
    var content: MLogRight
    let navigationRequest: NavigationRequest
    var body: some View {
        VStack {
            Spacer()
            VStack {
                HeaderBar(line1: "LOG", line2: "Checksum: " + content.checksum)
                MenuButtonsStack {
                    BigButton(
                        text: "Add note",
                        action: {
                            navigationRequest(.init(action: .createLogComment))
                        }
                    )
                    BigButton(
                        text: "Clear log",
                        isShaded: true,
                        isDangerous: true,
                        action: {
                            clearConfirm = true
                        }
                    )
                }
            }
            .padding([.leading, .trailing, .top])
            .padding(.bottom, 24)
            .background(Asset.bg000.swiftUIColor)
            .alert(isPresented: $clearConfirm, content: {
                Alert(
                    title: Text("Clear log?"),
                    message: Text("Do you want this Signer to forget all logged events? This is not reversible."),
                    primaryButton: .cancel(Text("Cancel")),
                    secondaryButton: .destructive(
                        Text("Clear log"),
                        action: { navigationRequest(.init(action: .clearLog)) }
                    )
                )
            })
        }
    }
}

// struct LogMenu_Previews: PreviewProvider {
// static var previews: some View {
// LogMenu()
// }
// }

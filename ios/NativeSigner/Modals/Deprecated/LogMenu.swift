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
                HeaderBar(
                    line1: Localizable.Log.uppercased.key,
                    line2: LocalizedStringKey(Localizable.checksumContent(content.checksum))
                )
                MenuButtonsStack {
                    BigButton(
                        text: Localizable.addNote.key,
                        action: {
                            navigationRequest(.init(action: .createLogComment))
                        }
                    )
                    BigButton(
                        text: Localizable.clearLog.key,
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
            .background(Asset.backgroundPrimary.swiftUIColor)
            .alert(isPresented: $clearConfirm, content: {
                Alert(
                    title: Localizable.clearLogQuestion.text,
                    message: Localizable.doYouWantThisSignerToForgetAllLoggedEventsThisIsNotReversible.text,
                    primaryButton: .cancel(Localizable.cancel.text),
                    secondaryButton: .destructive(
                        Localizable.clearLog.text,
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

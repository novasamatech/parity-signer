//
//  SettingsScreen.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 29.7.2021.
//

import SwiftUI

struct SettingsScreen: View {
    @State private var wipe = false
    @State private var jailbreak = false
    let content: MSettings
    let doWipe: () -> Void
    let navigationRequest: NavigationRequest

    private let appVersion = Bundle.main.infoDictionary?["CFBundleShortVersionString"] as? String

    var body: some View {
        VStack(spacing: 2) {
            Button(
                action: {
                    navigationRequest(.init(action: .manageNetworks))
                },
                label: {
                    SettingsCardTemplate(text: "Networks")
                }
            )
            Button(
                action: {
                    navigationRequest(.init(action: .backupSeed))
                },
                label: {
                    SettingsCardTemplate(text: "Backup keys")
                }
            )
            Button(
                action: { navigationRequest(.init(action: .viewGeneralVerifier)) },
                label: {
                    VStack {
                        HStack {
                            Text("Verifier certificate").font(Fontstyle.header1.base)
                                .foregroundColor(Asset.text600.swiftUIColor)
                            Spacer()
                        }
                        VStack {
                            if content.publicKey != nil {
                                AddressCard(address: Address(
                                    base58: "encryption: " + (content.encryption ?? "unknown"),
                                    path: content.publicKey?.truncateMiddle(length: 8) ?? "",
                                    hasPwd: false,
                                    identicon: content.identicon ?? [],
                                    seedName: "",
                                    multiselect: false,
                                    secretExposed: false
                                ))
                            } else {
                                if let errorMessage = content.error {
                                    Text("Error!").foregroundColor(Asset.signalDanger.swiftUIColor)
                                        .font(Fontstyle.header4.base)
                                    Text(errorMessage).foregroundColor(Asset.signalDanger.swiftUIColor)
                                        .font(Fontstyle.body2.base)
                                } else {
                                    AddressCard(address: Address(
                                        base58: "",
                                        path: "None",
                                        hasPwd: false,
                                        identicon: [],
                                        seedName: "",
                                        multiselect: false,
                                        secretExposed: false
                                    ))
                                }
                            }
                        }
                    }
                    .padding()
                }
            )
            Button(
                action: {
                    wipe = true
                },
                label: {
                    SettingsCardTemplate(
                        text: "Wipe all data",
                        danger: true
                    )
                }
            )
            .alert(isPresented: $wipe, content: {
                Alert(
                    title: Text("Wipe ALL data?"),
                    message: Text("Factory reset the Signer app. This operation can not be reverted!"),
                    primaryButton: .cancel(),
                    secondaryButton: .destructive(
                        Text("Wipe"),
                        action: {
                            doWipe()
                        }
                    )
                )
            })
            Button(
                action: {
                    navigationRequest(.init(action: .showDocuments))
                },
                label: {
                    SettingsCardTemplate(text: "About")
                }
            )
            SettingsCardTemplate(
                text: "App version: " + (appVersion ?? "Unknown!"),
                withIcon: false,
                withBackground: false
            )
        }
    }
}

// struct SettingsScreen_Previews: PreviewProvider {
// static var previews: some View {
// NavigationView {
// SettingsScreen()
// }
// }
// }

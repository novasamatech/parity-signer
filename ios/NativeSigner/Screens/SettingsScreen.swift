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

    var body: some View {
        VStack(spacing: 2) {
            Button(
                action: {
                    navigationRequest(.init(action: .manageNetworks))
                },
                label: {
                    SettingsCardTemplate(text: Localizable.networks.key)
                }
            )
            Button(
                action: {
                    navigationRequest(.init(action: .backupSeed))
                },
                label: {
                    SettingsCardTemplate(text: Localizable.backupKeys.key)
                }
            )
            Button(
                action: { navigationRequest(.init(action: .viewGeneralVerifier)) },
                label: {
                    VStack {
                        HStack {
                            Localizable.verifierCertificate.text
                                .font(Fontstyle.header1.base)
                                .foregroundColor(Asset.text600.swiftUIColor)
                            Spacer()
                        }
                        VStack {
                            if content.publicKey != nil {
                                AddressCard(card: MAddressCard(
                                    base58: "encryption: " + (content.encryption ?? "unknown"),
                                    address: Address(
                                        path: content.publicKey?.truncateMiddle(length: 8) ?? "",
                                        hasPwd: false,
                                        identicon: .svg(image: content.identicon?.svgPayload ?? []),
                                        seedName: "",
                                        secretExposed: false
                                    ),
                                    multiselect: false
                                ))
                            } else {
                                if let errorMessage = content.error {
                                    Localizable.errorCapitalised.text
                                        .foregroundColor(Asset.signalDanger.swiftUIColor)
                                        .font(Fontstyle.header4.base)
                                    Text(errorMessage)
                                        .foregroundColor(Asset.signalDanger.swiftUIColor)
                                        .font(Fontstyle.body2.base)
                                } else {
                                    AddressCard(card: MAddressCard(
                                        base58: "",
                                        address: Address(
                                            path: "None",
                                            hasPwd: false,
                                            identicon: .svg(image: []),
                                            seedName: "",
                                            secretExposed: false
                                        ),
                                        multiselect: false
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
                        text: Localizable.wipeAllDataAlt.key,
                        danger: true
                    )
                }
            )
            .alert(isPresented: $wipe, content: {
                Alert(
                    title: Localizable.wipeALLData.text,
                    message: Localizable.FactoryResetTheSignerApp.thisOperationCanNotBeReverted.text,
                    primaryButton: .cancel(),
                    secondaryButton: .destructive(
                        Localizable.wipe.text,
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
                    SettingsCardTemplate(text: Localizable.about.key)
                }
            )
            SettingsCardTemplate(
                text: LocalizedStringKey(Localizable.appVersion(ApplicationInformation.cfBundleShortVersionString)),
                withIcon: false,
                withBackground: false
            )
            Spacer().frame(idealHeight: .infinity)
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

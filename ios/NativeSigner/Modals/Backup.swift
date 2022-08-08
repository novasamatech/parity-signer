//
//  Backup.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 10.12.2021.
//

import SwiftUI

struct Backup: View {
    let content: MBackup
    let alert: Bool
    let getSeedForBackup: (String) -> String
    let navigationRequest: NavigationRequest
    let timer = Timer.publish(every: 1, on: .main, in: .common).autoconnect()

    @State private var secret: String = ""
    @State private var countdown = 60
    @State private var failure = false

    var body: some View {
        ZStack {
            RoundedRectangle(cornerRadius: 20.0)
                .foregroundColor(Asset.bg200.swiftUIColor)
            VStack {
                ZStack {
                    HeaderBar(
                        line1: Localizable.backup.key,
                        line2: LocalizedStringKey(content.seedName)
                    )
                    HStack {
                        Spacer()
                        Button(
                            action: {
                                navigationRequest(.init(action: .goBack))
                            },
                            label: {
                                Image(.xmark).imageScale(.large)
                                    .foregroundColor(Asset.text300.swiftUIColor)
                            }
                        )
                    }
                }
                ScrollView {
                    VStack {
                        HStack {
                            Localizable.seedPhrase.text
                                .foregroundColor(Asset.text300.swiftUIColor)
                                .font(Fontstyle.overline.base)
                            Spacer()
                        }
                        HStack {
                            Text(secret)
                                .font(.system(size: 16, weight: .semibold, design: .monospaced))
                                .foregroundColor(
                                    failure ? Asset.signalDanger.swiftUIColor : Asset.crypto400
                                        .swiftUIColor
                                )
                                .padding(8)
                            Spacer()
                        }
                        .onAppear {
                            secret = getSeedForBackup(content.seedName)
                            if secret.isEmpty {
                                failure = true
                                countdown = -1
                                secret = alert ?
                                    Localizable.Seed.Alert.networkConnected.string :
                                    Localizable.Seed.Alert.unknown.string
                            }
                            UIApplication.shared.isIdleTimerDisabled = true
                        }
                        .onDisappear {
                            secret = ""
                            UIApplication.shared.isIdleTimerDisabled = false
                        }
                        .background(
                            RoundedRectangle(cornerRadius: 8)
                                .foregroundColor(
                                    countdown > 0 ? Asset.crypto100.swiftUIColor :
                                        failure ? Asset.bgDanger.swiftUIColor :
                                        Asset.bg300.swiftUIColor
                                )
                        )
                        HStack {
                            Localizable.derivedKeys.text
                                .foregroundColor(Asset.text300.swiftUIColor)
                                .font(Fontstyle.overline.base)
                            Spacer()
                        }
                        LazyVStack {
                            ForEach(
                                content.derivations.sorted(by: { $0.networkOrder < $1.networkOrder }),
                                id: \.networkOrder
                            ) { pack in
                                VStack {
                                    HStack {
                                        NetworkCard(
                                            title: pack.networkTitle,
                                            logo: pack.networkLogo,
                                            fancy: true
                                        )
                                        .padding(.top, 10)
                                        Spacer()
                                    }
                                    ForEach(pack.idSet.sorted(by: { $0.path < $1.path }), id: \.self) { record in
                                        HStack {
                                            Text((record.path.isEmpty && !record.hasPwd) ? "seed key" : record.path)
                                                .foregroundColor(Asset.crypto400.swiftUIColor)
                                                .font(Fontstyle.body2.crypto)
                                            if record.hasPwd {
                                                Localizable.Path.delimeter.text
                                                    .foregroundColor(Asset.crypto400.swiftUIColor)
                                                    .font(Fontstyle.body2.crypto)
                                                Image(.lock).foregroundColor(Asset.crypto400.swiftUIColor)
                                                    .font(Fontstyle.body2.crypto)
                                            }
                                            Spacer()
                                        }.padding(8)
                                    }
                                }
                            }
                        }
                    }.padding(.bottom, 132)
                }
            }.padding(16)
            if countdown > 0 {
                VStack {
                    Spacer()
                    ZStack {
                        BigButton(
                            text: LocalizedStringKey(Localizable.hideSeedPhraseIn(String(countdown))),
                            isShaded: true
                        ) {
                            countdown = 0
                            secret = Localizable.Seed.Alert.timeout.string
                        }
                        .onReceive(timer) { _ in
                            if countdown > 0 { countdown -= 1 }
                            if countdown == 0 {
                                secret = Localizable.Seed.Alert.timeout.string
                            }
                        }.padding(.horizontal, 16)
                    }.padding(.bottom, 75)
                }
            }
        }
    }
}

// struct Backup_Previews: PreviewProvider {
// static var previews: some View {
// Backup()
// }
// }

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
    let pushButton: (Action, String, String) -> Void
    @State var secret: String = ""
    let timer = Timer.publish(every: 1, on: .main, in: .common).autoconnect()
    @State var countdown = 60
    @State var failure = false
    var body: some View {
        ZStack {
            RoundedRectangle(cornerRadius: 20.0).foregroundColor(Asset.bg200.swiftUIColor)
            VStack {
                ZStack {
                    HeaderBar(line1: "Backup", line2: content.seedName)
                    HStack {
                        Spacer()
                        Button(
                            action: {
                                pushButton(.goBack, "", "")
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
                            Text("SEED PHRASE").foregroundColor(Asset.text300.swiftUIColor)
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
                                    "Network connected! Seeds are not available now. " +
                                    "Please enable airplane mode and disconnect all cables to access the seed phrase." :
                                    "Seeds are not available now! Come back again to access them."
                            }
                            UIApplication.shared.isIdleTimerDisabled = true
                        }
                        .onDisappear {
                            secret = ""
                            UIApplication.shared.isIdleTimerDisabled = false
                        }
                        .background(
                            RoundedRectangle(cornerRadius: 8)
                                .foregroundColor(Color(
                                    countdown > 0 ? "Crypto100" :
                                        failure ? "BgDanger" :
                                        "Bg300"
                                ))
                        )
                        HStack {
                            Text("DERIVED KEYS").foregroundColor(Asset.text300.swiftUIColor)
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
                                                Text("///").foregroundColor(Asset.crypto400.swiftUIColor)
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
                            text: "Hide seed phrase in " + String(countdown) + "s",
                            isShaded: true
                        ) {
                            countdown = 0
                            secret = "Time out\n\nCome back again\nto see the seed phrase!"
                        }
                        .onReceive(timer) { _ in
                            if countdown > 0 { countdown -= 1 }
                            if countdown == 0 {
                                secret = "Time out\n\nCome back again\nto see the seed phrase!"
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

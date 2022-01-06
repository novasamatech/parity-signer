//
//  Backup.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 10.12.2021.
//

import SwiftUI

struct Backup: View {
    @EnvironmentObject var data: SignerDataModel
    let content: MBackup
    @State var secret: String = ""
    let timer = Timer.publish(every: 1, on: .main, in: .common).autoconnect()
    @State var countdown = 60
    //TODO: chop chop chop
    var body: some View {
        ZStack {
            RoundedRectangle(cornerRadius: 20.0).foregroundColor(Color("Bg200"))
            VStack{
                ZStack {
                    HeaderBar(line1: "Backup", line2: content.seed_name)
                    HStack {
                        Spacer()
                        Button(action: {
                            data.pushButton(buttonID: .GoBack)
                        }) {
                            Image(systemName: "xmark").imageScale(.large).foregroundColor(Color("Text300"))
                        }
                    }
                }
                ScrollView{
                    VStack {
                        Text("SEED PHRASE").foregroundColor(Color("Text300")).font(FBase(style: .overline))
                        ZStack {
                            //RoundedRectangle(cornerRadius: 8).foregroundColor(Color(countdown>0 ? "Crypto100" : "Bg300")).frame(height: 200)
                            Text(secret)
                                .font(.system(size: 16, weight: .semibold, design: .monospaced))
                                .foregroundColor(Color("Crypto400"))
                                .padding(8)
                        }
                        .onAppear{
                            secret = data.getSeed(seedName: content.seed_name, backup: true)
                        }
                        .onDisappear{
                            secret = ""
                            data.seedBackup = ""
                        }
                        .background(RoundedRectangle(cornerRadius: 8).foregroundColor(Color(countdown>0 ? "Crypto100" : "Bg300")))
                        Text("DERIVED KEYS").foregroundColor(Color("Text300")).font(FBase(style: .overline))
                        LazyVStack {
                            ForEach(content.derivations.sorted(by: {$0.network_order < $1.network_order}), id: \.network_order) {
                                pack in
                                VStack {
                                    HStack {
                                        NetworkCard(title: pack.network_title, logo: pack.network_logo).padding(.top, 10)
                                        Spacer()
                                    }
                                    ForEach(pack.id_set.sorted(by: {$0.path < $1.path}), id: \.self) {
                                        record in
                                        HStack{
                                            Text((record.path == "" && !record.has_pwd) ? "seed key" : record.path)
                                                .foregroundColor(Color("Crypto400"))
                                                .font(FCrypto(style: .body2))
                                            if record.has_pwd {
                                                Text("///").foregroundColor(Color("Crypto400"))
                                                    .font(FCrypto(style: .body2))
                                                Image(systemName: "lock").foregroundColor(Color("Crypto400"))
                                                    .font(FCrypto(style: .body2))
                                            }
                                            Spacer()
                                        }.padding(8)
                                    }
                                }
                            }
                        }
                    }.padding(.bottom, 80)
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
                        .onReceive(timer) { input in
                            if countdown > 0 {countdown -= 1}
                            if countdown == 0 {
                                secret = "Time out\n\nCome back again\nto see the seed phrase!"
                            }
                        }.padding(.horizontal, 16)
                        /*
                         RoundedRectangle(cornerRadius: 8).foregroundColor(Color("Bg300")).frame(height: 40)
                         Text("Hide seed phrase in " + String(countdown) + "s")
                         .onReceive(timer) { input in
                         countdown -= 1
                         if countdown == 0 {
                         secret = "Time out\n\nCome back again\nto see the seed phrase!"
                         }
                         }
                         .foregroundColor(Color("Action400"))
                         .font(FBase(style: .button))
                         */
                    }
                }
            }
        }
    }
}

/*
 struct Backup_Previews: PreviewProvider {
 static var previews: some View {
 Backup()
 }
 }
 */

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
            RoundedRectangle(cornerRadius: 20.0).foregroundColor(Color("Bg000"))
            VStack{
                HeaderBar(line1: "Backup", line2: content.seed_name)
                ScrollView{
                    VStack {
                        ZStack {
                            RoundedRectangle(cornerRadius: 8).stroke(Color("Crypto400")).foregroundColor(Color("Bg000")).frame(height: 200)
                            Text(secret)
                                .font(.system(size: 16, weight: .semibold, design: .monospaced))
                                .foregroundColor(Color("Crypto400"))
                                .padding(8)
                        }
                        .onAppear{
                            if data.seedBackup == "" {
                                secret = data.getSeed(seedName: content.seed_name, backup: true)
                            } else {
                                secret = data.seedBackup
                            }
                        }
                        .onDisappear{
                            secret = ""
                            data.seedBackup = ""
                        }
                        LazyVStack {
                            ForEach(content.derivations.sorted(by: {$0.network_order < $1.network_order}), id: \.network_order) {
                                pack in
                                VStack {
                                    NetworkCard(title: pack.network_title, logo: pack.network_logo)
                                    ForEach(pack.id_set.sorted(by: {$0.path < $1.path}), id: \.self) {
                                        record in
                                        HStack{
                                            Text(record.path)
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
                    }
                }
            }
            if countdown > 0 {
                VStack {
                    Spacer()
                    ZStack {
                        RoundedRectangle(cornerRadius: 8).foregroundColor(Color("Bg300")).frame(height: 40)
                        Text("Clear in " + String(countdown) + "s")
                            .onReceive(timer) { input in
                                countdown -= 1
                                if countdown == 0 {
                                    secret = "time out"
                                }
                            }
                            .foregroundColor(Color("Action400"))
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

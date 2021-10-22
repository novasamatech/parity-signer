//
//  SeedBackup.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 14.9.2021.
//

import SwiftUI

struct SeedBackup: View {
    @EnvironmentObject var data: SignerDataModel
    var body: some View {
        ZStack{
            ModalBackdrop()
            VStack {
                SeedCardForManager(seedName: data.selectedSeed)
                Text("Backup your seed phrase!").font(.headline)
                Text("Keep your seed phrase in safe place; anyone could restore accounts using this seed phrase; there is no other way to restore accounts.").font(.footnote)
                ZStack {
                    RoundedRectangle(cornerRadius: 8).stroke(Color("AccentColor")).foregroundColor(Color("backgroundColor")).frame(height: 200)
                    Text(data.selectedSeed == "" ? "" : data.getRememberedSeedPhrate())
                        .font(.system(size: 16, weight: .semibold, design: .monospaced))
                        .foregroundColor(Color("cryptoColor"))
                        .padding(8)
                }
            }
        }.onDisappear {
            data.seedBackup = ""
            data.selectSeed(seedName: "")
        }
    }
}

/*
 struct SeedBackup_Previews: PreviewProvider {
 static var previews: some View {
 SeedBackup()
 }
 }*/

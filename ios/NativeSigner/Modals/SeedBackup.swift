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
                Text("Backup your seed phrase!").font(.headline)
                Text("Keep your seed phrase in safe place; anyone could restore accounts using this seed phrase; there is no other way to restore accounts.").font(.subheadline)
                Text(data.getRememberedSeedPhrate()).font(.callout).padding()
                Button(action: {data.totalRefresh()}) {
                    Text("Done").font(.largeTitle)
                }
            }
        }
    }
}

/*
struct SeedBackup_Previews: PreviewProvider {
    static var previews: some View {
        SeedBackup()
    }
}*/

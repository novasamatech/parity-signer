//
//  RecoverSeedName.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 8.12.2021.
//

import SwiftUI

struct RecoverSeedName: View {
    @EnvironmentObject var data: SignerDataModel
    @State private var seedName: String = ""
    @FocusState private var nameFocused: Bool
    var content: MRecoverSeedName
    
    var body: some View {
        VStack(alignment: .leading) {
            Text("DISPLAY NAME").font(FBase(style: .overline))
            ZStack {
                RoundedRectangle(cornerRadius: 8)
                    .stroke(Color("Border400"))
                    //.foregroundColor(Color("Border400"))
                    .frame(height: 39)
                TextField("Seed", text: $seedName, prompt: Text("Seed name"))
                    .focused($nameFocused)
                    .foregroundColor(Color("Text600"))
                    .font(FBase(style: .body2))
                    .disableAutocorrection(true)
                    .keyboardType(.asciiCapable)
                    .submitLabel(.done)
                    .onChange(of: seedName, perform: { newName in
                        data.lastError = ""
                    })
                    .onSubmit {
                        if (seedName != "") && (!seedName.contains(",")) {
                            data.pushButton(buttonID: .GoForward, details: seedName)
                        }
                    }
                    .onAppear(perform: {
                        seedName = content.seed_name
                        nameFocused = content.keyboard
                    })
                    .padding(.horizontal, 8)
            }
            Text("Display name visible only to you").font(.callout)
            Text(data.lastError).foregroundColor(Color("SignalDanger"))
            Spacer()
            BigButton(
                text: "Next",
                action: {
                    if !data.checkSeedCollision(seedName: seedName) {
                        data.pushButton(buttonID: .GoForward, details: seedName)
                    } else {
                        data.lastError = "This seed name already exists"
                    }
                },
                isDisabled: (seedName == "") || (seedName.contains(","))
            )
            Spacer()
        }.padding()
    }
}

/*
 struct RecoverSeedName_Previews: PreviewProvider {
 static var previews: some View {
 RecoverSeedName()
 }
 }
 */

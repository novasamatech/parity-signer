//
//  NewSeedScreen.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 28.7.2021.
//

import SwiftUI

struct NewSeedScreen: View {
    @EnvironmentObject var data: SignerDataModel
    @State private var seedName: String = ""
    @FocusState private var nameFocused: Bool
    var content: MNewSeed
    
    var body: some View {
        VStack(alignment: .leading) {
            Text("DISPLAY NAME").font(FBase(style: .overline)).foregroundColor(Color("Text500"))
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
                    .onChange(of: seedName, perform: { _ in
                        data.lastError = ""
                    })
                    .onSubmit {
                        nameFocused = false
                        if (seedName != "") && !data.checkSeedCollision(seedName: seedName.encode64()) {
                            data.pushButton(action: .goForward, details: seedName.encode64())
                        }
                    }
                    .onAppear(perform: {
                        nameFocused = content.keyboard
                    })
                    .onDisappear {
                        data.lastError = ""
                    }
                    .padding(.horizontal, 8)
            }
            Text("Display name is visible only on this device").font(.callout)
            Text(data.lastError).foregroundColor(.red)
            Spacer()
            BigButton(
                text: "Generate seed phrase",
                action: {
                    nameFocused = false
                    data.pushButton(action: .goForward, details: seedName.encode64())
                },
                isDisabled: (seedName == "") || data.checkSeedCollision(seedName: seedName.encode64())
            )
            Spacer()
        }.padding()
    }
}

/*
 struct NewSeedScreen_Previews: PreviewProvider {
 static var previews: some View {
 NewSeedScreen().previewLayout(.sizeThatFits)
 }
 }
 */

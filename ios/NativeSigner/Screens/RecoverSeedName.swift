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
    
    init() {
        UITextView.appearance().backgroundColor = .clear
    }
    
    var body: some View {
        VStack(alignment: .leading) {
            Text("DISPLAY NAME").font(FBase(style: .overline))
            ZStack {
                RoundedRectangle(cornerRadius: 8)
                    .stroke(Color("Borders400"))
                    .foregroundColor(Color("Borders400"))
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
                        data.pushButton(buttonID: .RecoverSeed, details: seedName)
                    }
                    .onAppear(perform: {nameFocused = true})
                    .padding(.horizontal, 8)
            }
            Text("Display name visible only to you").font(.callout)
            Text(data.lastError).foregroundColor(Color("SignalDanger"))
            Spacer()
            BigButton(
                text: "Next",
                action: {
                    data.pushButton(buttonID: .RecoverSeed, details: seedName)
                },
                isDisabled: seedName == ""
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

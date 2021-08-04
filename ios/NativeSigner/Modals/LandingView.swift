//
//  LandingView.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 23.7.2021.
//

import SwiftUI

struct LandingView: View {
    @EnvironmentObject var data: SignerDataModel
    var body: some View {
        ZStack {
            RoundedRectangle(cornerRadius: 50).foregroundColor(/*@START_MENU_TOKEN@*/Color("backgroundCard")/*@END_MENU_TOKEN@*/)
            VStack {
                Text("There should be TC and PP I don't copy and paste code I don't copy and paste code I don't copy and paste code I don't copy and paste code I don't copy and paste code I don't copy and paste code I don't copy and paste code I don't copy and paste code I don't copy and paste code I don't copy and paste code I don't copy and paste code I don't copy and paste code I don't copy and paste code I don't copy and paste code ")
                    .foregroundColor(Color("textMainColor"))
                Button(action: {
                    data.onboard()
                }) {
                    Text("Accept")
                }
            }
        }
    }
}

struct LandingView_Previews: PreviewProvider {
    static var previews: some View {
        LandingView()
    }
}

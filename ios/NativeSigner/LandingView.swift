//
//  LandingView.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 23.7.2021.
//

import SwiftUI

struct LandingView: View {
    @Binding var onBoard: OnBoardingStruct
    var body: some View {
        VStack {
            Text("There should be TC and PP")
            Button(action: {
                onBoard.onboard()
            }) {
                Text("Accept")
            }
        }
    }
}

struct LandingView_Previews: PreviewProvider {
    static var onBoard: OnBoardingStruct = OnBoardingStruct.init()
    static var previews: some View {
        LandingView(onBoard: .constant(onBoard))
    }
}

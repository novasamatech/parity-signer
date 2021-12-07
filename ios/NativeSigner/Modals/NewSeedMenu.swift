//
//  NewSeedMenu.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 7.12.2021.
//

import SwiftUI

struct NewSeedMenu: View {
    @EnvironmentObject var data: SignerDataModel
    var body: some View {
        VStack {
            Spacer()
            HeaderBar(line1: "ADD SEED", line2: "Select seed addition method" )
            Text("Buttonees")
        }
    }
}

/*
struct NewSeedMenu_Previews: PreviewProvider {
    static var previews: some View {
        NewSeedMenu()
    }
}
*/

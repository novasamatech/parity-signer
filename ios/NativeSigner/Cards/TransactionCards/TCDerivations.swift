//
//  TCDerivations.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 6.1.2022.
//

import SwiftUI

struct TCDerivations: View {
    let value: [String]
    var body: some View {
        VStack {
            Text("Importing derivations:")
            ForEach(value, id: \.self) {derivation in
                Text(derivation)
            }
        }
    }
}

/*
struct TCDerivations_Previews: PreviewProvider {
    static var previews: some View {
        TCDerivations()
    }
}
*/

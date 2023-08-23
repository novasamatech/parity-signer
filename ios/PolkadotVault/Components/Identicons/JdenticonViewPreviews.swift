//
//  JdenticonViewPreviews.swift
//  PolkadotVault
//
//  Created by Krzysztof Rodak on 23/08/2023.
//

import Jdenticon
import SwiftUI

struct JdenticonView_Previews: PreviewProvider {
    static var previews: some View {
        VStack {
            JdenticonView(
                hash: "8PegJD6VsjWwinrP6AfgNqejWYdJ8KqF4xutpyq7AdFJ3W5",
                size: 36
            )
            .padding()
            JdenticonView(hash: "8PegJD6VsjWwinrP6AfgNqejWYdJ8KqF4xutpyq7AdFJ3W5", size: 120)
                .padding()
        }
    }
}

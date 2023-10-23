//
//  OnboadingAirgapView+Components.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 15/02/2023.
//

import SwiftUI

extension AirgapComponent {
    var uncheckedForegroundColor: Color {
        .accentRed300
    }

    var checkedForegroundColor: Color {
        .accentGreen300
    }

    var title: String {
        switch self {
        case .aiplaneMode:
            Localizable.Onboarding.Airgap.Label.airplane.string
        case .wifi:
            Localizable.Onboarding.Airgap.Label.wifi.string
        }
    }

    var uncheckedIcon: Image {
        switch self {
        case .aiplaneMode:
            Image(.airgapAirplaneError)
        case .wifi:
            Image(.airgapWifiError)
        }
    }

    var checkedIcon: Image {
        switch self {
        case .aiplaneMode:
            Image(.airgapAirplane)
        case .wifi:
            Image(.airgapWifi)
        }
    }
}

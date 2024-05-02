//
//  OnboadingAirgapView+Components.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 15/02/2023.
//

import SwiftUI

enum AirgapComponent: Equatable, Hashable {
    case aiplaneMode
    case wifi
    case location
}

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
            Localizable.Airgap.Label.airplane.string
        case .wifi:
            Localizable.Airgap.Label.wifi.string
        case .location:
            Localizable.Airgap.Label.location.string
        }
    }

    var uncheckedIcon: Image {
        switch self {
        case .aiplaneMode:
            Image(.airgapAirplaneError)
        case .wifi:
            Image(.airgapWifiError)
        case .location:
            Image(.airgapLocationError)
        }
    }

    var checkedIcon: Image {
        switch self {
        case .aiplaneMode:
            Image(.airgapAirplane)
        case .wifi:
            Image(.airgapWifi)
        case .location:
            Image(.airgapLocation)
        }
    }
}

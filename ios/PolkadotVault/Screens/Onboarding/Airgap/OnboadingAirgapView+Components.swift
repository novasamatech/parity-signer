//
//  OnboadingAirgapView+Components.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 15/02/2023.
//

import SwiftUI

extension AirgapComponent {
    var uncheckedForegroundColor: Color {
        Asset.accentRed300.swiftUIColor
    }

    var checkedForegroundColor: Color {
        Asset.accentGreen300.swiftUIColor
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
            Asset.airgapAirplaneError.swiftUIImage
        case .wifi:
            Asset.airgapWifiError.swiftUIImage
        }
    }

    var checkedIcon: Image {
        switch self {
        case .aiplaneMode:
            Asset.airgapAirplane.swiftUIImage
        case .wifi:
            Asset.airgapWifi.swiftUIImage
        }
    }
}

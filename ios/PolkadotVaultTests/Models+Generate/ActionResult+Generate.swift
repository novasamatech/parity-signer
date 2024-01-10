//
//  ActionResult+Generate.swift
//  PolkadotVaultTests
//
//  Created by Krzysztof Rodak on 05/08/2022.
//

import Foundation
@testable import PolkadotVault

extension ActionResult {
    static func generate(
        screenLabel: String = "Label",
        back: Bool = false,
        footer: Bool = false,
        footerButton: FooterButton? = nil,
        rightButton: RightButton? = nil,
        screenNameType: ScreenNameType = .h1,
        screenData: ScreenData = .scan,
        modalData: ModalData? = nil,
        alertData: AlertData? = nil
    ) -> ActionResult {
        ActionResult(
            screenLabel: screenLabel,
            back: back,
            footer: footer,
            footerButton: footerButton,
            rightButton: rightButton,
            screenNameType: screenNameType,
            screenData: screenData,
            modalData: modalData,
            alertData: alertData
        )
    }
}

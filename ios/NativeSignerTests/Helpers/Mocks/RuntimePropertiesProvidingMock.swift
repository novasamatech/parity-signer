//
//  RuntimePropertiesProvidingMock.swift
//  NativeSignerTests
//
//  Created by Krzysztof Rodak on 29/08/2022.
//

import Foundation
@testable import NativeSigner

final class RuntimePropertiesProvidingMock: RuntimePropertiesProviding {
    var isInDevelopmentMode: Bool = false
}

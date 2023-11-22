//
//  KeychainAccessAdapterTests.swift
//  PolkadotVaultTests
//
//  Created by Krzysztof Rodak on 22/11/2023.
//

import Foundation
@testable import PolkadotVault
import XCTest

final class KeychainAccessAdapterTests: XCTestCase {
    private var keychainQueryProviderMock: KeychainQueryProviderMock!
    private var accessControlProviderMock: AccessControlProviderMock!
    private var keychainAccessAdapter: KeychainAccessAdapter!

    override func setUp() {
        super.setUp()
        keychainQueryProviderMock = KeychainQueryProviderMock()
        accessControlProviderMock = AccessControlProviderMock()
        keychainAccessAdapter = KeychainAccessAdapter(
            acccessControlProvider: accessControlProviderMock,
            queryProvider: keychainQueryProviderMock
        )
    }

    override func tearDown() {
        keychainQueryProviderMock = nil
        accessControlProviderMock = nil
        keychainAccessAdapter = nil
        super.tearDown()
    }
}

// MARK: - Mocks

final class KeychainQueryProviderMock: KeychainQueryProviding {
    var queryCallsCount = 0
    var queryReceivedQueryTypes: [KeychainQuery] = []
    var queryReturnValue: CFDictionary!

    func query(for queryType: KeychainQuery) -> CFDictionary {
        queryCallsCount += 1
        queryReceivedQueryTypes.append(queryType)
        return queryReturnValue
    }
}

final class AccessControlProviderMock: AccessControlProviding {
    var accessControlCallsCount = 0
    var accessControlToReturn: SecAccessControl?
    var accessControlToThrow: Error!

    func accessControl() throws -> SecAccessControl {
        accessControlCallsCount += 1
        if let accessControl = accessControlToReturn {
            return accessControl
        } else {
            throw accessControlToThrow
        }
    }
}

//
//  BackendServiceTests.swift
//  PolkadotVaultTests
//
//  Created by Krzysztof Rodak on 04/12/2023.
//

import Foundation
@testable import PolkadotVault
import XCTest

final class BackendServiceTests: XCTestCase {
    private var callQueueMock: DispatchingMock!
    private var callbackQueueMock: DispatchingMock!
    private var backendService: BackendService!

    override func setUp() {
        super.setUp()
        callQueueMock = DispatchingMock()
        callbackQueueMock = DispatchingMock()
        backendService = BackendService(
            callQueue: callQueueMock,
            callbackQueue: callbackQueueMock
        )
    }

    override func tearDown() {
        callQueueMock = nil
        callbackQueueMock = nil
        backendService = nil
        super.tearDown()
    }

    func testPerformCallServiceError_Success() {
        // Given
        let expectedResult = "SuccessResult"
        let call = { expectedResult }

        var completionCalled = false
        var completionResult: Result<String, ServiceError>?

        // When
        backendService.performCall(call) { result in
            completionCalled = true
            completionResult = result
        }

        // Then
        XCTAssertTrue(completionCalled)
        XCTAssertEqual(callQueueMock.asyncCallsCount, 1)
        XCTAssertEqual(callbackQueueMock.asyncCallsCount, 1)
        if case let .success(value) = completionResult {
            XCTAssertEqual(value, expectedResult)
        } else {
            XCTFail("Expected success, but got failure")
        }
    }

    func testPerformCallServiceError_InvalidTypeFailure() {
        // Given
        let call = { 123 } // Returning an unexpected type

        var completionResult: Result<String, ServiceError>?

        // When
        backendService.performCall(call) { result in
            completionResult = result
        }

        // Then
        if case let .failure(error) = completionResult {
            XCTAssertEqual(error.message, Localizable.ErrorDisplayed.invalidType.string)
        } else {
            XCTFail("Expected invalid type error, but got \(String(describing: completionResult))")
        }
    }

    func testPerformCallServiceError_GenericErrorHandling() {
        // Given
        struct GenericError: Error {
            var localizedDescription: String {
                "Generic error occurred"
            }
        }
        let genericError = GenericError()
        let call = { throw genericError }

        var completionResult: Result<String, ServiceError>?

        // When
        backendService.performCall(call) { result in
            completionResult = result
        }

        // Then
        if case let .failure(error) = completionResult {
            XCTAssertEqual(error.message, genericError.backendDisplayError)
        } else {
            XCTFail("Expected generic error handling, but got \(String(describing: completionResult))")
        }
    }

    func testPerformCallErrorDisplayed_Success() {
        // Given
        let expectedResult = "SuccessResult"
        let call = { expectedResult }

        var completionCalled = false
        var completionResult: Result<String, ErrorDisplayed>?

        // When
        backendService.performCall(call) { result in
            completionCalled = true
            completionResult = result
        }

        // Then
        XCTAssertTrue(completionCalled)
        if case let .success(value) = completionResult {
            XCTAssertEqual(value, expectedResult)
        } else {
            XCTFail("Expected success, but got failure")
        }
    }

    func testPerformCallErrorDisplayed_InvalidTypeFailure() {
        // Given
        let call = { 123 } // Returning an unexpected type

        var completionResult: Result<String, ErrorDisplayed>?

        // When
        backendService.performCall(call) { result in
            completionResult = result
        }

        // Then
        if case let .failure(error) = completionResult, case let .Str(message) = error {
            XCTAssertEqual(message, Localizable.ErrorDisplayed.invalidType.string)
        } else {
            XCTFail("Expected invalid type error, but got \(String(describing: completionResult))")
        }
    }

    func testPerformCallErrorDisplayed_ErrorDisplayedFailure() {
        // Given
        let displayedError = ErrorDisplayed.WrongPassword
        let call = { throw displayedError }

        var completionResult: Result<String, ErrorDisplayed>?

        // When
        backendService.performCall(call) { result in
            completionResult = result
        }

        // Then
        if case let .failure(error) = completionResult {
            XCTAssertEqual(error, displayedError)
        } else {
            XCTFail("Expected ErrorDisplayed.WrongPassword, but got \(String(describing: completionResult))")
        }
    }

    func testPerformCallErrorDisplayed_GenericErrorHandling() {
        // Given
        struct GenericError: Error, LocalizedError {
            let message: String
            var errorDescription: String? {
                message
            }
        }
        let genericError = GenericError(message: "Generic error occurred")
        let call = { throw genericError }

        var completionResult: Result<String, ErrorDisplayed>?

        // When
        backendService.performCall(call) { result in
            completionResult = result
        }

        // Then
        if case let .failure(error) = completionResult, case let .Str(message) = error {
            XCTAssertEqual(message, genericError.localizedDescription)
        } else {
            XCTFail("Expected generic error handling, but got \(String(describing: completionResult))")
        }
    }
}

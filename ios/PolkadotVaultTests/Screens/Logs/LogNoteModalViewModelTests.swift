//
//  LogNoteModalViewModelTests.swift
//  PolkadotVaultTests
//
//  Created by Krzysztof Rodak on 22/01/2024.
//

import Combine
import Foundation
@testable import PolkadotVault
import SwiftUI
import XCTest

final class LogNoteModalViewModelTests: XCTestCase {
    private var viewModel: LogNoteModal.ViewModel!
    private var logsServiceMock: LogsServicingMock!
    private var cancelBag: Set<AnyCancellable>!
    private var isPresented: Bool = false

    override func setUp() {
        super.setUp()
        logsServiceMock = LogsServicingMock()
        isPresented = false
        viewModel = LogNoteModal.ViewModel(
            isPresented: Binding(get: { self.isPresented }, set: { self.isPresented = $0 }),
            logsService: logsServiceMock
        )
        cancelBag = []
    }

    override func tearDown() {
        viewModel = nil
        logsServiceMock = nil
        cancelBag = nil
        super.tearDown()
    }

    func testInit_SetsIsActionDisabledToTrue() {
        // Then
        XCTAssertTrue(viewModel.isActionDisabled)
    }

    func testNoteChange_UpdatesIsActionDisabled() {
        // When
        viewModel.note = "new note"

        // Then
        XCTAssertFalse(viewModel.isActionDisabled)
    }

    func testOnCancelTap_SetsIsPresentedToFalse() {
        // When
        viewModel.onCancelTap()

        // Then
        XCTAssertFalse(isPresented)
    }

    func testOnDoneTap_WithSuccess() {
        // Given
        viewModel.note = "new note"

        // When
        viewModel.onDoneTap()
        logsServiceMock.addCommentToLogsReceivedCompletion.first?(.success(()))

        // Then
        XCTAssertFalse(isPresented)
    }

    func testOnDoneTap_WithFailure() {
        // Given
        viewModel.note = "new note"
        let error = ServiceError(message: "Error")
        let presentableError: ErrorBottomModalViewModel = .alertError(message: error.localizedDescription)

        // When
        viewModel.onDoneTap()
        logsServiceMock.addCommentToLogsReceivedCompletion.first?(.failure(error))

        // Then
        XCTAssertEqual(viewModel.presentableError, presentableError)
        XCTAssertTrue(viewModel.isPresentingError)
    }
}

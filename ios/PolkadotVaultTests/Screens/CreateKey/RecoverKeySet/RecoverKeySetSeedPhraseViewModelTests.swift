//
//  RecoverKeySetSeedPhraseViewModelTests.swift
//  PolkadotVaultTests
//
//  Created by Krzysztof Rodak on 30/01/2024.
//

import Foundation
@testable import PolkadotVault
import SwiftUI
import XCTest

final class RecoverKeySetSeedPhraseViewModelTests: XCTestCase {
    private var viewModel: RecoverKeySetSeedPhraseView.ViewModel!
    private var serviceMock: RecoverKeySetServicingMock!
    private var seedsMediatorMock: SeedsMediatingMock!
    private var isPresented: Bool = false
    private var onCompletionExecuted: Bool = false

    override func setUp() {
        super.setUp()
        serviceMock = RecoverKeySetServicingMock()
        seedsMediatorMock = SeedsMediatingMock()
        isPresented = false
        onCompletionExecuted = false
        viewModel = RecoverKeySetSeedPhraseView.ViewModel(
            seedName: "testSeed",
            isPresented: Binding(get: { self.isPresented }, set: { self.isPresented = $0 }),
            seedsMediator: seedsMediatorMock,
            service: serviceMock,
            onCompletion: { _ in self.onCompletionExecuted = true }
        )
    }

    override func tearDown() {
        viewModel = nil
        serviceMock = nil
        seedsMediatorMock = nil
        super.tearDown()
    }

    func testInitialSetup() {
        // Then
        XCTAssertFalse(viewModel.isPresentingDetails)
        XCTAssertFalse(viewModel.isValidSeedPhrase)
        XCTAssertEqual(viewModel.userInput, RecoverKeySetSeedPhraseView.ViewModel.Constants.invisibleNonEmptyCharacter)
    }

    func testOnAppear_requestGuesses() {
        // When
        viewModel.onAppear()

        // Then
        XCTAssertEqual(serviceMock.updateGuessWordsUserInputCallsCount, 1)
        XCTAssertEqual(serviceMock.updateGuessWordsUserInputReceivedUserInput, [""])
    }

    func testOnAppear_updatesGuesses_onSuccess() {
        // Given
        let guesses = ["abc", "bcd"]

        // When
        viewModel.onAppear()
        serviceMock.updateGuessWordsUserInputReceivedCompletion.first?(.success(guesses))

        // Then
        XCTAssertEqual(viewModel.guesses, guesses)
    }

    func testOnAppear_updatesGuesses_onFailure() {
        // Given
        let error = ServiceError(message: "Error")
        let expectedErrorMessage = ErrorBottomModalViewModel.alertError(message: error.localizedDescription)

        // When
        viewModel.onAppear()
        serviceMock.updateGuessWordsUserInputReceivedCompletion.first?(.failure(error))

        // Then
        XCTAssert(viewModel.isPresentingError)
        XCTAssertEqual(viewModel.presentableError, expectedErrorMessage)
    }

    func testOnGuessTap_AddsGuessToSeedPhraseDraft() {
        // Given
        let guess = "testGuess"

        // When
        viewModel.onGuessTap(guess)

        // Then
        XCTAssertTrue(viewModel.seedPhraseDraft.contains(guess))
    }

    func testOnUserInput_whenUserInputEmpty_deletesLastWordFromDraftAndUpdatesUserInput_updatesGuesses() {
        // Given
        let userInput = ""
        let currentPhraseDraft = ["abc", "bcd"]
        let expectedPhraseDraft = ["abc"]
        let expectedUserInput = RecoverKeySetSeedPhraseView.ViewModel.Constants.invisibleNonEmptyCharacter
        viewModel.seedPhraseDraft = currentPhraseDraft

        // When
        viewModel.onUserInput(userInput)

        // Then
        XCTAssertEqual(viewModel.seedPhraseDraft, expectedPhraseDraft)
        XCTAssertEqual(viewModel.userInput, expectedUserInput)
        XCTAssertEqual(serviceMock.updateGuessWordsUserInputCallsCount, 2)
        XCTAssertEqual(serviceMock.updateGuessWordsUserInputReceivedUserInput, ["", ""])
    }

    func testOnUserInput_whenUserInputEndsWithWhitespace_whenGuessExist_guessIsAdded_guessesAreUpdated() {
        // Given
        let userInput = "\(RecoverKeySetSeedPhraseView.ViewModel.Constants.invisibleNonEmptyCharacter)abc "
        let expectedUserInput = RecoverKeySetSeedPhraseView.ViewModel.Constants.invisibleNonEmptyCharacter
        let expectedPhraseDraft = ["abc"]
        viewModel.guesses = ["abc"]
        viewModel.seedPhraseDraft = []

        // When
        viewModel.onUserInput(userInput)

        // Then
        XCTAssertEqual(viewModel.seedPhraseDraft, expectedPhraseDraft)
        XCTAssertEqual(viewModel.userInput, expectedUserInput)
        XCTAssertEqual(serviceMock.updateGuessWordsUserInputCallsCount, 2)
        XCTAssertEqual(serviceMock.updateGuessWordsUserInputReceivedUserInput, ["", ""])
    }

    func testOnUserInput_whenUserInputEndsWithWhitespace_whenGuessDoesNotExist_whitespaceIsDeleted_guessesAreNotUpdated(
    ) {
        // Given
        let userInput = "\(RecoverKeySetSeedPhraseView.ViewModel.Constants.invisibleNonEmptyCharacter)abc "
        let expectedUserInput = "abc"
        viewModel.guesses = ["bcd"]
        viewModel.seedPhraseDraft = []

        // When
        viewModel.onUserInput(userInput)

        // Then
        XCTAssertEqual(viewModel.seedPhraseDraft, [])
        XCTAssertEqual(viewModel.userInput, expectedUserInput)
        XCTAssertEqual(serviceMock.updateGuessWordsUserInputCallsCount, 1)
    }

    func testOnUserInput_whenUserInputIsAnotherCharacter_guessesAreUpdated() {
        // Given
        let userInput = "\(RecoverKeySetSeedPhraseView.ViewModel.Constants.invisibleNonEmptyCharacter)a"
        viewModel.seedPhraseDraft = []

        // When
        viewModel.onUserInput(userInput)

        // Then
        XCTAssertEqual(viewModel.seedPhraseDraft, [])
        XCTAssertEqual(serviceMock.updateGuessWordsUserInputCallsCount, 2)
        XCTAssertEqual(serviceMock.updateGuessWordsUserInputReceivedUserInput, ["", "a"])
    }

    func testOnUserInput_whenSeedPhraseDraftIsUpdated_seedPhraseIsValidated() {
        // Given
        let userInput = "\(RecoverKeySetSeedPhraseView.ViewModel.Constants.invisibleNonEmptyCharacter)abc "
        let expectedSeedPhrase = "bcd abc"
        viewModel.guesses = ["abc"]
        viewModel.seedPhraseDraft = ["bcd"]

        // When
        viewModel.onUserInput(userInput)

        // Then
        XCTAssertEqual(viewModel.seedPhrase, expectedSeedPhrase)
        XCTAssertEqual(serviceMock.validateSeedPhraseCallsCount, 2)
        XCTAssertEqual(serviceMock.validateSeedPhraseReceivedSeedPhrase, ["bcd", expectedSeedPhrase])
    }

    func testOnUserInput_whenValidation_whenSuccess_isValidIsUpdated() {
        // Given
        let userInput = "\(RecoverKeySetSeedPhraseView.ViewModel.Constants.invisibleNonEmptyCharacter)abc "
        let expectedSeedPhrase = "bcd abc"
        viewModel.isValidSeedPhrase = false
        viewModel.guesses = ["abc"]
        viewModel.seedPhraseDraft = ["bcd"]

        // When
        viewModel.onUserInput(userInput)
        serviceMock.validateSeedPhraseReceivedCompletion.first?(.success(true))

        // Then
        XCTAssert(viewModel.isValidSeedPhrase)
    }

    func testOnUserInput_whenValidation_whenFailure_errorIsPresented_isValidIsSetToFalse() {
        // Given
        let error = ServiceError(message: "Error")
        let expectedErrorMessage = ErrorBottomModalViewModel.alertError(message: error.localizedDescription)
        let userInput = "\(RecoverKeySetSeedPhraseView.ViewModel.Constants.invisibleNonEmptyCharacter)abc "
        viewModel.isValidSeedPhrase = true
        viewModel.guesses = ["abc"]
        viewModel.seedPhraseDraft = ["bcd"]

        // When
        viewModel.onUserInput(userInput)
        serviceMock.validateSeedPhraseReceivedCompletion.first?(.failure(error))

        // Then
        XCTAssert(viewModel.isPresentingError)
        XCTAssertEqual(viewModel.presentableError, expectedErrorMessage)
    }

    func testOnDoneTap_SetsIsPresentingDetails() {
        // When
        viewModel.onDoneTap()

        // Then
        XCTAssertTrue(viewModel.isPresentingDetails)
    }

    func testCreateDerivedKeys_ReturnsExpectedViewModel() {
        // When
        let derivedViewModel = viewModel.createDerivedKeys()

        // Then
        XCTAssertEqual(derivedViewModel.seedName, viewModel.seedName)
        XCTAssertEqual(derivedViewModel.seedPhrase, viewModel.seedPhrase)
    }
}

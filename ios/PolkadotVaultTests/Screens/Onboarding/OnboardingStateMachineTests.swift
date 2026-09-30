import Combine
import Foundation
@testable import PolkadotVault
import XCTest

final class OnboardingStateMachineTests: XCTestCase {
    func testScreenshotsNextWaitsForBackendPreparation() {
        let mediator = OnboardingPreparationSpy()
        let stateMachine = OnboardingStateMachine(onboardingMediator: mediator)
        stateMachine.currentState = .screenshots

        stateMachine.onScreenshotNextTap()

        XCTAssertEqual(stateMachine.currentState, .screenshots)
        XCTAssertEqual(mediator.preparationCompletions.count, 1)

        mediator.preparationCompletions.first?(true)

        XCTAssertEqual(stateMachine.currentState, .setUpNetworksIntro)
    }

    func testRepeatedScreenshotsNextDoesNotStartAnotherPreparation() {
        let mediator = OnboardingPreparationSpy()
        let stateMachine = OnboardingStateMachine(onboardingMediator: mediator)

        stateMachine.onScreenshotNextTap()
        stateMachine.onScreenshotNextTap()

        XCTAssertEqual(mediator.preparationCompletions.count, 1)
    }

    func testPreparationFailureAllowsRetry() {
        let mediator = OnboardingPreparationSpy()
        let stateMachine = OnboardingStateMachine(onboardingMediator: mediator)
        stateMachine.currentState = .screenshots

        stateMachine.onScreenshotNextTap()
        mediator.preparationCompletions.first?(false)

        XCTAssertEqual(stateMachine.currentState, .screenshots)

        stateMachine.onScreenshotNextTap()
        mediator.preparationCompletions.last?(true)

        XCTAssertEqual(mediator.preparationCompletions.count, 2)
        XCTAssertEqual(stateMachine.currentState, .setUpNetworksIntro)
    }

    func testTutorialCompletionDoesNotRequestReset() {
        let mediator = OnboardingPreparationSpy()
        let stateMachine = OnboardingStateMachine(onboardingMediator: mediator)

        stateMachine.finishOnboarding()

        XCTAssertEqual(mediator.finishCallsCount, 1)
        XCTAssertEqual(mediator.resetCallsCount, 0)
    }
}

private final class OnboardingPreparationSpy: OnboardingMediating {
    var onboardingDone: AnyPublisher<Bool, Never> { Just(false).eraseToAnyPublisher() }
    var isUserOnboarded = false
    var preparationCompletions: [(Bool) -> Void] = []
    var finishCallsCount = 0
    var resetCallsCount = 0

    func prepareForScanning(_ completion: @escaping (Bool) -> Void) {
        preparationCompletions.append(completion)
    }

    func finishOnboarding() {
        finishCallsCount += 1
    }

    func onboard(verifierRemoved _: Bool) {
        resetCallsCount += 1
    }
}

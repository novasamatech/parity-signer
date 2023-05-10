//
//  OnboardingState.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 30/01/2023.
//

import Combine
import SwiftUI

enum OnboardingState: Equatable {
    case overview
    case terms
    case airgap
    case screenshots
    case setUpNetworksIntro
    case setUpNetworksStepOne
    case setUpNetworksStepTwo
}

final class OnboardingStateMachine: ObservableObject {
    @Published var currentState: OnboardingState = .overview
    private let onboardingMediator: OnboardingMediator

    init(
        onboardingMediator: OnboardingMediator = ServiceLocator.onboardingMediator
    ) {
        self.onboardingMediator = onboardingMediator
    }

    @ViewBuilder
    func currentView() -> some View {
        switch currentState {
        case .overview:
            OnboardingOverviewView(viewModel: .init(onNextTap: { self.onOverviewFinishTap() }))
        case .terms:
            OnboardingAgreementsView(viewModel: .init(onNextTap: { self.onAgreementNextTap() }))
        case .airgap:
            OnboardingAirgapView(viewModel: .init(onNextTap: { self.onAirgapNextTap() }))
        case .screenshots:
            OnboardingScreenshotsView(viewModel: .init(onNextTap: { self.onScreenshotNextTap() }))
        case .setUpNetworksIntro:
            SetUpNetworksIntroView(
                viewModel: .init(
                    onNextTap: { self.onSetUpNetworksIntroNext() },
                    onSkipTap: { self.finishOnboarding() }
                )
            )
        case .setUpNetworksStepOne:
            SetUpNetworksStepOneView(
                viewModel: .init(
                    onNextTap: { self.onSetUpNetworksStepOne() },
                    onBackTap: { self.onSetUpNetworksStepOneBackTap() }
                )
            )
        case .setUpNetworksStepTwo:
            SetUpNetworksStepTwoView(
                viewModel: .init(
                    onNextTap: { self.finishOnboarding() },
                    onBackTap: { self.onSetUpNetworksStepTwoBackTap() }
                )
            )
        }
    }

    func onOverviewFinishTap() {
        currentState = .terms
    }

    func onAgreementNextTap() {
        currentState = .airgap
    }

    func onAirgapNextTap() {
        currentState = .screenshots
    }

    func onScreenshotNextTap() {
        currentState = .setUpNetworksIntro
    }

    func onSetUpNetworksIntroNext() {
        currentState = .setUpNetworksStepOne
    }

    func onSetUpNetworksStepOne() {
        currentState = .setUpNetworksStepTwo
    }

    func onSetUpNetworksStepOneBackTap() {
        currentState = .setUpNetworksIntro
    }

    func onSetUpNetworksStepTwoBackTap() {
        currentState = .setUpNetworksStepOne
    }

    func finishOnboarding() {
        currentState = .overview
        onboardingMediator.onboard()
    }
}

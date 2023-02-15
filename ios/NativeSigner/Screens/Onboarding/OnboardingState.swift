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
}

final class OnboardingStateMachine: ObservableObject {
    @Published var currentState: OnboardingState = .overview

    @ViewBuilder
    func currentView() -> some View {
        switch currentState {
        case .overview:
            OnboardingOverviewView(viewModel: .init(stateMachine: self))
        case .terms:
            OnboardingAgreementsView(viewModel: .init(stateMachine: self))
        case .airgap:
            EmptyView()
        case .screenshots:
            EmptyView()
        }
    }

    func onOverviewFinishTap() {
        currentState = .terms
    }

    func onAgreementNextTap() {
        // This is just temporary, on last step we need to revert to first one
        currentState = .overview
    }
}

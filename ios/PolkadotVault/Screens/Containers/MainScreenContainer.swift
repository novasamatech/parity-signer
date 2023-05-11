//
//  ContentView.swift
//  Polkadot Vault
//
//  Created by Alexander Slesarev on 19.7.2021.
//

import Combine
import SwiftUI

struct MainScreenContainer: View {
    @StateObject var viewModel: ViewModel
    @StateObject var onboarding: OnboardingStateMachine

    var body: some View {
        switch viewModel.viewState {
        case .authenticated:
            AuthenticatedScreenContainer(viewModel: .init())
        case .deviceLocked:
            UnlockDeviceView(viewModel: .init(
                passwordProtectionStatePublisher: viewModel
                    .passwordProtectionStatePublisher
            ))
        case .onboarding:
            onboarding.currentView()
        case .noPincode:
            DevicePincodeRequired(viewModel: .init())
        }
    }
}

extension MainScreenContainer {
    enum ViewState: Equatable, Hashable {
        case authenticated
        case deviceLocked
        case onboarding
        case noPincode
    }

    final class ViewModel: ObservableObject {
        private let authenticationStateMediator: AuthenticatedStateMediator
        private let onboardingMediator: OnboardingMediator
        let passwordProtectionStatePublisher: PasswordProtectionStatePublisher
        private let cancelBag = CancelBag()
        @Published var viewState: ViewState = .deviceLocked

        init(
            authenticationStateMediator: AuthenticatedStateMediator = ServiceLocator.authenticationStateMediator,
            onboardingMediator: OnboardingMediator = ServiceLocator.onboardingMediator,
            passwordProtectionStatePublisher: PasswordProtectionStatePublisher = PasswordProtectionStatePublisher()
        ) {
            self.authenticationStateMediator = authenticationStateMediator
            self.onboardingMediator = onboardingMediator
            self.passwordProtectionStatePublisher = passwordProtectionStatePublisher
            listenToStateChanges()
        }

        private func listenToStateChanges() {
            Publishers.CombineLatest3(
                onboardingMediator.$onboardingDone,
                authenticationStateMediator.$authenticated,
                passwordProtectionStatePublisher.$isProtected
            )
            .map {
                let (onboardingDone, authenticated, isProtected) = $0
                if !isProtected {
                    return .noPincode
                }
                if !onboardingDone {
                    return .onboarding
                }
                return authenticated ? .authenticated : .deviceLocked
            }
            .assign(to: \.viewState, on: self)
            .store(in: cancelBag)
        }
    }
}

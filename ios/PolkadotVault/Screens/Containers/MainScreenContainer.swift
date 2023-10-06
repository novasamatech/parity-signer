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
            UnlockDeviceView(viewModel: .init())
        case .onboarding:
            onboarding.currentView()
        case .updateRequired:
            ApplicationUpdateRequiredView(viewModel: .init())
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
        case updateRequired
        case noPincode
    }

    final class ViewModel: ObservableObject {
        private let authenticationStateMediator: AuthenticatedStateMediator
        private let onboardingMediator: OnboardingMediator
        private let passwordProtectionStatePublisher: PasswordProtectionStatePublisher
        private let databaseVersionMediator: DatabaseVersionMediator
        private let cancelBag = CancelBag()
        @Published var viewState: ViewState = .deviceLocked

        init(
            authenticationStateMediator: AuthenticatedStateMediator = ServiceLocator.authenticationStateMediator,
            onboardingMediator: OnboardingMediator = ServiceLocator.onboardingMediator,
            passwordProtectionStatePublisher: PasswordProtectionStatePublisher = PasswordProtectionStatePublisher(),
            databaseVersionMediator: DatabaseVersionMediator = DatabaseVersionMediator()
        ) {
            self.authenticationStateMediator = authenticationStateMediator
            self.onboardingMediator = onboardingMediator
            self.passwordProtectionStatePublisher = passwordProtectionStatePublisher
            self.databaseVersionMediator = databaseVersionMediator
            checkInitialState()
        }

        private func checkInitialState() {
            databaseVersionMediator.checkDatabaseScheme { result in
                switch result {
                case .success:
                    self.listenToStateChanges()
                case let .failure(error):
                    switch error {
                    case .invalidVersion:
                        self.viewState = .updateRequired
                    case .error:
                        /// If DB version check was unavailable, assume user needs to update
                        /// If that's not the case (i.e. there is no newer version), app restart will fix it so should be ok
                        self.viewState = .updateRequired
                    }
                }
            }
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

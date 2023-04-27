//
//  SettingsView.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 12/12/2022.
//

import SwiftUI

struct SettingsView: View {
    @StateObject var viewModel: ViewModel
    @EnvironmentObject private var navigation: NavigationCoordinator
    @EnvironmentObject private var appState: AppState

    var body: some View {
        NavigationView {
            ZStack(alignment: .bottom) {
                VStack(spacing: 0) {
                    NavigationBarView(
                        viewModel: NavigationBarViewModel(
                            title: Localizable.Settings.Label.title.string,
                            leftButtons: [.init(type: .empty)],
                            rightButtons: [.init(type: .empty)],
                            backgroundColor: Asset.backgroundPrimary.swiftUIColor
                        )
                    )
                    ScrollView {
                        VStack(alignment: .leading, spacing: 0) {
                            ForEach(viewModel.renderable.items, id: \.id) { renderable in
                                NavigationLink(
                                    destination: detailView(viewModel.detailScreen)
                                        .navigationBarHidden(true),
                                    isActive: $viewModel.isDetailsPresented
                                ) { EmptyView() }
                                SettingsRowView(renderable: renderable)
                                    .contentShape(Rectangle())
                                    .onTapGesture {
                                        viewModel.onTapAction(renderable.item)
                                    }
                            }
                            Text(Localizable.Settings.Label.version(ApplicationInformation.cfBundleShortVersionString))
                                .font(PrimaryFont.captionM.font)
                                .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
                                .padding(.top, Spacing.medium)
                                .padding(.horizontal, Spacing.large)
                                .padding(.bottom, Spacing.extraSmall)
                        }
                    }
                    Spacer()
                    TabBarView(
                        selectedTab: $navigation.selectedTab
                    )
                }
                .background(Asset.backgroundPrimary.swiftUIColor)
                .navigationViewStyle(StackNavigationViewStyle())
                .navigationBarHidden(true)
                ConnectivityAlertOverlay(viewModel: .init())
            }
            .background(Asset.backgroundPrimary.swiftUIColor)
        }
        .onAppear {
            viewModel.use(navigation: navigation)
            viewModel.use(appState: appState)
            viewModel.loadData()
        }
        .fullScreenCover(isPresented: $viewModel.isPresentingWipeConfirmation) {
            HorizontalActionsBottomModal(
                viewModel: .wipeAll,
                mainAction: viewModel.wipe(),
                isShowingBottomAlert: $viewModel.isPresentingWipeConfirmation
            )
            .clearModalBackground()
        }
        .environment(\.rootPresentationMode, $viewModel.isDetailsPresented)
    }

    @ViewBuilder
    func detailView(_ item: SettingsItem?) -> some View {
        switch item {
        case .logs:
            LogsListView(viewModel: .init())
        case .networks:
            NetworkSelectionSettings(viewModel: .init())
        case .verifier:
            VerifierCertificateView(viewModel: .init())
        case .backup:
            BackupSelectKeyView(viewModel: .init())
        case .privacyPolicy:
            PrivacyPolicyView(viewModel: .init())
        case .termsAndConditions:
            TermsOfServiceView(viewModel: .init())
        case .wipe:
            EmptyView()
        case .none:
            EmptyView()
        }
    }
}

extension SettingsView {
    final class ViewModel: ObservableObject {
        private let cancelBag = CancelBag()
        @Published var renderable: SettingsViewRenderable = .init()
        @Published var isPresentingWipeConfirmation = false
        @Published var isDetailsPresented = false
        @Published var detailScreen: SettingsItem?
        private weak var appState: AppState!
        private weak var navigation: NavigationCoordinator!
        private let onboardingMediator: OnboardingMediator

        init(
            onboardingMediator: OnboardingMediator = ServiceLocator.onboardingMediator
        ) {
            self.onboardingMediator = onboardingMediator
        }

        func use(navigation: NavigationCoordinator) {
            self.navigation = navigation
            listenToUpdates()
        }

        func use(appState: AppState) {
            self.appState = appState
        }

        func loadData() {
            renderable = SettingsViewRenderable()
        }

        private func listenToUpdates() {
            $isDetailsPresented.sink { [weak self] isPresented in
                guard let self = self, !isPresented else { return }
                self.navigation.performFake(navigation: .init(action: .start))
                self.navigation.performFake(navigation: .init(action: .navbarSettings))
            }.store(in: cancelBag)
        }

        func onTapAction(_ item: SettingsItem) {
            switch item {
            case .logs:
                detailScreen = .logs
                isDetailsPresented = true
            case .wipe:
                onTapWipe()
            case .termsAndConditions:
                detailScreen = .termsAndConditions
                isDetailsPresented = true
            case .privacyPolicy:
                detailScreen = .privacyPolicy
                isDetailsPresented = true
            case .backup:
                detailScreen = .backup
                isDetailsPresented = true
            case .networks:
                guard case let .manageNetworks(value) = navigation
                    .performFake(navigation: .init(action: .manageNetworks)).screenData else { return }
                appState.userData.manageNetworks = value
                detailScreen = .networks
                isDetailsPresented = true
            case .verifier:
                guard case let .vVerifier(value) = navigation
                    .performFake(navigation: .init(action: .viewGeneralVerifier)).screenData else { return }
                navigation.performFake(navigation: .init(action: .goBack))
                appState.userData.verifierDetails = value
                detailScreen = .verifier
                isDetailsPresented = true
            }
        }

        private func onTapWipe() {
            isPresentingWipeConfirmation = true
        }

        func wipe() {
            onboardingMediator.onboard()
            navigation.perform(navigation: .init(action: .start))
        }
    }
}

struct SettingsViewRenderable: Equatable {
    let items: [SettingsRowRenderable]

    init(items: [SettingsItem] = SettingsItem.allCases) {
        self.items = items
            .map { .init(item: $0, title: $0.title, isDestructive: $0.isDestructive, hasDetails: $0.hasDetails) }
    }
}

#if DEBUG
    struct SettingsView_Previews: PreviewProvider {
        static var previews: some View {
            SettingsView(viewModel: .init())
                .environmentObject(NavigationCoordinator())
        }
    }
#endif

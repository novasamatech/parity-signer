//
//  LogsListView.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 02/12/2022.
//

import SwiftUI

struct LogsListView: View {
    @StateObject var viewModel: ViewModel
    @EnvironmentObject private var navigation: NavigationCoordinator
    @Environment(\.presentationMode) var mode: Binding<PresentationMode>

    var body: some View {
        ZStack(alignment: .bottom) {
            NavigationLink(
                destination:
                LogDetailsView(viewModel: .init(viewModel.selectedDetails))
                    .navigationBarHidden(true),
                isActive: $viewModel.isPresentingDetails
            ) { EmptyView() }
            VStack(spacing: 0) {
                NavigationBarView(
                    viewModel: NavigationBarViewModel(
                        title: Localizable.LogsList.Label.title.string,
                        leftButtons: [.init(type: .arrow, action: { mode.wrappedValue.dismiss() })],
                        rightButtons: [.init(type: .more, action: viewModel.onMoreMenuTap)],
                        backgroundColor: Asset.backgroundPrimary.swiftUIColor
                    )
                )
                ScrollView {
                    LazyVStack(alignment: .leading, spacing: 0) {
                        ForEach(viewModel.renderables, id: \.id) { renderable in
                            LogEntryView(viewModel: .init(renderable: renderable))
                                .onTapGesture {
                                    viewModel.onEventTap(renderable)
                                }
                        }
                    }
                }
            }
            .background(Asset.backgroundPrimary.swiftUIColor)
            ConnectivityAlertOverlay(viewModel: .init())
        }
        .onAppear {
            viewModel.use(navigation: navigation)
            viewModel.loadData()
        }
        .fullScreenCover(
            isPresented: $viewModel.isShowingActionSheet,
            onDismiss: {
                // iOS 15 handling of following .fullscreen presentation after dismissal, we need to dispatch this async
                DispatchQueue.main.async { viewModel.onMoreActionSheetDismissal() }
            }
        ) {
            LogsMoreActionsModal(
                isShowingActionSheet: $viewModel.isShowingActionSheet,
                shouldPresentClearConfirmationModal: $viewModel.shouldPresentClearConfirmationModal,
                shouldPresentAddNoteModal: $viewModel.shouldPresentAddNoteModal
            )
            .clearModalBackground()
        }
        .fullScreenCover(isPresented: $viewModel.isPresentingClearConfirmationModal) {
            HorizontalActionsBottomModal(
                viewModel: .clearLog,
                mainAction: viewModel.clearLogsAction(),
                isShowingBottomAlert: $viewModel.isPresentingClearConfirmationModal
            )
            .clearModalBackground()
        }
        .fullScreenCover(
            isPresented: $viewModel.isPresentingAddNoteModal,
            onDismiss: { viewModel.loadData() }
        ) {
            LogNoteModal(viewModel: .init(isPresented: $viewModel.isPresentingAddNoteModal))
                .clearModalBackground()
        }
    }
}

extension LogsListView {
    final class ViewModel: ObservableObject {
        private var logs: MLog = .init(log: [])
        @Published var renderables: [LogEntryRenderable] = []
        @Published var shouldPresentClearConfirmationModal = false
        @Published var shouldPresentAddNoteModal = false
        @Published var isShowingActionSheet = false
        @Published var isPresentingClearConfirmationModal = false
        @Published var isPresentingAddNoteModal = false
        @Published var selectedDetails: MLogDetails!
        @Published var isPresentingDetails = false
        private weak var navigation: NavigationCoordinator!
        private let logsService: LogsService
        private let snackBarPresentation: BottomSnackbarPresentation
        private let renderableBuilder: LogEntryRenderableBuilder

        init(
            logsService: LogsService = LogsService(),
            snackBarPresentation: BottomSnackbarPresentation = ServiceLocator.bottomSnackbarPresentation,
            renderableBuilder: LogEntryRenderableBuilder = LogEntryRenderableBuilder()
        ) {
            self.logsService = logsService
            self.snackBarPresentation = snackBarPresentation
            self.renderableBuilder = renderableBuilder
        }

        func use(navigation: NavigationCoordinator) {
            self.navigation = navigation
        }

        func loadData() {
            logsService.getLogs { [weak self] result in
                guard let self else { return }
                switch result {
                case let .success(logs):
                    self.logs = logs
                    self.renderables = self.renderableBuilder.build(logs)
                case let .failure(error):
                    self.snackBarPresentation.viewModel = .init(title: error.description)
                    self.snackBarPresentation.isSnackbarPresented = true
                }
            }
        }

        func onMoreMenuTap() {
            isShowingActionSheet = true
        }

        func onMoreActionSheetDismissal() {
            if shouldPresentAddNoteModal {
                shouldPresentAddNoteModal = false
                isPresentingAddNoteModal = true
            }
            if shouldPresentClearConfirmationModal {
                shouldPresentClearConfirmationModal = false
                isPresentingClearConfirmationModal = true
            }
        }

        func onEventTap(_ renderable: LogEntryRenderable) {
            guard renderable.type != .basic else { return }
            logsService.getLogDetails(renderable.navigationDetails) { [weak self] result in
                guard let self else { return }
                switch result {
                case let .success(logDetails):
                    self.selectedDetails = logDetails
                    self.isPresentingDetails = true
                case let .failure(error):
                    self.selectedDetails = nil
                    self.snackBarPresentation.viewModel = .init(title: error.description)
                    self.snackBarPresentation.isSnackbarPresented = true
                }
            }
        }

        func onEventDetailsDismiss() {
            selectedDetails = nil
        }

        func clearLogsAction() {
            logsService.cleaLogHistory { [weak self] result in
                guard let self else { return }
                switch result {
                case .success:
                    self.loadData()
                case let .failure(error):
                    self.snackBarPresentation.viewModel = .init(title: error.description)
                    self.snackBarPresentation.isSnackbarPresented = true
                }
            }
        }
    }
}

#if DEBUG
    struct LogsListView_Previews: PreviewProvider {
        static var previews: some View {
            LogsListView(viewModel: .init())
                .environmentObject(NavigationCoordinator())
        }
    }
#endif

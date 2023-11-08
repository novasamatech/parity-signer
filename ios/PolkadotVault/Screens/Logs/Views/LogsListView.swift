//
//  LogsListView.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 02/12/2022.
//

import SwiftUI

struct LogsListView: View {
    @StateObject var viewModel: ViewModel
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
                        title: .title(Localizable.LogsList.Label.title.string),
                        leftButtons: [.init(type: .arrow, action: { mode.wrappedValue.dismiss() })],
                        rightButtons: [.init(type: .more, action: viewModel.onMoreMenuTap)],
                        backgroundColor: .backgroundPrimary
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
            .background(.backgroundPrimary)
        }
        .onAppear {
            viewModel.loadData()
        }
        .fullScreenModal(
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
        .fullScreenModal(isPresented: $viewModel.isPresentingClearConfirmationModal) {
            HorizontalActionsBottomModal(
                viewModel: .clearLog,
                mainAction: viewModel.clearLogsAction(),
                isShowingBottomAlert: $viewModel.isPresentingClearConfirmationModal
            )
            .clearModalBackground()
        }
        .fullScreenModal(
            isPresented: $viewModel.isPresentingAddNoteModal,
            onDismiss: { viewModel.loadData() }
        ) {
            LogNoteModal(viewModel: .init(isPresented: $viewModel.isPresentingAddNoteModal))
                .clearModalBackground()
        }
        .fullScreenModal(
            isPresented: $viewModel.isPresentingError
        ) {
            ErrorBottomModal(
                viewModel: viewModel.presentableError,
                isShowingBottomAlert: $viewModel.isPresentingError
            )
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
        @Published var isPresentingError: Bool = false
        @Published var presentableError: ErrorBottomModalViewModel = .noNetworksAvailable()
        private let logsService: LogsService
        private let renderableBuilder: LogEntryRenderableBuilder

        init(
            logsService: LogsService = LogsService(),
            renderableBuilder: LogEntryRenderableBuilder = LogEntryRenderableBuilder()
        ) {
            self.logsService = logsService
            self.renderableBuilder = renderableBuilder
        }

        func loadData() {
            logsService.getLogs { [weak self] result in
                guard let self else { return }
                switch result {
                case let .success(logs):
                    self.logs = logs
                    renderables = renderableBuilder.build(logs)
                case let .failure(error):
                    presentableError = .init(title: error.description)
                    isPresentingError = true
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
                    selectedDetails = logDetails
                    isPresentingDetails = true
                case let .failure(error):
                    selectedDetails = nil
                    presentableError = .init(title: error.description)
                    isPresentingError = true
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
                    loadData()
                case let .failure(error):
                    presentableError = .init(title: error.description)
                    isPresentingError = true
                }
            }
        }
    }
}

#if DEBUG
    struct LogsListView_Previews: PreviewProvider {
        static var previews: some View {
            LogsListView(viewModel: .init())
        }
    }
#endif

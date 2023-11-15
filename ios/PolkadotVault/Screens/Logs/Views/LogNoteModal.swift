//
//  LogNoteModal.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 18/01/2023.
//

import SwiftUI

struct LogNoteModal: View {
    @StateObject var viewModel: ViewModel
    @FocusState var focused: Bool
    @State var animateBackground: Bool = false

    var body: some View {
        FullScreenRoundedModal(
            backgroundTapAction: {
                viewModel.onCancelTap()
            },
            animateBackground: $animateBackground,
            ignoredEdges: .top
        ) {
            VStack(spacing: Spacing.medium) {
                HStack {
                    Button(
                        action: viewModel.onCancelTap
                    ) {
                        Image(.xmarkButtonMedium)
                            .foregroundColor(.textAndIconsSecondary)
                            .frame(
                                width: Heights.navigationButton,
                                height: Heights.navigationButton,
                                alignment: .center
                            )
                    }
                    .padding(.leading, Spacing.extraExtraSmall)
                    Spacer()
                    CapsuleButton(
                        action: viewModel.onDoneTap,
                        title: Localizable.LogsList.Modal.AddNote.Action.done.string,
                        isDisabled: $viewModel.isActionDisabled
                    )
                }
                .padding(.top, -Spacing.extraSmall)
                .padding(.horizontal, Spacing.extraSmall)
                VStack(alignment: .leading, spacing: 0) {
                    Localizable.LogsList.Modal.AddNote.Label.title.text
                        .font(PrimaryFont.titleM.font)
                        .foregroundColor(.textAndIconsPrimary)
                        .padding(.bottom, Spacing.medium)
                    Localizable.LogsList.Modal.AddNote.Label.header.text
                        .font(PrimaryFont.bodyM.font)
                        .foregroundColor(.textAndIconsSecondary)
                        .padding(.bottom, Spacing.small)
                    TextEditor(text: $viewModel.note)
                        .frame(minHeight: Heights.minTextEditorHeight, maxHeight: Heights.maxTextEditorHeight)
                        .hiddenTextEditorBackground()
                        .foregroundColor(.textAndIconsPrimary)
                        .font(PrimaryFont.bodyL.font)
                        .autocorrectionDisabled()
                        .autocapitalization(.none)
                        .keyboardType(.asciiCapable)
                        .submitLabel(.return)
                        .padding(.horizontal, Spacing.small)
                        .background(.fill6)
                        .cornerRadius(CornerRadius.small)
                        .submitLabel(.done)
                        .focused($focused)
                }
                .padding(.horizontal, Spacing.large)
                .padding(.bottom, Spacing.small)
            }
            .background(.backgroundTertiary)
        }
        .onAppear {
            focused = true
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

extension LogNoteModal {
    final class ViewModel: ObservableObject {
        @Binding var isPresented: Bool
        @Published var note: String = ""
        @Published var isActionDisabled: Bool = true
        private var cancelBag = CancelBag()
        private let logsService: LogsService
        @Published var isPresentingError: Bool = false
        @Published var presentableError: ErrorBottomModalViewModel = .noNetworksAvailable()

        init(
            isPresented: Binding<Bool>,
            logsService: LogsService = LogsService()
        ) {
            _isPresented = isPresented
            self.logsService = logsService
            subscribeToUpdates()
        }

        func onCancelTap() {
            isPresented = false
        }

        func onDoneTap() {
            logsService.addCommentToLogs(note) { [weak self] result in
                guard let self else { return }
                switch result {
                case .success:
                    isPresented = false
                case let .failure(error):
                    presentableError = .init(title: error.description)
                    isPresentingError = true
                }
            }
        }

        private func subscribeToUpdates() {
            $note.sink { newValue in
                self.isActionDisabled = newValue.isEmpty
            }
            .store(in: cancelBag)
        }
    }
}

#if DEBUG
    struct LogNoteModal_Previews: PreviewProvider {
        static var previews: some View {
            LogNoteModal(
                viewModel: .init(
                    isPresented: .constant(true)
                )
            )
            .preferredColorScheme(.light)
        }
    }
#endif
